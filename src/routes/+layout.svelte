<script lang="ts">
    import "$lib/styles/app.css";

    import { Toaster } from "$lib/shared/components/ui/sonner";

    import { browser } from "$app/environment";
    import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query";

    const queryClient = new QueryClient({
        defaultOptions: {
            queries: {
                enabled: browser,
            },
        },
    });
    let { children } = $props();
</script>

<QueryClientProvider client={queryClient}>
    <Toaster richColors />
    {@render children()}
</QueryClientProvider>
