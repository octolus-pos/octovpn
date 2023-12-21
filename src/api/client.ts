import { Body, getClient } from '@tauri-apps/api/http';
import {
    APIResponse,
    Account,
    ConfigResponse,
    Location,
    type LoginResponse,
    type LogoutResponse,
} from './structs';
import type { Credentials } from '../structs';

const BASE_URL = 'https://617069.6f63746f76706e.com:8443/api/v1';

/// Logs in a user with the specified username and password.
/// @param email The email address of the user.
/// @param password The password of the user.
/// @return The session token.
export async function login(credentials: Credentials): Promise<APIResponse<LoginResponse>> {
    let client = await getClient();
    const response = await client.post<APIResponse<LoginResponse>>(
        `${BASE_URL}/account/login`,
        Body.json({ username: credentials.username, password: credentials.password }),
    );
    return response.data;
}

/// Logs out a user with the specified access token.
/// @param email The email address of the user.
/// @param password The password of the user.
/// @return The message returned by the server.
export async function logout(token: string): Promise<APIResponse<LogoutResponse>> {
    let client = await getClient();
    const response = await client.post<APIResponse<LogoutResponse>>(
        `${BASE_URL}/account/logout`,
        Body.json({ token }),
    );
    return response.data;
}

/// Gets account information with the specified access token.
/// @param token The access token of the user.
/// @return The message returned by the server.
export async function account(token: string): Promise<APIResponse<Account>> {
    let client = await getClient();
    const response = await client.get<APIResponse<Account>>(`${BASE_URL}/account`, {
        headers: {
            Authorization: `Bearer ${token}`,
        },
    });
    return response.data;
}

/// Returns a list of all available locations
/// @param token The access token of the user.
/// @return The list of locations.
export async function locations(token: string): Promise<APIResponse<Location[]>> {
    let client = await getClient();
    const response = await client.get<APIResponse<Location[]>>(`${BASE_URL}/locations`, {
        headers: {
            Authorization: `${token}`,
        },
    });
    return response.data;
}

/// Returns the OpenVPN configuration for the specified location and protocol
/// @param token The access token of the user.
/// @param location The location to get the configuration for.
/// @param protocol The protocol to use.
/// @return The OpenVPN configuration.
export async function openvpn(
    token: string,
    location: Location,
    protocol: string,
): Promise<APIResponse<ConfigResponse>> {
    let client = await getClient();
    const response = await client.get<APIResponse<ConfigResponse>>(
        `${BASE_URL}/configs/openvpn/${location.id}/${protocol}`,
        {
            headers: {
                Authorization: `${token}`,
            },
        },
    );
    return response.data;
}

/// Returns the WireGuard configuration for the specified location
/// @param token The access token of the user.
/// @param location The location to get the configuration for.
/// @return The WireGuard configuration.
export async function wireguard(
    token: string,
    location: Location,
): Promise<APIResponse<ConfigResponse>> {
    let client = await getClient();

    const response = await client.get<APIResponse<ConfigResponse>>(
        `${BASE_URL}/configs/wireguard/${location.id}`,
        {
            headers: {
                Authorization: `${token}`,
            },
        },
    );
    
    return response.data;
}
