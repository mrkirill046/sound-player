import type {LayoutLoad} from "./$types"
import "@/app.css"
import "@/lib/i18n"

export const ssr = false
export const prerender = true

export const load: LayoutLoad = (children) => {
    return {children}
}
