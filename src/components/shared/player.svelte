<script lang="ts">
    import {hasAudio} from "@/stores/audio-store"
    import Controls from "./controls.svelte"
    import DropZone from "./drop-zone.svelte"
    import {onDestroy, onMount} from "svelte"
    import {setupHotkeys} from "@/lib/hotkeys"
    import {toaster} from "@/lib/utils"
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

    onMount(async () => {
        cleanup = setupHotkeys()
    })

    onDestroy(() => {
        cleanup?.()
    })
</script>

<Toaster {toaster}/>

<article class="space-y-8 w-full my-2">
    {#if !$hasAudio}
        <DropZone/>
    {:else if delayed}
        <Controls/>
    {/if}
</article>
