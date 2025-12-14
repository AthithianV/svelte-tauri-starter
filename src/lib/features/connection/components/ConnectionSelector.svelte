<script lang="ts">
    import {
        Select,
        SelectContent,
        SelectItem,
        SelectTrigger,
    } from "$lib/shared/components/ui/select";
    import { Skeleton } from "$lib/shared/components/ui/skeleton";
    import { Alert, AlertDescription, AlertTitle } from "$lib/shared/components/ui/alert";
    import { CircleAlert } from "lucide-svelte";
    import { useGetConnectionById, useGetConnections } from "../hooks/index.svelte";

    // Fetch Connection Where default is set.
    let selection = $state<string>("");
    const selectedConnection = useGetConnectionById(1);
    const connectionsQuery = useGetConnections();

    $effect(() => {
        if (selectedConnection.data) {
            selection = selectedConnection.data.id.toString();
        }
    });
</script>

<div class="w-full max-w-sm">
    {#if connectionsQuery.isLoading || selectedConnection.isLoading}
        <Skeleton class="h-10 w-full" />
    {:else if connectionsQuery.isError}
        <Alert variant="destructive">
            <CircleAlert class="h-4 w-4" />
            <AlertTitle>Error</AlertTitle>
            <AlertDescription>
                Failed to load connections: {connectionsQuery.error.message}
            </AlertDescription>
        </Alert>
    {:else if connectionsQuery.data && connectionsQuery.data.length > 0 && selectedConnection.data}
        <Select type="single" bind:value={selection}>
            <SelectTrigger class="w-full">
                <span>{selectedConnection.data?.name ?? "Select a Connection"}</span>
            </SelectTrigger>
            <SelectContent>
                {#each connectionsQuery.data as connection}
                    <SelectItem value={`${connection.id}`}>
                        {connection.name} ({connection.host}:{connection.port})
                    </SelectItem>
                {/each}
            </SelectContent>
        </Select>
    {:else}
        <Alert>
            <AlertTitle>No Connections Found</AlertTitle>
            <AlertDescription>You haven't created any database connections yet.</AlertDescription>
        </Alert>
    {/if}
</div>
