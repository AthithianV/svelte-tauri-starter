import { writable, type Writable } from "svelte/store";

export interface SidebarStore {
    activeMode: string;
}

export const activeMode: Writable<string> = writable("databases");
export const setActiveMode = (mode: string) => activeMode.set(mode);
