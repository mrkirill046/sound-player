import {cover, isPlaying} from "@/stores/audio-store"
import {invoke} from "@tauri-apps/api/core"
import {debug, error, info} from "@tauri-apps/plugin-log"

export async function playAudio(path: string) {
    debug("Invoke play_audio function")

    try {
        await invoke("play_audio", {path})

        isPlaying.set(true)

        await loadCover(path)
    } catch (e) {
        error(`play_audio failed: ${e}`)
    }
}

export async function pauseAudio() {
    debug("Invoke pause_audio function")

    try {
        await invoke("pause_audio")

        isPlaying.set(false)
    } catch (e) {
        error(`pause_audio failed: ${e}`)
    }
}

export async function resumeAudio() {
    debug("Invoke resume_audio function")

    try {
        await invoke("resume_audio")

        isPlaying.set(true)
    } catch (e) {
        error(`resume_audio failed: ${e}`)
    }
}

export async function restartAudio() {
    debug("Invoke restart_audio function")

    try {
        await invoke("restart_audio")

        isPlaying.set(true)
    } catch (e) {
        error(`restart_audio failed: ${e}`)
    }
}

export async function loadCover(path: string) {
    debug("Invoke get_audio_cover function")

    try {
        let invokedCover: string | null = await invoke("get_audio_cover", {path})

        if (invokedCover) {
            cover.set(invokedCover)

            info("Audio cover found")
        } else {
            info("Audio cover not found")
        }
    } catch (e) {
        error(`get_audio_cover failed: ${e}`)
    }
}
