<script lang="ts">
    import { afterNavigate } from "$app/navigation";
    import { page } from "$app/state";
    import { currentPath } from "$lib/svelte/state.svelte";
    import "../app.css";
    let { children } = $props();

    afterNavigate(() => {
        currentPath.value = page.url.pathname;
    });
    let splitPath = $derived(currentPath.value.split("/"));
</script>

<div class="breadcrumbs text-sm">
    <ul>
        {#each splitPath as path, index}
            {#if index === 0}
                <li>
                    <a href="/">
                        Home
                    </a>
                </li>
            {:else}
                <li>
                    <a
                        class="capitalize"
                        href={splitPath.slice(0, index + 1).join("/")}
                    >{path.split("-").join(" ")}</a>
                </li>
            {/if}
        {/each}
    </ul>
</div>

{@render children?.()}
