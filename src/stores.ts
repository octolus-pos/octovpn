import { writable, type Writable } from 'svelte/store';
import { Config, OpenVPNStatus, Status } from './structs';
import { Account, Location } from './api/structs';

export const page = writable('locations');
export const location = writable<Location | null>(null);
export const search = writable<string>('');

export const config = writable<Config>();

export const status: Writable<Status> = writable(Status.DISCONNECTED);
export const ovpn_status: Writable<OpenVPNStatus | null> = writable(null);
export const loggingIn = writable(false);
export const loggedIn = writable(false);

export const authToken = writable<string | null>(null);
export const locations = writable<Location[]>([]);
export const account = writable<Account | null>(null);

export const preflight = writable(false);