<script lang="ts">
    import * as Sidebar from "$lib/shared/components/ui/sidebar/index.js";
    import ConnectionSelector from "$lib/features/connection/components/ConnectionSelector.svelte";
    import { SIDEBAR_ITEMS } from "$lib/shared/constants/sidebar.svelte";
</script>

<Sidebar.Root>
    <Sidebar.Header class="p-5">
        <Sidebar.Menu>
            <Sidebar.MenuItem>
                <ConnectionSelector selectConnection={undefined} />
            </Sidebar.MenuItem>
        </Sidebar.Menu>
    </Sidebar.Header>

    <Sidebar.Content class="px-5">
        <Sidebar.Group class="p-0">
            <Sidebar.GroupLabel>Application</Sidebar.GroupLabel>
            <Sidebar.GroupContent>
                <Sidebar.Menu>
                    {#each SIDEBAR_ITEMS as item (item.title)}
                        <Sidebar.MenuItem>
                            <Sidebar.MenuButton>
                                {#snippet child({ props })}
                                    <a href={item.url} {...props}>
                                        <item.icon />
                                        <span>{item.title}</span>
                                    </a>
                                {/snippet}
                            </Sidebar.MenuButton>
                        </Sidebar.MenuItem>
                    {/each}
                </Sidebar.Menu>
            </Sidebar.GroupContent>
        </Sidebar.Group>
    </Sidebar.Content>
</Sidebar.Root>
