// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use tauri::{Manager, Window};
use window_shadows::set_shadow;
use tauri_plugin_log::{Builder, LogTarget};
use discord_presence::Client;
use lazy_static::lazy_static;
use windows::{is_service_started, patch_config, write_config, read_config};
use std::{sync::Mutex, fmt::{Display, Formatter}, io::{Write, Read}, thread, net::TcpStream, time::Duration};

use crate::windows::{preflight, start_service, stop_service};

mod windows;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Credentials {
    username: String,
    password: String
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Location {
    city: String,
    config: Option<String>,
    country: String,
    #[serde(rename = "hasWireGuardConfig")]
    wireguard: bool,
    id: i32,
    ip: String,
    name: String,
    status: bool
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Protocol {
    OpenVPN,
    WireGuard
}

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::OpenVPN => write!(f, "OpenVPN"),
            Protocol::WireGuard => write!(f, "WireGuard")
        }
    }
}

impl Protocol {
    fn executable(&self) -> &'static str {
        match self {
            Protocol::OpenVPN => "openvpnserv2.exe",
            Protocol::WireGuard => "wireguard.exe"
        }
    }

    fn service_name(&self) -> &'static str {
        match self {
            Protocol::OpenVPN => "OctoVPNService$OpenVPN",
            Protocol::WireGuard => "OctoVPNService$WireGuard"
        }
    }
}

lazy_static! {
    static ref DISCORD_RPC_CLIENT: Mutex<Client> = Mutex::new(Client::new(743953368518492190));
    static ref CONFIGURATION: Mutex<Configuration> = Mutex::new(Configuration::default());
    
    static ref CONNECTED: Mutex<bool> = Mutex::new(false);
    static ref PROTOCOL_CONNECTED: Mutex<Option<Protocol>> = Mutex::new(None);
    static ref TAURI_WINDOW: Mutex<Option<Window>> = Mutex::new(None);
    static ref STATUS: Mutex<Status> = Mutex::new(Status::Disconnected);

    static ref OPENVPN_TIMEOUT_WAITING: Mutex<bool> = Mutex::new(false);
}

