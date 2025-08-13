import {init, register} from "svelte-i18n"

import en from "@/locales/en.json"
import ru from "@/locales/ru.json"

register("en", () => Promise.resolve(en))
register("ru", () => Promise.resolve(ru))

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
