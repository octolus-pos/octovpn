import './styles.css';
import App from './App.svelte';
import { invoke } from '@tauri-apps/api';
import { Config, Credentials, Status, Protocol, OpenVPNStatus } from './structs';
import {
    authToken,
    locations as storeLocations,
    config,
    account as storeAccount,
    status,
    ovpn_status,
    loggingIn,
    loggedIn,
} from './stores';
import { account, locations, login } from './api/client';
import { listen } from '@tauri-apps/api/event';
import { get } from 'svelte/store';
import { LogicalSize, appWindow } from '@tauri-apps/api/window';

loggedIn.subscribe((loggedIn) => {
    if (!loggedIn) {
        appWindow.setSize(new LogicalSize(400, 500));
    } else {
        appWindow.setSize(new LogicalSize(800, 650));
    }
});

// TODO: handle token being null
/// Updates the account and locations stores when the token changes.
authToken.subscribe(async (token) => {
    if (!token) { return; }

    let locs = await locations(token!);
    if (locs.success) {
        storeLocations.set(locs.data!);
    }

    let acc = await account(token!);
    if (acc.success) {
        storeAccount.set(acc.data!);
    }

    loggedIn.set(acc.success && locs.success);
    loggingIn.set(false);
});

status.subscribe((status) => {
    console.log(status);
});

/// Loads the config on startup and tries to auto-login
invoke<Config>('load_config').then(async (conf) => {
    config.set(conf);

    if (conf.credentials) {
        loggingIn.set(true);

        let creds = new Credentials(conf.credentials.username, conf.credentials.password);

        // TODO: insert something to handle "logging in" phase
        let log = await login(creds);
        if (log.success) {
            authToken.set(log.data!.token);
        }
    }
});

/// Updates the saved config when it changes.
config.subscribe(async (config) => {
    if (config && config.protocol != null) {
        invoke('save_config', { config: config });
    }

    console.log(config);

    let token = get(authToken);

    if (token) {
        locations(token).then((locs) => {
            if (locs.success) {
                storeLocations.set(locs.data!);
            }
        });
    }

    // if (config.credentials) {
    //     await login(config.credentials).then((token) => {
    //         if (token.success) {
    //             authToken.set(token.data!.token);
    //         }
    //     });
    // }
});

/// Listens for status events
listen<Status>('status', (message) => {
    // If WireGuard, wait a little for it to launch to prevent spam
    if (get(config).protocol == Protocol.WireGuard) {
        if (message.payload == Status.CONNECTED || message.payload == Status.DISCONNECTED) {
            setTimeout(() => {
                status.set(message.payload);
            }, 1000);
            return;
        }
    }

    status.set(message.payload);
});

/// Listens for OpenVPN status events
listen<OpenVPNStatus>('openvpn_status', (message) => {
    ovpn_status.set(message.payload);
});

const app = new App({
    target: document.getElementById('app')!,
});

/// Checks if the system is set up properly.
invoke('preflight_check');

/// Checks if the user is connected to a VPN.
invoke<Protocol>('is_connected').then((res) => {
    if (res !== null) {
        status.set(Status.CONNECTED);
    }
});

export default app;
