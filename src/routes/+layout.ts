import type {LayoutLoad} from "./$types"
import "../app.css"

export const ssr = false
export const prerender = true

export const load: LayoutLoad = (children) => {
    return {children}
}