#[tauri::command]
fn toggle_discord_rpc(enable: bool) {
    (*CONFIGURATION.lock().unwrap()).discord_rpc = enable;
    log::info!("Discord RPC enabled: {}", enable);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum Status {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting
}

#[tauri::command]
fn toggle_connection(state: bool, config: Option<String>, credentials: Option<Credentials>, window: Window) -> bool {
    let protocol = CONFIGURATION.lock().unwrap().protocol;
    let protocol_connected = *PROTOCOL_CONNECTED.lock().unwrap();

    let status = match state {
        true => Status::Connecting,
        false => Status::Disconnecting
    };
    let _ = window.emit("status", status);
    *STATUS.lock().unwrap() = status;

    let result;
    unsafe {
        result = if state {
            config.and_then(|cfg| credentials.map(|creds| {
                let patch = patch_config(&protocol, cfg, creds);
                let service = if !is_service_started(&protocol) { start_service(&protocol) } else { true };

                log::debug!("Patch: {:?}, Service: {:?}", patch, service);

                patch && service
            })).unwrap_or(false)
        } else {
            if let Some(protocol) = protocol_connected {
                if is_service_started(&protocol) {
                    stop_service(&protocol)
                } else {
                    true
                }
            } else {
                false
            }
        };
    }

    if result {
        *PROTOCOL_CONNECTED.lock().unwrap() = if state { Some(protocol) } else { None };
        
        log::info!("Successfully {}" , if state { "connected" } else { "disconnected" })
    } else {
        log::info!("Failed to {}" , if state { "connect" } else { "disconnect" })
    }

    if CONFIGURATION.lock().unwrap().protocol == Protocol::WireGuard {
        let status = if state {
            if result { Status::Connected } else { Status::Disconnected }
        } else {
            if result { Status::Disconnected } else { Status::Connected }
        };

        let _ = window.emit("status", status);
    } else {
        // Initialize 15 seconds
        openvpn_timeout(window, state, protocol)
    }

    return result
}

#[tauri::command]
fn is_connected(window: Window) -> Option<Protocol> {
    let _ = window.emit("status", match *PROTOCOL_CONNECTED.lock().unwrap() {
        Some(_) => Status::Connected,
        None => Status::Disconnected
    });
    *PROTOCOL_CONNECTED.lock().unwrap()
}

#[tauri::command]
fn preflight_check() {
    // Set logging var
    std::env::set_var("RUST_LOG", "info");
    
    unsafe {
        let protocol = CONFIGURATION.lock().unwrap().protocol;
        let preflight = preflight(&protocol);
        log::info!("Preflight check: {:?}", preflight);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Configuration {
    credentials: Option<Credentials>,
    theme: String,
    #[serde(rename = "discordRPC")]
    discord_rpc: bool,
    protocol: Protocol
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            credentials: None,
            theme: "dark".to_string(),
            discord_rpc: false,
            protocol: Protocol::OpenVPN
        }
    }
}

#[tauri::command]
fn save_config(config: Configuration) {
    write_config(&config);
    *CONFIGURATION.lock().unwrap() = config;
}

#[tauri::command]
fn load_config() -> Configuration {
    let config = read_config();
    *CONFIGURATION.lock().unwrap() = config.clone();
    return config
}

fn main() {
    // Start the Discord RPC thread
    let _discord = discord_thread();

    // Check if one of the protocols is already started
    unsafe {
        let openvpn = is_service_started(&Protocol::OpenVPN);
        let wireguard = is_service_started(&Protocol::WireGuard);

        if openvpn {
            *PROTOCOL_CONNECTED.lock().unwrap() = Some(Protocol::OpenVPN);
        } else if wireguard {
            *PROTOCOL_CONNECTED.lock().unwrap() = Some(Protocol::WireGuard);
        }
    }

    let home = home::home_dir().unwrap();
    let _ = std::fs::create_dir_all(format!("{}/{}/{}", home.to_str().unwrap(), ".octovpn", "logs"));

    let log_dir = format!("{}/{}/{}", home.to_str().unwrap(), ".octovpn", "logs");
    let log_files = std::fs::read_dir(log_dir.clone()).unwrap();
    let log_name = format!("octovpn.{}", log_files.count());

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let _ = set_shadow(&window, true);
            let _ = openvpn_thread(window);

            Ok(())
        })
        .plugin(
            Builder::new()
                .targets([
                    LogTarget::Stdout,
                    LogTarget::Webview,
                    LogTarget::Folder(log_dir.into())
                ])
                .log_name(log_name)
                .filter(|metadata| metadata.target().starts_with("octovpn")) 
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            toggle_discord_rpc,
            preflight_check,
            toggle_connection,
            is_connected,
            save_config,
            load_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenVPNState {
    start: u64,
    connected: bool,
    state: String,
    local_ip: String,
    remote_ip: String,
    port: u16,
}

fn openvpn_timeout(window: Window, expected_success: bool, protocol: Protocol) -> () {
    // Lock the timeout mutex, this is now in charge of the timeout
    *OPENVPN_TIMEOUT_WAITING.lock().unwrap() = true;

    // Check expected response 10 times
    let status = if expected_success { Status::Connected } else { Status::Disconnected };
    let opposite = if expected_success { Status::Disconnected } else { Status::Connected };

    let mut failed_tries = 0;
    let max_failed_tries = 5;

    let mut successful_tries = 0;
    let min_successful_tries = if expected_success { 4 } else { 1 }; // Checking for connection failed takes longer

    thread::spawn(move || {
        loop {
            // If we've tried 10 times, we're done and we failed
            if failed_tries >= max_failed_tries {
                log::error!("Failed to connect to OpenVPN management interface");
                let _ = window.emit("status", opposite);

                if expected_success { unsafe { stop_service(&protocol) } } else { unsafe { start_service(&protocol) } };
                break;
            }

            let client = TcpStream::connect("::1:7505");

            if (client.is_ok() && expected_success)
                || (client.is_err() && !expected_success)
            {
                if let Ok(c) = client {
                    c.shutdown(std::net::Shutdown::Both).ok();
                }

                successful_tries += 1;

                log::debug!("Status was expected, {} successful tries", successful_tries);

                if successful_tries >= min_successful_tries {
                    log::debug!("Status was expected, we're done");

                    // Expected disconnect, we got one so update immediately
                    if !expected_success {
                        let _ = window.emit::<Option<OpenVPNState>>("openvpn_status", None);
                    }

                    let _ = window.emit("status", status);
                    break;
                } else {
                    // Otherwise, wait a second and try again
                    thread::sleep(Duration::from_secs(1));
                }
            } else {
                // Otherwise, wait a second and try again
                failed_tries += 1;
                successful_tries = 0;
                
                log::debug!("Status was not expected, trying again ({}/{})", failed_tries, max_failed_tries);
                thread::sleep(Duration::from_secs(1));
            }
        }

        // Set the timeout to false
        *OPENVPN_TIMEOUT_WAITING.lock().unwrap() = false;
    });
}

fn openvpn_thread(window: Window) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let is_timeout_active = *OPENVPN_TIMEOUT_WAITING.lock().unwrap();
            let is_openvpn = CONFIGURATION.lock().unwrap().protocol == Protocol::OpenVPN;

            // Something else is in charge of the timeout, so we'll just wait
            if is_timeout_active || !is_openvpn {
                thread::sleep(Duration::from_millis(500));
                continue;
            }

            let client = TcpStream::connect("::1:7505");

            if client.is_err() {
                // let _ = window.emit("status", Status::Disconnected);
                let _ = window.emit::<Option<OpenVPNState>>("openvpn_status", None);

                thread::sleep(Duration::from_millis(500));
                continue;
            }

            let mut client = client.unwrap();
            loop {
                let write = client.write(b"state\n");
                if write.is_err() {
                    // log::error!("Failed to write to OpenVPN management interface");
                    break;
                }

                let mut buffer: [u8; 1024] = [0; 1024];
                if client.read(&mut buffer).is_err() {
                    // log::error!("Failed to read from OpenVPN management interface");
                    break;
                }

                // Replace final null bytes
                let buffer = String::from_utf8_lossy(&buffer);
                let buffer = buffer.replace("\u{0}", "");

                if buffer.trim().len() == 0 {
                    // log::error!("OpenVPN management interface returned empty response");
                    break;
                }

                // Ignore if status is not yet available (i.e. starts with "INFO: ")
                if buffer.starts_with(">INFO") {
                    // log::warn!("OpenVPN management interface returned INFO response");
                    continue;
                }

                // Parse into the struct
                let split = buffer.split(',').collect::<Vec<&str>>();
                if split.len() < 6 {
                    // log::error!("OpenVPN management interface returned invalid response: {}", buffer);
                    break;
                }

                let start = split[0].parse::<u64>();
                let connected = split[1].to_string() == "CONNECTED";
                let state = split[2].to_string();
                let local_ip = split[3].to_string();
                let remote_ip = split[4].to_string();
                let port = split[5].parse::<u16>();

                if start.is_err() || port.is_err() {
                    // log::error!("OpenVPN management interface returned invalid response: {}", buffer);
                    break;
                }

                let state = OpenVPNState {
                    start: start.unwrap(),
                    connected,
                    state,
                    local_ip,
                    remote_ip,
                    port: port.unwrap()
                };

                // log::info!("OpenVPN management interface says {:?}", state);
                let _ = window.emit("openvpn_status", Some(state));

                // If not in an inbetween state, update the status
                if *STATUS.lock().unwrap() != Status::Disconnecting && CONFIGURATION.lock().unwrap().protocol == Protocol::OpenVPN {
                    // let _ = window.emit("status", Status::Connected);
                }

                // Try to get status every half-second
                thread::sleep(Duration::from_millis(500));
            }

            // Don't update the status if we're waiting for a timeout
            if *OPENVPN_TIMEOUT_WAITING.lock().unwrap() || CONFIGURATION.lock().unwrap().protocol != Protocol::OpenVPN {
                continue;
            }
            
            let _ = window.emit::<Option<OpenVPNState>>("openvpn_status", None);

            if !vec![Status::Connecting, Status::Disconnecting].contains(&*STATUS.lock().unwrap())
                && CONFIGURATION.lock().unwrap().protocol == Protocol::OpenVPN
            {
                // let _ = window.emit("status", Status::Disconnected);
            }

            thread::sleep(Duration::from_millis(500));
        }
    })
}

