import {writable} from "svelte/store"

export const currentPath = writable<string | null>(null)
export const isPlaying = writable(false)
export const hasAudio = writable(false)
export const cover = writable<string | null>(null)
