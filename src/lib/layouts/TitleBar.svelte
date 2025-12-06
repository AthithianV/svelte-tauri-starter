<script lang="ts">
    import { onMount } from "svelte";
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import Close from "$lib/components/icons/Close.svelte";
    import Minimize from "$lib/components/icons/Minimize.svelte";
    import RestoreDown from "$lib/components/icons/RestoreDown.svelte";
    import RestoreUp from "$lib/components/icons/RestoreUp.svelte";
    import { Button } from "$lib/components/ui/button/index.js";

    let isMaximized = $state(false);
    $effect(() => {
        const appWindow = getCurrentWindow();

        let cleanup: () => void;
        (async () => {
            isMaximized = await appWindow.isMaximized();

            const unlisten = await appWindow.onResized(async () => {
                isMaximized = await appWindow.isMaximized();
            });

            cleanup = unlisten;
        })();

        return () => {
            if (cleanup) cleanup();
        };
    });

    const handleMinimize = async () => {
        const appWindow = getCurrentWindow();
        await appWindow.minimize();
    };

    const handleMaximize = async () => {
        const appWindow = getCurrentWindow();
        await appWindow.toggleMaximize();
    };

    const handleClose = async () => {
        const appWindow = getCurrentWindow();
        await appWindow.close();
    };

    const handleStartDragging = async (e: MouseEvent) => {
        if ((e.target as HTMLElement).closest("button")) return;
        const appWindow = getCurrentWindow();
        await appWindow.startDragging();
    };
</script>

<div class="w-full h-(--titlebar-height) flex justify-between items-center bg-sidebar border-b">
    <div
        role="button"
        tabindex="0"
        onmousedown={handleStartDragging}
        class="flex-1 px-3 flex gap-2 text-sm items-center font-semibold"
    ></div>
    <div class="flex items-center h-full">
        <Button
            variant="ghost"
            size="icon"
            class="px-6 h-full rounded-none text-foreground"
            onclick={handleMinimize}
        >
            <Minimize />
        </Button>
        <Button
            variant="ghost"
            size="icon"
            class="px-6 text-foreground h-full rounded-none"
            onclick={handleMaximize}
        >
            {#if isMaximized}
                <RestoreDown />
            {:else}
                <RestoreUp />
            {/if}
        </Button>
        <Button
            variant="ghost"
            size="icon"
            class="px-6 text-foreground h-full rounded-none hover:bg-red-500 dark:hover:bg-red-800"
            onclick={handleClose}
        >
            <Close />
        </Button>
    </div>
</div>
