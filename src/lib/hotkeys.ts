import {get} from "svelte/store"
import {hotkeys} from "@/stores/hotkeys-store"
import {hasAudio, isPlaying} from "@/stores/audio-store"
import {pauseAudio, resumeAudio, nextAudio, previousAudio, restartAudio} from "@/lib/player"

type Cleanup = () => void

export function setupHotkeys(): Cleanup {
    const handler = (e: KeyboardEvent) => {
        const cfg = get(hotkeys)

        const target = e.target as HTMLElement | null

        if (target && (["INPUT", "TEXTAREA"].includes(target.tagName) || target.isContentEditable)) {
            return
        }

        const code = e.code

        if (code === cfg.playPause) {
            e.preventDefault()

            if (get(hasAudio)) {
                get(isPlaying) ? pauseAudio() : resumeAudio()
            }

            return
        }

        if (code === cfg.next) {
            e.preventDefault()

            if (get(hasAudio)) {
                nextAudio()
            }

            return
        }

        if (code === cfg.prev) {
            e.preventDefault()

            if (get(hasAudio)) {
                previousAudio()
            }

            return
        }

        if (code === cfg.restart) {
            e.preventDefault()

            if (get(hasAudio)) {
                restartAudio()
            }

            return
        }
    }

    window.addEventListener("keydown", handler)

    return () => window.removeEventListener("keydown", handler)
}
