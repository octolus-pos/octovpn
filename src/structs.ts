export class Region {
    name: string;
    countries: Country[];

    constructor(name: string, countries: Country[]) {
        this.name = name;
        this.countries = countries;
    }
}

export class Country {
    name: string;
    code: string;
    cities: City[];

    constructor(name: string, code: string, cities: City[]) {
        this.name = name;
        this.code = code;
        this.cities = cities;
    }
}

export class City {
    name: string;

    constructor(name: string) {
        this.name = name;
    }
}

export class Placeholder {
    country: Country;
    city: City;

    constructor(country: Country, city: City) {
        this.country = country;
        this.city = city;
    }
}

export enum Protocol {
    OpenVPN,
    WireGuard,
}

export class Credentials {
    username: string;
    password: string;

    constructor(username: string, password: string) {
        this.username = username;
        this.password = password;
    }
}

export class Config {
    credentials: Credentials | null;
    theme: string;
    discordRPC: boolean;
    protocol: Protocol;

    constructor(credentials: Credentials, theme: string, discordRPC: boolean, protocol: Protocol) {
        this.credentials = credentials;
        this.theme = theme;
        this.discordRPC = discordRPC;
        this.protocol = protocol;
    }
}

export enum Status {
    DISCONNECTED,
    CONNECTING,
    CONNECTED,
    DISCONNECTING
}

export function statusToString(status: Status): string {
    switch (status) {
        case Status.DISCONNECTED:
            return 'Disconnected';
        case Status.CONNECTING:
            return 'Connecting';
        case Status.CONNECTED:
            return 'Connected';
        case Status.DISCONNECTING:
            return 'Disconnecting';
    }
}

export class OpenVPNStatus {
    start: number;
    connected: boolean;
    state: string;
    local_ip: string;
    remote_ip: string;
    port: number;

    constructor(
        start: number,
        connected: boolean,
        state: string,
        local_ip: string,
        remote_ip: string,
        port: number,
    ) {
        this.start = start;
        this.connected = connected;
        this.state = state;
        this.local_ip = local_ip;
        this.remote_ip = remote_ip;
        this.port = port;
    }

    static fromJSON(json: any): OpenVPNStatus {
        return new OpenVPNStatus(
            json.start,
            json.connected,
            json.state,
            json.local_ip,
            json.remote_ip,
            json.port,
        );
    }
}
