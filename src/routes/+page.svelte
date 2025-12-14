<script lang="ts">
    import * as Resizable from "$lib/shared/components/ui/resizable";

    import MiniSideBar from "$lib/layouts/MiniSideBar.svelte";
    import TitleBar from "$lib/layouts/TitleBar.svelte";
    import Card from "$lib/shared/components/ui/card/card.svelte";
</script>

<main class="w-full fixed top-0">
    <TitleBar />
    <div class="flex h-(--app-height)">
        <MiniSideBar />
        <div class="w-full h-full p-1 pt-0 bg-sidebar">
            <Card class="h-full rounded-sm">
                <Resizable.PaneGroup direction="horizontal" class="gap-[0.2px]">
                    <Resizable.Pane defaultSize={20} maxSize={40} minSize={10}>
                        <div></div>
                    </Resizable.Pane>
                    <Resizable.Handle class="bg-transparent" />
                    <Resizable.Pane>
                        <div></div>
                    </Resizable.Pane>
                </Resizable.PaneGroup>
            </Card>
        </div>
    </div>
</main>
