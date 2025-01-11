<script>
    import {page} from '$app/state';
	let { children } = $props();

    // get page path
    let pagePath = page.route
</script>

{JSON.stringify(pagePath)}

{@render children?.()}