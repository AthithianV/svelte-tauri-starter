import { Box, Clock, Database, FolderCode, Settings } from "@lucide/svelte";

export const SIDEBAR_MODES = [
    { id: "databases", icon: Database, label: "Databases" },
    { id: "queries", icon: FolderCode, label: "Queries" },
    { id: "history", icon: Clock, label: "History" },
    { id: "models", icon: Box, label: "Models" },
    { id: "settings", icon: Settings, label: "Settings" },
];
