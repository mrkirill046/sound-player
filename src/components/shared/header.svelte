<script lang="ts">
    import {theme, toggleTheme} from "@/stores/theme-store"
    import {ArrowLeft, Moon, Sun} from "@lucide/svelte"
    import {AppBar} from "@skeletonlabs/skeleton-svelte"
</script>

<AppBar>
    {#snippet lead()}
        <button>
            <ArrowLeft size={24} />
        </button>
    {/snippet}

    {#snippet trail()}
        <button on:click={toggleTheme}>
            {#if $theme === "dark"}
                <Moon size={20} />
            {:else}
                <Sun size={20} />
            {/if}
        </button>
    {/snippet}

    {#snippet headline()}
        <h2 class="text-4xl">Play Audio Files</h2>
    {/snippet}

    <h1>Kazuha's Sound Player</h1>
</AppBar>
