<script lang="ts">
    import {goto} from "$app/navigation"
    import {page} from "$app/state"
    import {initPalette} from "@/stores/palette-store"
    import {theme, toggleTheme} from "@/stores/theme-store"
    import {ArrowLeft, Moon, Sun, Settings, House} from "@lucide/svelte"
    import {AppBar} from "@skeletonlabs/skeleton-svelte"

    export let title: string

    initPalette()
</script>

<AppBar>
    {#snippet lead()}
        {#if page.url.pathname === "/"}
            <button>
                <House size={24} />
            </button>
        {:else}
            <button on:click={() => goto("/")}>
                <ArrowLeft size={24} />
            </button>
        {/if}
    {/snippet}

    {#snippet trail()}
        <button on:click={() => goto("/settings")}>
            <Settings size={20} />
        </button>

        <button on:click={toggleTheme}>
            {#if $theme === "dark"}
                <Moon size={20} />
            {:else}
                <Sun size={20} />
            {/if}
        </button>
    {/snippet}

    {#snippet headline()}
        <h2 class="text-4xl font-oswald">{title}</h2>
    {/snippet}

    <h1 class="font-lobster">Kazuha's Sound Player</h1>
</AppBar>
