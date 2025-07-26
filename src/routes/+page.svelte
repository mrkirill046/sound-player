<script lang="ts">
    import Header from "@/components/shared/header.svelte"
    import Player from "@/components/shared/player.svelte"
    import {invoke} from "@tauri-apps/api/core"
    import {debug} from "@tauri-apps/plugin-log"
    import {onMount} from "svelte"

    onMount(async () => {
        debug("Invoke frontend_ready function")

        await invoke("frontend_ready")
    })
</script>

<Header title="Play Audio Files" />

<main class="flex flex-col">
    <section class="flex flex-col h-full w-full justify-center items-center flex-1">
        <Player />
    </section>
</main>
