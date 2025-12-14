import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query";
import { invoke } from "@tauri-apps/api/core";
import type { CreateConnection, ReadConnection, UpdateConnection } from "../interface/index.svelte";

// Define the unique key for the connections list
const CONNECTION_QUERY_KEY = ["connections"];

// --- QUERY HOOKS (READ) ---

/**
 * 🎣 Hook to fetch and cache ALL connections.
 * Calls the Rust command `get_all_connections`.
 */
export const useGetConnections = () => {
    return createQuery(() => ({
        queryKey: CONNECTION_QUERY_KEY,
        queryFn: () => invoke("get_all_connections") as Promise<ReadConnection[]>,
    }));
};

/**
 * 🎣 Hook to fetch a SINGLE connection by ID.
 * Calls the Rust command `get_connection`.
 * @param id The ID of the connection to fetch.
 */
export const useGetConnectionById = (id: number) => {
    return createQuery(() => ({
        queryKey: [...CONNECTION_QUERY_KEY, id],
        queryFn: () => invoke("get_connection", { id }) as Promise<ReadConnection | null>,
        enabled: !!id,
    }));
};

// --- MUTATION HOOKS (CREATE, UPDATE, DELETE) ---

/**
 * 🛠️ Hook to create a new connection.
 * Calls the Rust command `create_connection`.
 */
export const useCreateConnection = () => {
    const queryClient = useQueryClient();

    return createMutation(() => ({
        mutationFn: (data: CreateConnection) =>
            invoke("create_connection", { data }) as Promise<void>,
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: CONNECTION_QUERY_KEY });
        },
    }));
};

/**
 * 🛠️ Hook to update an existing connection.
 * Calls the Rust command `update_connection`.
 */
export const useUpdateConnection = () => {
    const queryClient = useQueryClient();

    return createMutation(() => ({
        mutationFn: ({ id, data }: { id: number; data: UpdateConnection }) =>
            invoke("update_connection", { id, data }) as Promise<ReadConnection>,

        onSuccess: (updatedConnection) => {
            queryClient.setQueryData<ReadConnection | null>(
                [...CONNECTION_QUERY_KEY, updatedConnection.id],
                updatedConnection,
            );

            queryClient.invalidateQueries({ queryKey: CONNECTION_QUERY_KEY });
        },
    }));
};

/**
 * 🛠️ Hook to delete a connection.
 * Calls the Rust command `delete_connection`.
 */
export const useDeleteConnection = () => {
    const queryClient = useQueryClient();

    return createMutation(() => ({
        mutationFn: (id: number) => invoke("delete_connection", { id }) as Promise<void>,

        onSuccess: (_result, deletedId) => {
            queryClient.setQueryData<ReadConnection[]>(CONNECTION_QUERY_KEY, (oldData) => {
                return oldData ? oldData.filter((conn) => conn.id !== deletedId) : [];
            });
            queryClient.removeQueries({ queryKey: [...CONNECTION_QUERY_KEY, deletedId] });
        },
    }));
};
