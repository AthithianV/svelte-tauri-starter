export interface ReadConnection {
    id: number;
    name: string;
    host: string;
    port: number;
    username: string | null;
    password: string | null;
    db: number;
    is_default: boolean;
    created_at: string;
}

export interface CreateConnection {
    name: string;
    host: string;
    port: number;
    username?: string;
    password?: string;
    db?: number;
    is_default?: boolean;
}

export interface UpdateConnection {
    name?: string;
    host?: string;
    port?: number;
    username?: string;
    password?: string;
    db?: number;
    is_default?: boolean;
}
