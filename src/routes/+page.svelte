<script lang="ts">
    import Header from "@/components/shared/header.svelte"
    import Player from "@/components/shared/player.svelte"
    import {invoke} from "@tauri-apps/api/core"
    import {debug} from "@tauri-apps/plugin-log"
    import {onMount} from "svelte"
    import {listen} from "@tauri-apps/api/event"
    import {playAudio} from "@/lib/player"
    import {info, error} from "@tauri-apps/plugin-log"
    import {isFileSupported, toaster} from "@/lib/utils"
    import {currentPath, hasAudio} from "@/stores/audio-store"

    listen<string>("open-file", async (event) => {
        const path = event.payload

        info(`Invoked open-file event with path: ${path}`)

        try {
            if (await isFileSupported(path)) {
                currentPath.set(path)
                hasAudio.set(true)

                setTimeout(async () => {
                    await playAudio(path)
                }, 300)
            } else {
                toaster.error({title: "File is not supported", closable: false})
            }
        } catch (e) {
            error(`open-file event failed: ${e}`)
        }
    })

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
