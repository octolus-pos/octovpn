use std::path::PathBuf;

use lazy_static::lazy_static;
use windows::{Win32::{System::{Services::{OpenSCManagerW, SC_MANAGER_ALL_ACCESS, OpenServiceW, SERVICE_ALL_ACCESS, CreateServiceW, SERVICE_WIN32_OWN_PROCESS, SERVICE_DEMAND_START, SERVICE_ERROR_NORMAL, StartServiceW, ControlService, SERVICE_CONTROL_STOP, SERVICE_STOPPED, ChangeServiceConfig2W, SERVICE_CONFIG_SERVICE_SID_INFO, SERVICE_SID_TYPE_UNRESTRICTED, SERVICE_SID_INFO, QueryServiceStatus, SERVICE_STATUS, SERVICE_RUNNING, SERVICE_START_PENDING, SERVICE_STOP_PENDING, ChangeServiceConfigW, SERVICE_NO_CHANGE}, Registry::{RegCreateKeyW, HKEY_LOCAL_MACHINE, HKEY, REG_SZ, RegOpenKeyExW, KEY_WRITE, RRF_RT_REG_SZ, RegGetValueW, RegSetKeyValueW}}, Security::SC_HANDLE}, core::{w, PWSTR, HSTRING}};
use reqwest::blocking::Client;

use crate::{CONNECTED, Protocol, Credentials, Configuration};


lazy_static! {
    static ref HOME: PathBuf = home::home_dir().unwrap();
}

#[derive(Debug)]
pub enum PreflightError {
    ServiceNotInstalled,
    FilesMissing,
    InvalidHash,
}

/// Performs configuration checks and ensures program
/// has everything it needs to correctly start.
pub unsafe fn preflight(protocol: &Protocol) -> Result<SC_HANDLE, PreflightError> {
    are_files_present(protocol)?;
    if protocol == &Protocol::OpenVPN {
        // Wireguard does not use the registry
        is_registry_set()?;
    }
    let service = is_service_installed(protocol)?;

    Ok(service)
}

/// Checks if the OpenVPN service is installed.
/// Requires admin privileges.
unsafe fn is_service_installed(protocol: &Protocol) -> Result<SC_HANDLE, PreflightError> {
    let sc_manager = OpenSCManagerW(
        None, 
        None,
        SC_MANAGER_ALL_ACCESS
    ).map_err(|_| PreflightError::ServiceNotInstalled)?;

    let service_name = HSTRING::from(protocol.service_name());
    
    let service = match OpenServiceW(
        sc_manager,
        &service_name,
        SERVICE_ALL_ACCESS
    ) {
        Ok(service) => Ok(service),
        Err(_) => match install_service(protocol) {
            Ok(service) => Ok(service),
            Err(e) => Err(e)
        }
    }.map_err(|_| PreflightError::ServiceNotInstalled)?;

    // Set to startup MANUAL
    let home = HOME.to_str().unwrap();

    let mut protocol_dir = format!("{}/.octovpn/{}/{}", home, protocol.to_string().to_lowercase(), protocol.executable());
    if protocol == &Protocol::WireGuard {
        protocol_dir += format!(" /service {}/.octovpn/wireguard/wireguard.conf", home).as_str();
    }
    let protocol_dir = HSTRING::from(protocol_dir);

    let display_name = HSTRING::from(format!("OctoVPN Service ({})", protocol.to_string()));

    ChangeServiceConfigW(
        service,
        SERVICE_NO_CHANGE,
        SERVICE_DEMAND_START,
        SERVICE_ERROR_NORMAL,
        &protocol_dir,
        None,
        None,
        None,
        None,
        None,
        &display_name,
    ).map_err(|_| PreflightError::ServiceNotInstalled)?;

    Ok(service)
}

unsafe fn ensure_reg_key(key: &str, value: String) -> bool {
    let hkey = HKEY_LOCAL_MACHINE;

    let key = HSTRING::from(key);
    let value = HSTRING::from(value);

    let mut result: PWSTR = std::mem::zeroed();

    if RegGetValueW(
        hkey,
        w!("Software\\OctoVPN"),
        &key,
        RRF_RT_REG_SZ,
        None,
        Some(&mut result as *mut PWSTR as *mut u8 as *mut _),
        None
    ).is_err() || result.to_string().unwrap() != value.to_string() {
        RegSetKeyValueW(
            hkey,
            w!("Software\\OctoVPN"),
            &key,
            REG_SZ.0,
            Some(value.as_wide().as_ptr() as *const _),
            value.len() as u32 * 2
        ).is_ok()
    } else {
        return true;
    }
}

