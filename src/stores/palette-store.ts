import {writable} from "svelte/store"

export type Palette =
    | "wintry"
    | "catppuccin"
    | "cerberus"
    | "concord"
    | "crimson"
    | "nosh"
    | "vintage"
    | "vox"
    | "terminus"
    | "seafoam"

const THEME_NAMES: Palette[] = [
    "wintry",
    "catppuccin",
    "cerberus",
    "concord",
    "crimson",
    "nosh",
    "vintage",
    "vox",
    "terminus",
    "seafoam"
]

const COLOR_KEYS = ["primary", "secondary", "tertiary", "success", "warning", "error", "surface"]
const STORAGE_KEY = "palette_code_v1"

export const palettes = writable<Record<Palette, string[]>>(
    THEME_NAMES.reduce((acc, theme) => {
        acc[theme] = []

        return acc
    }, {} as Record<Palette, string[]>)
)

export const currentPalette = writable<Palette>("wintry")

export function loadPalettes() {
    const map = {} as Record<Palette, string[]>
    const originalTheme = document.documentElement.getAttribute("data-theme") || "wintry"

    for (const theme of THEME_NAMES) {
        document.documentElement.setAttribute("data-theme", theme)

        const styles = getComputedStyle(document.documentElement)

        map[theme] = COLOR_KEYS.map((key) => styles.getPropertyValue(`--color-${key}-500`).trim())
    }

    document.documentElement.setAttribute("data-theme", originalTheme)

    palettes.set(map)
}

export function initPalette() {
    const stored = localStorage.getItem(STORAGE_KEY) as Palette | null

    if (stored && THEME_NAMES.includes(stored)) {
        currentPalette.set(stored)
    }

    currentPalette.subscribe((value) => {
        document.documentElement.setAttribute("data-theme", value)
        localStorage.setItem(STORAGE_KEY, value)
    })

    loadPalettes()
}

export function setPallette(palette: Palette) {
    if (THEME_NAMES.includes(palette)) {
        currentPalette.set(palette)
    } else {
        currentPalette.set("wintry")
    }

    document.documentElement.setAttribute("data-theme", palette)
    localStorage.setItem(STORAGE_KEY, palette)
}
