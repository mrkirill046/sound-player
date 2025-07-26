<script lang="ts">
    import {currentPath, hasAudio} from "@/stores/audio-store"
    import Controls from "./controls.svelte"
    import DropZone from "./drop-zone.svelte"
    import {onDestroy, onMount} from "svelte"
    import {setupHotkeys} from "@/lib/hotkeys"
    import {listen} from "@tauri-apps/api/event"
    import {playAudio} from "@/lib/player"
    import {isFileSupported, toaster} from "@/lib/utils"
    import {Toaster} from "@skeletonlabs/skeleton-svelte"

    let delayed: boolean = false
    let timeout: number
    let cleanup: () => void

    $: hasAudio.subscribe((value) => {
        clearTimeout(timeout)

        if (value) {
            timeout = setTimeout(() => {
                delayed = true
            }, 300)
        } else {
            delayed = false
        }
    })

    onMount(() => {
        cleanup = setupHotkeys()

        const unlisten = listen<string>("open-file", async (event) => {
            const path = event.payload

            console.log(path)

            if (await isFileSupported(path)) {
                currentPath.set(path)
                hasAudio.set(true)

                setTimeout(async () => {
                    await playAudio(path)
                }, 300)
            } else {
                toaster.error({title: "File is not supported", closable: false})
            }
        })

        return () => {
            unlisten.then((f) => f())
        }
    })

    onDestroy(() => {
        cleanup?.()
    })
</script>

<Toaster {toaster} />

<article class="space-y-8 w-full my-2">
    {#if !$hasAudio}
        <DropZone />
    {:else if delayed}
        <Controls />
    {/if}
</article>