/// Checks if the registry is set correctly.
/// - `autostart_config_dir` should be set to the user's home dir + .octovpn/openvpn/
/// - `openvpn_exe` should be set to the user's home dir + .octovpn/openvpn/openvpn.exe
/// 
/// Requires admin privileges.
/// * This is only required for OpenVPN, Wireguard does not use the registry.
unsafe fn is_registry_set() -> Result<(), PreflightError> {
    // Open registry key for OctoVPN
    let mut hkey: HKEY = std::mem::zeroed();
    if RegOpenKeyExW(HKEY_LOCAL_MACHINE, w!("Software\\OctoVPN"), 0, KEY_WRITE, &mut hkey)
        .is_err() { 
            // Create registry key for OctoVPN
            let mut result: HKEY = std::mem::zeroed();
            RegCreateKeyW(HKEY_LOCAL_MACHINE, w!("Software\\OctoVPN"), &mut result)
                .map_err(|_| PreflightError::ServiceNotInstalled)?;
        };

    let home = HOME.to_str().unwrap();

    // Set registry keys
    ensure_reg_key("autostart_config_dir", format!("{}/.octovpn/openvpn", home));
    ensure_reg_key("exe_path", format!("{}/.octovpn/openvpn/openvpn.exe", home));
    ensure_reg_key("config_ext", String::from("ovpn"));

    ensure_reg_key("log_append", String::from("0"));
    ensure_reg_key("log_dir", format!("{}/.octovpn/openvpn/logs", home));

    ensure_reg_key("priority", String::from("NORMAL_PRIORITY_CLASS"));
    
    Ok(())
}

unsafe fn install_service(protocol: &Protocol) -> Result<SC_HANDLE, PreflightError> {
    let sc_manager = OpenSCManagerW(
        None, 
        None,
        SC_MANAGER_ALL_ACCESS
    ).map_err(|_| PreflightError::ServiceNotInstalled)?;

    let home = HOME.to_str().unwrap();

    let mut protocol_dir = format!("{}/.octovpn/{}/{}", home, protocol.to_string().to_lowercase(), protocol.executable());
    if protocol == &Protocol::WireGuard {
        protocol_dir += format!(" /service {}/.octovpn/wireguard/wireguard.conf", home).as_str();
    }
    let protocol_dir = HSTRING::from(protocol_dir);

    let service_name = HSTRING::from(protocol.service_name());
    let display_name = HSTRING::from(format!("OctoVPN Service ({})", protocol.to_string()));

    // Create service
    let service = CreateServiceW(
        sc_manager,
        &service_name,
        &display_name,
        SERVICE_ALL_ACCESS,
        SERVICE_WIN32_OWN_PROCESS,
        SERVICE_DEMAND_START,
        SERVICE_ERROR_NORMAL,
        &protocol_dir,
        None,
        None,
        None,
        None,
        None
    ).map_err(|_| PreflightError::ServiceNotInstalled)?;

    // WireGuard requires the unrestricted SID type
    if protocol == &Protocol::WireGuard {
        let info = SERVICE_SID_INFO {
            dwServiceSidType: SERVICE_SID_TYPE_UNRESTRICTED
        };

        ChangeServiceConfig2W(
            service,
            SERVICE_CONFIG_SERVICE_SID_INFO, // SERVICE_CONFIG_SERVICE_SID_INFO
            Some(&info as *const _ as *const u8 as *const _)
        ).map_err(|_| PreflightError::ServiceNotInstalled)?;
    }

    Ok(service)
} 

fn ensure_paths(protocol: &Protocol) -> Result<(), PreflightError> {
    let home_dir = home::home_dir().unwrap();
    let home_dir = home_dir.to_str().unwrap();
    let protocol_dir = format!("{}/.octovpn/{}", home_dir, protocol.to_string().to_lowercase());

    std::fs::create_dir_all(protocol_dir).map_err(|_| PreflightError::FilesMissing)?;

    // create log file
    // only required for OpenVPN, Wireguard makes its own `log.bin`
    if protocol == &Protocol::OpenVPN {
        let logs_dir = format!("{}/.octovpn/openvpn/logs", home_dir);
        std::fs::create_dir_all(logs_dir).map_err(|_| PreflightError::FilesMissing)?;
        let _ = std::fs::File::create(format!("{}/openvpn.log", home_dir)).unwrap();
    }

    Ok(())
}

unsafe fn are_files_present(protocol: &Protocol) -> Result<(), PreflightError> {
    ensure_paths(protocol)?;

    // download hashes file for the protocol
    let client = Client::new();
    let req = client.get(format!("https://vpn.zephs.tech/{}/hashes", protocol.to_string().to_lowercase())).send()
        .map_err(|_| PreflightError::InvalidHash)?;
    let response = req.text().unwrap().trim().to_string();

    // format them into a usable map
    let mut hashes: Vec<(&str, &str)> = vec![];
    for line in response.lines() {
        let mut split = line.split_whitespace();
        let hash = split.next();
        let file = split.next();

        // ensure both are present
        if hash.is_none() || file.is_none() {
            return Err(PreflightError::InvalidHash)
        }

        hashes.push((hash.unwrap(), file.unwrap()));
    };

    // get user's home dir + .octovpn/{protocol}/
    let home = HOME.to_str().unwrap();
    let protocol_dir = format!("{}/.octovpn/{}/", home, protocol.to_string().to_lowercase());

    // check every file
    for (hash, file) in hashes {
        // check if present
        let path = format!("{}{}", protocol_dir, file);
        if !std::path::Path::new(&path).exists() {
            log::warn!("File {} is missing, downloading...", file);

            if !download(&client, protocol, file, &protocol_dir) {
                log::error!("Failed to download file {}", file);
                return Err(PreflightError::FilesMissing);
            }
        }

        // check if hash is correct
        let file_hash = sha256::digest(std::fs::read(path).unwrap());
        if file_hash != *hash {
            log::warn!("File {} is invalid, downloading...", file);

            if !download(&client, protocol, file, &protocol_dir) {
                log::error!("Failed to download file {}", file);
                return Err(PreflightError::InvalidHash);
            }
        }
    }

    // the config files get downloaded upon connection

    Ok(())
}

