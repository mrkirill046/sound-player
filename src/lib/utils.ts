import {createToaster} from "@skeletonlabs/skeleton-svelte"
import {invoke} from "@tauri-apps/api/core"
import {debug, error} from "@tauri-apps/plugin-log"
import {type ClassValue, clsx} from "clsx"
import {twMerge} from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs))
}

export const isFileSupported = async (path: string): Promise<boolean> => {
    debug("Invoke is_valid_audio_file function")

    try {
        let invokedBool: boolean = await invoke("is_valid_audio_file", {path})

        if (invokedBool) {
            return true
        } else {
            error("Audio file not supported")

            return false
        }
    } catch (e) {
        error(`is_valid_audio_file failed: ${e}`)

        return false
    }
}

export const toaster = createToaster({
    placement: "top",
    max: 3,
    duration: 1500
})
