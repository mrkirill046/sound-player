import {writable} from "svelte/store"
import type {Theme} from "@/@types/theme"

function getInitialTheme(): Theme {
    if (typeof window === "undefined") return "light"

    const stored = localStorage.getItem("theme")

    if (stored === "dark" || stored === "light") return stored as Theme

    return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light"
}

export const theme = writable<Theme>(getInitialTheme())

theme.subscribe((value) => {
    if (typeof document !== "undefined") {
        document.documentElement.setAttribute("data-mode", value)
    }
    if (typeof window !== "undefined") {
        localStorage.setItem("theme", value)
    }
})

export function toggleTheme() {
    theme.update((current) => (current === "dark" ? "light" : "dark"))
}
