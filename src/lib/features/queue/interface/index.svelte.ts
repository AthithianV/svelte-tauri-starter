export interface ReadQueueModel {
    id: number;
    connection_id: number;
    queue_name: string;
    display_name: string | null;
    is_starred: boolean | null;
    auto_refresh_rate: number | null;
    notification_settings: string | null;
    created_at: Date | string;
}

export interface CreateQueueModel {
    connection_id: number;
    queue_name: string;
    display_name: string | null;
    is_starred: boolean | null;
    auto_refresh_rate: number | null;
    notification_settings: string | null;
}

export interface UpdateQueueModel {
    display_name?: string;
    is_starred?: boolean;
    auto_refresh_rate?: number;
    notification_settings?: string;
}
