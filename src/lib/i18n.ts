import {init, register} from "svelte-i18n"

register("en", () => import("@/locales/en.json"))
register("ru", () => import("@/locales/ru.json"))

export const STORAGE_KEY = "locale_code_v1"

const savedLocale = localStorage.getItem(STORAGE_KEY)

if (savedLocale) {
    init({
        fallbackLocale: "en",
        initialLocale: savedLocale
    })
} else {
    init({
        fallbackLocale: "en",
        initialLocale: "en"
    })
}
