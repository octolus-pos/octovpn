export class Location {
    id: string;
    ip: string;
    country: string;
    city: string | null;
    name: string;
    status: string;
    hasWireGuardConfig: string;
    config: string;
    country_code: string;

    constructor(
        id: string,
        ip: string,
        country: string,
        city: string,
        name: string,
        status: string,
        hasWireGuardConfig: string,
        config: string,
        country_code: string,
    ) {
        this.id = id;
        this.ip = ip;
        this.country = country;
        this.city = city;
        this.name = name;
        this.status = status;
        this.hasWireGuardConfig = hasWireGuardConfig;
        this.config = config;
        this.country_code = country_code;
    }

    static fromJson(json: any): Location {
        return new Location(
            json.id,
            json.ip,
            json.country,
            json.city,
            json.name,
            json.status,
            json.hasWireGuardConfig,
            json.config,
            json.country_code,
        );
    }
}

export class Account {
    id: string;
    email: string;
    username: string;
    firstName: string;
    lastName: string;
    userType: string;

    constructor(
        id: string,
        email: string,
        username: string,
        firstName: string,
        lastName: string,
        userType: string,
    ) {
        this.id = id;
        this.email = email;
        this.username = username;
        this.firstName = firstName;
        this.lastName = lastName;
        this.userType = userType;
    }

    static fromJson(json: any): Account {
        return new Account(
            json.id,
            json.email,
            json.username,
            json.firstName,
            json.lastName,
            json.userType,
        );
    }
}

export class LoginResponse {
    token: string;

    constructor(token: string) {
        this.token = token;
    }
}

export class LogoutResponse {
    message: string;

    constructor(message: string) {
        this.message = message;
    }
}

export class ConfigResponse {
    config: string;

    constructor(config: string) {
        this.config = config;
    }
}

/// Wrapper for every API response.
export class APIResponse<T> {
    success: boolean;
    data: T | undefined;
    error: Error | undefined;

    constructor(success: boolean, data: T | undefined, error: Error | undefined) {
        this.success = success;
        this.data = data;
        this.error = error;
    }
}

export class Error {
    code: number;
    message: string;

    constructor(code: number, message: string) {
        this.code = code;
        this.message = message;
    }
}
