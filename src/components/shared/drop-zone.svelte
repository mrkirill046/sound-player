<script lang="ts">
    import {cn, isFileSupported} from "@/lib/utils"
    import {debug, error, info} from "@tauri-apps/plugin-log"
    import {quadInOut} from "svelte/easing"
    import {writable} from "svelte/store"
    import {open} from "@tauri-apps/plugin-dialog"
    import {fade} from "svelte/transition"
    import {onDestroy, onMount} from "svelte"
    import {getCurrentWebview} from "@tauri-apps/api/webview"
    import {currentPath, hasAudio} from "@/stores/audio-store"
    import {playAudio} from "@/lib/player"

    const isHovering = writable(false)
    const isEnter = writable(false)

    let dropZone!: HTMLButtonElement
    let unlistenDragEvents: (() => void) | null = null

    async function openFileDialog() {
        debug("User triggered dialog")

        try {
            const selected = await open({
                multiple: false,
                filters: [
                    {
                        name: "Audio",
                        extensions: ["mp3", "wav", "ogg", "flac", "m4a"]
                    }
                ]
            })

            if (typeof selected === "string") {
                if (await isFileSupported(selected)) {
                    currentPath.set(selected)
                    hasAudio.set(true)

                    info(`File path found: ${selected}`)

                    setTimeout(async () => {
                        await playAudio(selected)
                    }, 300)
                }
            }
        } catch (e) {
            error(`Failed to open file dialog: ${e}`)
        }
    }

    function isInsideElement(el: HTMLElement, x: number, y: number): boolean {
        const rect = el.getBoundingClientRect()

        return x >= rect.left && x <= rect.right && y >= rect.top && y <= rect.bottom
    }

    onMount(async () => {
        unlistenDragEvents = await getCurrentWebview().onDragDropEvent(async (event) => {
            switch (event.payload.type) {
                case "enter": {
                    debug("Webview enter event triggered")

                    const {x, y} = event.payload.position

                    if (isInsideElement(dropZone, x, y)) {
                        isHovering.set(true)
                    }

                    isEnter.set(true)

                    break
                }

                case "over": {
                    debug("Webview over event triggered")

                    const {x, y} = event.payload.position

                    isHovering.set(isInsideElement(dropZone, x, y))

                    break
                }

                case "leave": {
                    debug("Webview leave event triggered")

                    isHovering.set(false)
                    isEnter.set(false)

                    break
                }

                case "drop": {
                    debug("Webview drop event triggered")

                    const {x, y} = event.payload.position

                    if (isInsideElement(dropZone, x, y) && event.payload.paths.length > 0) {
                        const path = event.payload.paths[0]

                        if (await isFileSupported(path)) {
                            currentPath.set(path)
                            hasAudio.set(true)

                            info(`File path found: ${path}`)

                            setTimeout(() => {
                                playAudio(path)
                            }, 300)
                        }
                    } else {
                        debug("Drop was outside dropZone — ignored")
                    }

                    isHovering.set(false)
                    isEnter.set(false)

                    break
                }
            }
        })
    })

    onDestroy(() => {
        if (unlistenDragEvents) {
            unlistenDragEvents()
        }
    })
</script>

<div
    class="pr-4 pl-4 sm:pl-24 sm:pr-24 lg:pr-32 lg:pl-32"
    out:fade={{duration: 150, easing: quadInOut}}
    on:outroend={(el: CustomEvent) => {
        const target = el.target as HTMLElement

        target.classList.add("hidden")
    }}
>
    <button
        type="button"
        class={cn(
            "border-2 z-10 w-full p-24 text-center rounded-lg select-none",
            "transition-all duration-300 hover:shadow-md",
            "bg-surface-100-900 border-secondary-400-600 text-secondary-800-200",
            $isEnter ? "ring-4 ring-blue-400 shadow-2xl scale-[1.1]" : "border-dashed"
        )}
        bind:this={dropZone}
        on:click={openFileDialog}
    >
        <p class="text-sm font-marker-gothic">
            {#if $isHovering}
                Drop file here
            {:else if $isEnter}
                Drag file here
            {:else}
                Click or drag & drop file here
            {/if}
        </p>
    </button>
</div>
