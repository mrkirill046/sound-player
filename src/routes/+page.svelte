<script lang="ts">
    import Header from "@/components/shared/header.svelte"
    import Player from "@/components/shared/player.svelte"
    import {onMount} from "svelte"
    import {listen} from "@tauri-apps/api/event"
    import {frontendReady, playAudio} from "@/lib/player"
    import {error, info} from "@tauri-apps/plugin-log"
    import {isFileSupported, toaster} from "@/lib/utils"
    import {currentPath, hasAudio} from "@/stores/audio-store"
    import {t} from "svelte-i18n"

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
                toaster.error({title: $t("Toast.FileNotSupported"), closable: false})
            }
        } catch (e) {
            error(`open-file event failed: ${e}`)
        }
    })

    onMount(async () => {
        await frontendReady()
    })
</script>

<Header title={$t("Header.Play")}/>

<main class="flex flex-col">
    <section class="flex flex-col h-full w-full justify-center items-center flex-1">
        <Player/>
    </section>
</main>
