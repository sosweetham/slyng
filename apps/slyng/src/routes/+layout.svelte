<script>
    import { page } from "$app/state";
    let { children } = $props();
</script>

<!-- display a back button if different from root -->
<div>
    {#if page.route.id !== "/"}
        <a href="..">Back</a>
    {/if}
</div>
{@render children?.()}
