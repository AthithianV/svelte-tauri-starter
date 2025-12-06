<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar/index";
    import AppSideBar from "$lib/layouts/AppSideBar.svelte";
    import TitleBar from "$lib/layouts/TitleBar.svelte";
</script>

<main>
    <div class="flex flex-1 overflow-hidden">
        <Sidebar.Provider class="w-screen">
            <AppSideBar />
            <div class="w-full">
                <TitleBar />
                <main>My App here</main>
            </div>
        </Sidebar.Provider>
    </div>
</main>