fn download(client: &Client, protocol: &Protocol, name: &str, home: &String) -> bool {
    let req = client.get(format!("https://vpn.zephs.tech/{}/{}", protocol.to_string().to_lowercase(), name)).send();
    if req.is_err() {
        return false;
    }

    std::fs::create_dir_all(home).unwrap();
    let mut file = std::fs::File::create(format!("{}/{}", home, name)).unwrap();
    let mut res = req.unwrap();
    std::io::copy(&mut res, &mut file).unwrap();

    true
}

pub unsafe fn is_service_started(protocol: &Protocol) -> bool {
    let service = preflight(protocol);
    if service.is_err() {
        log::error!("Failed to check service: {:?}", service);
        return false;
    }

    let mut status: SERVICE_STATUS = std::mem::zeroed();
    let status = QueryServiceStatus(service.unwrap(), &mut status).is_ok() && status.dwCurrentState == SERVICE_STOPPED;

    !status
}

pub unsafe fn start_service(protocol: &Protocol) -> bool {
    // Ensure everything is set up correctly
    let service = preflight(protocol);
    if service.is_err() {
        log::error!("Failed preflight check: {:?}", service);
        return false;
    }

    let service = service.unwrap();
    if StartServiceW(service, None).is_err() {
        log::error!("Failed to start service");
        return false;
    }

    // Check if service is running
    let mut status: SERVICE_STATUS = SERVICE_STATUS::default();
    if QueryServiceStatus(service, &mut status).is_err() {
        log::error!("Failed to query service status");
        return false;
    }

    // Sometimes it takes a while for the service to start
    let status = status.dwCurrentState == SERVICE_RUNNING || status.dwCurrentState == SERVICE_START_PENDING;

    *CONNECTED.lock().unwrap() = status;
    status
}

pub unsafe fn stop_service(protocol: &Protocol) -> bool {
    // Ensure everything is set up correctly
    let service = preflight(protocol);
    if service.is_err() {
        log::error!("Failed preflight check: {:?}", service);
        return false;
    }

    let service = service.unwrap();
    let mut status = SERVICE_STATUS::default();
    if ControlService(service, SERVICE_CONTROL_STOP, &mut status).is_err() {
        return false;
    }

    // Sometimes it takes a while for the service to stop
    let status = status.dwCurrentState == SERVICE_STOPPED || status.dwCurrentState == SERVICE_STOP_PENDING;

    *CONNECTED.lock().unwrap() = status;
    status
}

/// Rewrites the config file with necessary additions.
/// This should've been done API-side, but it is what it is.
pub fn patch_config(protocol: &Protocol, mut config: String, credentials: Credentials) -> bool {
    let path = format!("{}/.octovpn/{}/{}", HOME.to_str().unwrap(), protocol.to_string().to_lowercase(), if protocol == &Protocol::OpenVPN { "config.ovpn" } else { "wireguard.conf" });

    if protocol == &Protocol::OpenVPN {
        // Patch config with credentials
        config.push_str(format!("\n\n<auth-user-pass>\n{}\n{}\n</auth-user-pass>", credentials.username, credentials.password).as_str());

        // Windows-specific fix: remove these parameters as they break the CLI, for some reason
        let parameters = vec!["route-method exe", "route-delay 2", "register-dns"];
        for param in parameters {
            config = config.replace(param, format!("# {}", param).as_str());
        }

        // Add management interface and reduce verbosity
        config.push_str("\n\nmanagement localhost 7505\nverb 2");
    }

    std::fs::write(path, config).is_ok()
}

pub fn write_config(config: &Configuration) {
    let home = HOME.to_str().unwrap();
    let path = format!("{}/.octovpn/config.json", home);

    std::fs::write(path, serde_json::to_string(config).unwrap()).unwrap();
}

pub fn read_config() -> Configuration {
    let home = HOME.to_str().unwrap();
    let path = format!("{}/.octovpn/config.json", home);

    if !std::path::Path::new(&path).exists() {
        return Configuration::default();
    }

    let config = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(config.as_str()).unwrap()
}