fn discord_thread() -> thread::JoinHandle<()> {
    // Start a new background thread for Discord
    thread::spawn(move || {
        // Get the current timestamp
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Initialize the Discord RPC client
        let mut client = DISCORD_RPC_CLIENT.lock().unwrap();

        // Start the client
        let _ = client.start();

        loop {
            let enabled = CONFIGURATION.lock().unwrap().discord_rpc;
            let protocol_connected = *PROTOCOL_CONNECTED.lock().unwrap();

            if enabled {
                let res = client.set_activity(|activity| {
                    activity
                        .details("zeph's private VPN")
                        .state(if protocol_connected.is_some() { "Connected" } else { "Disconnected" })
                        .timestamps(|timestamps| {
                            if protocol_connected.is_some() { timestamps.start(timestamp) } else { timestamps }
                        })
                        .assets(|assets| {
                            let mut assets = assets
                                .large_image("icon");

                            if let Some(p) = protocol_connected {
                                assets = assets.large_text(format!("Koharu (via {})", p))
                            }

                            assets
                        })
                });

                if res.is_err() {
                    log::error!("Failed to set Discord RPC activity!");
                }
            } else {
                let _ = client.clear_activity();
            }

            // 15 seconds is the actual rate limit for Discord RPC.
            // We'll sleep for 16 seconds to be safe.
            thread::sleep(Duration::from_secs(16));
        }
    })
}