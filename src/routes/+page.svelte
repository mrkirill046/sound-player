<script lang="ts">
    import {onMount, onDestroy} from "svelte"
    import {invoke} from "@tauri-apps/api/core"
    import {open} from "@tauri-apps/plugin-dialog"
    import {getCurrentWebview} from "@tauri-apps/api/webview"
    import Header from "@/components/shared/header.svelte"
    import {debug, info, error} from "@tauri-apps/plugin-log"
    import {currentPath, isPlaying, hasAudio} from "@/stores/audio-store"
    import {writable} from "svelte/store"

    const isHovering = writable(false)

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
                currentPath.set(selected)
                hasAudio.set(true)

                info(`File path found: ${selected}`)

                await playAudio(selected)
            }
        } catch (e) {
            error(`Failed to open file dialog: ${e}`)
        }
    }

    async function playAudio(path: string) {
        debug("Invoke play_audio function")

        try {
            await invoke("play_audio", {path})

            isPlaying.set(true)
        } catch (e) {
            error(`play_audio failed: ${e}`)
        }
    }

    async function pauseAudio() {
        debug("Invoke pause_audio function")

        try {
            await invoke("pause_audio")

            isPlaying.set(false)
        } catch (e) {
            error(`pause_audio failed: ${e}`)
        }
    }

    async function resumeAudio() {
        debug("Invoke resume_audio function")

        try {
            await invoke("resume_audio")

            isPlaying.set(true)
        } catch (e) {
            error(`resume_audio failed: ${e}`)
        }
    }

    async function restartAudio() {
        debug("Invoke restart_audio function")

        try {
            await invoke("restart_audio")

            isPlaying.set(true)
        } catch (e) {
            error(`restart_audio failed: ${e}`)
        }
    }

    function isInsideElement(el: HTMLElement, x: number, y: number): boolean {
        const rect = el.getBoundingClientRect()

        return x >= rect.left && x <= rect.right && y >= rect.top && y <= rect.bottom
    }

    onMount(async () => {
        unlistenDragEvents = await getCurrentWebview().onDragDropEvent((event) => {
            switch (event.payload.type) {
                case "enter": {
                    debug("Webview enter event triggered")

                    const {x, y} = event.payload.position

                    if (isInsideElement(dropZone, x, y)) {
                        isHovering.set(true)
                    }

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

                    break
                }

                case "drop": {
                    debug("Webview drop event triggered")

                    const {x, y} = event.payload.position

                    if (isInsideElement(dropZone, x, y) && event.payload.paths.length > 0) {
                        const path = event.payload.paths[0]

                        currentPath.set(path)
                        hasAudio.set(true)

                        info(`File path found: ${path}`)

                        playAudio(path)
                    } else {
                        debug("Drop was outside dropZone — ignored")
                    }

                    isHovering.set(false)

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

<Header />

<main class="flex flex-col">
    <section class="flex flex-col h-full w-full justify-center items-center flex-1 space-y-8">
        <div class="pr-8 pl-8">
            <button
                type="button"
                class="border-2 border-dashed border-secondary-500 text-center rounded-lg select-none bg-surface-900 p-24"
                bind:this={dropZone}
                on:click={openFileDialog}
            >
                <p class="text-sm text-secondary-200">
                    {#if $isHovering}
                        Drop file
                    {:else}
                        Click or drag & drop file here
                    {/if}
                </p>
            </button>
        </div>

        {#if $currentPath}
            <div class="flex flex-col items-center justify-center">
                <div class="flex gap-3">
                    <button
                        on:click={resumeAudio}
                        class="bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded transition"
                        aria-label="Resume"
                    >
                        ▶️ Старт
                    </button>
                    <button
                        on:click={pauseAudio}
                        class="bg-yellow-500 hover:bg-yellow-600 text-white px-4 py-2 rounded transition"
                        aria-label="Pause"
                    >
                        ⏸ Пауза
                    </button>
                    <button
                        on:click={restartAudio}
                        class="bg-red-500 hover:bg-red-600 text-white px-4 py-2 rounded transition"
                        aria-label="Restart"
                    >
                        🔁 Заново
                    </button>
                </div>

                <div>
                    <p class="text-sm text-gray-500 mt-2 break-all">
                        Текущий файл: <code>{$currentPath}</code>
                    </p>
                </div>
            </div>
        {/if}
    </section>
</main>
