import type {LayoutLoad} from "./$types"

export const ssr = false
export const prerender = true

export const load: LayoutLoad = (children) => {
    return {children}
}
