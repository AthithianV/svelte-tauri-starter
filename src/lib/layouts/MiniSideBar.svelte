<script lang="ts">
    import { SIDEBAR_MODES } from "$lib/shared/constants/sidebar.svelte";
    import { cn } from "$lib/shared/utils";
    import { activeMode, setActiveMode } from "$lib/shared/stores/sidebar.svelte";
    import { Sun, Moon } from "@lucide/svelte";
    import { theme } from "$lib/shared/stores/theme.svelte";
    import { Button } from "$lib/shared/components/ui/button";

    function toggleTheme() {
        theme.update((t) => (t === "dark" ? "light" : "dark"));
    }
</script>

<div class="flex flex-col items-center justify-between gap-3 h-full px-2 py-1 bg-sidebar">
    <div class="flex flex-col items-center gap-3">
        {#each SIDEBAR_MODES as { id, icon: Icon, label }}
            <Button
                class={cn(
                    "rounded-md",
                    $activeMode === id
                        ? "text-primary hover:text-primary bg-accent"
                        : "text-muted-foreground dark:hover:bg-transparent dark:focus:bg-transparent",
                    "cursor-pointer transition-all rounded-sm h-8 w-8 flex-center active:scale-90",
                )}
                title={label}
                onclick={() => setActiveMode(id)}
                variant="ghost"
            >
                <svelte:component
                    this={Icon}
                    style="width:1.1rem;height:1.1rem;"
                    stroke-width={2.5}
                />
            </Button>
        {/each}
    </div>
    <div>
        <Button
            class={cn(
                "rounded-md",
                "text-muted-foreground dark:hover:bg-transparent hover:bg-transparent dark:focus:bg-transparent bg-transparent",
                "cursor-pointer transition-all rounded-sm h-8 w-8 flex-center active:scale-90",
            )}
            title="Toggle Theme"
            onclick={toggleTheme}
        >
            {#if $theme === "dark"}
                <Sun style="width:1.1rem;height:1.1rem;" stroke-width={2.5} />
            {:else}
                <Moon style="width:1.1rem;height:1.1rem;" stroke-width={2.5} />
            {/if}
        </Button>
    </div>
</div>
