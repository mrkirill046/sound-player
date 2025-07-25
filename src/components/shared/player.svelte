<script lang="ts">
    import {hasAudio} from "@/stores/audio-store"
    import Controls from "./controls.svelte"
    import DropZone from "./drop-zone.svelte"
    import {onDestroy, onMount} from "svelte"
    import {setupHotkeys} from "@/lib/hotkeys"

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
    })

    onDestroy(() => {
        cleanup?.()
    })
</script>

<article class="space-y-8 w-full">
    {#if !$hasAudio}
        <DropZone />
    {:else if delayed}
        <Controls />
    {/if}
</article>
