import {writable} from "svelte/store"

export type HotkeyAction = "playPause" | "next" | "prev" | "restart"
export type HotkeysMap = Record<HotkeyAction, string>

const STORAGE_KEY = "hotkeys_code_v1"

export const defaultHotkeys: HotkeysMap = {
    playPause: "Space",
    next: "ArrowRight",
    prev: "ArrowLeft",
    restart: "KeyR"
}

function load(): HotkeysMap {
    try {
        const raw = localStorage.getItem(STORAGE_KEY)

        if (!raw) return defaultHotkeys

        const parsed = JSON.parse(raw)

        return {...defaultHotkeys, ...parsed}
    } catch {
        return defaultHotkeys
    }
}

export const hotkeys = writable<HotkeysMap>(load())

hotkeys.subscribe((val) => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(val))
})
