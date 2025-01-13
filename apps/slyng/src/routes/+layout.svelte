<script lang="ts">
    import { afterNavigate } from "$app/navigation";
    import { page } from "$app/state";
    import { supportedThemes } from "$lib/global";
    import { currentPath } from "$lib/svelte/state.svelte";
    import { Settings } from "$lib/svelte/store.svelte";
    import { onMount } from "svelte";
    import { appDataDir } from "@tauri-apps/api/path";
    import { swipe, type SwipeCustomEvent } from "svelte-gestures";
    import {DeviceInfo} from "$lib/svelte/device-info.svelte";

    import "../app.css";

    let { children } = $props();

    let settings: Settings;

    let currentTheme: string | undefined = $state();

    const changeTheme = (theme: string) => {
        document.documentElement.setAttribute("data-theme", theme);
    };

    onMount(async () => {
        settings = await Settings.create();
        currentTheme = settings.theme;
        console.log(currentTheme);
        changeTheme(currentTheme);
        const appDataDirPath = await appDataDir();
        console.log(appDataDirPath);
    });

    afterNavigate(() => {
        currentPath.value = page.url.pathname;
    });
    let splitPath = $derived(currentPath.value.split("/"));

    let isSidebarVisible = $state(false);

    // Handle swipe gesture
    const handleSwipe = (event: SwipeCustomEvent) => {
        if (event.detail.direction === "right") {
            isSidebarVisible = true;
        } else if (event.detail.direction === "left") {
            isSidebarVisible = false;
        }
    };
</script>

<!-- <div class="px-4 py-2">
    <div class="navbar bg-base-300 rounded-box">
        <div class="flex px-2 lg:flex-none">
            <a href="/" class="text-lg font-bold">
                Kachoww
            </a>
        </div>
        {#if currentTheme}
            <div class="flex flex-1 justify-end px-2">
                <div class="flex items-stretch">
                    <button class="btn btn-ghost rounded-btn">Button</button>
                    <div class="dropdown dropdown-end">
                        <div
                            tabindex="0"
                            role="button"
                            class="btn btn-ghost rounded-btn"
                        >
                            Dropdown
                        </div>
                        <ul
                            tabindex="-1"
                            class="menu gap-2 dropdown-content bg-base-200 rounded-box z-[1] mt-4 max-w-52 p-2 shadow h-72 w-48 overflow-y-auto flex-nowrap"
                        >
                            {#each supportedThemes as theme}
                                <li class={`btn rounded-btn capitalize`} class:btn-primary={currentTheme === theme.name}
                                onclick={() => {
                                    settings.theme = theme.name;
                                    currentTheme = theme.name;
                                    changeTheme(theme.name);
                                }}>
                                        {theme.emoji} {theme.name}
                                </li>
                            {/each}
                        </ul>
                    </div>
                </div>
            </div>
        {/if}
    </div>
</div> -->

<div
    class="h-screen flex"
    use:swipe={() => ({
        timeframe: 300,
        minSwipeDistance: 60,
    })}
    onswipe={handleSwipe}
>
    <!-- Sidebar -->
    <div
        class="
            sidebar bg-base-300 w-64 h-full fixed md:static z-50 md:transform-none
            md:block
        "
        class:hidden-sm={!isSidebarVisible && !DeviceInfo.canHover}
        class:visible-sm={isSidebarVisible && !DeviceInfo.canHover}
    >
        <div class="p-4">
            <h2 class="text-lg font-bold">Sidebar</h2>
            <ul class="menu">
                <li><a href="#home">Home</a></li>
                <li><a href="#about">About</a></li>
                <li><a href="#contact">Contact</a></li>
            </ul>
        </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1">
        <div class="breadcrumbs text-sm px-4 bg-base-200">
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
        <div class="px-4">
            {@render children?.()}
        </div>
    </div>
</div>

<style>
    /* Optional: Smooth transition for sidebar visibility */
    .sidebar {
        transition: all 0.3s ease;
    }
    .hidden-sm {
        transform: translateX(-100%);
        opacity: 0;
        width: 0px;
    }
    .visible-sm {
        transform: translateX(0);
        display: block;
        opacity: 1;
        width: 256px;
    }
</style>
