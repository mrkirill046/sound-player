<script lang="ts">
    import {hotkeys, defaultHotkeys, type HotkeyAction} from "@/stores/hotkeys-store"
    import {get} from "svelte/store"

    let local = {...get(hotkeys)}
    let waitingFor: HotkeyAction | null = null

    function startCapture(action: HotkeyAction) {
        waitingFor = action
    }

    function onKeydown(e: KeyboardEvent) {
        if (!waitingFor) return

        e.preventDefault()

        const code = e.code

        local = {...local, [waitingFor]: code}

        hotkeys.set(local)

        waitingFor = null
    }

    function resetToDefaults() {
        local = {...defaultHotkeys}

        hotkeys.set(local)
    }
</script>

<svelte:window on:keydown={onKeydown} />

<section class="space-y-6 p-6 px-12">
    <h2 class="text-4xl font-bold font-marker-gothic text-center">Hotkeys</h2>

    <div class="overflow-x-auto rounded-lg shadow">
        <table class="w-full border-collapse bg-surface-100-900 rounded-lg text-center">
            <thead>
                <tr class="bg-surface-300-700">
                    <th class="p-3 text-sm font-semibold font-marker-gothic">Action</th>
                    <th class="p-3 text-sm font-semibold font-marker-gothic">Key</th>
                </tr>
            </thead>

            <tbody>
                {#each Object.entries(local) as [action, code]}
                    <tr
                        class="border-b border-surface-50-950 hover:bg-surface-200-800 transition-colors duration-300 ease-in-out"
                    >
                        <td class="p-3 text-sm font-marker-gothic capitalize">
                            {action}
                        </td>

                        <td class="p-3">
                            {#if waitingFor === action}
                                <span class="px-2 py-1 bg-secondary-50-950/20 rounded text-sm"> Press key... </span>
                            {:else}
                                <button
                                    class="px-2 py-1 bg-secondary-50-950/60 rounded text-sm transition-colors duration-300 ease-in-out hover:bg-secondary-50-950/100 font-marker-gothic"
                                    on:click={() => startCapture(action as HotkeyAction)}
                                >
                                    {code}
                                </button>
                            {/if}
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>

    <button
        class="mt-4 px-4 py-2 bg-red-500 text-white rounded text-sm font-marker-gothic hover:bg-red-600 transition-colors duration-300 ease-in-out"
        on:click={resetToDefaults}
    >
        Reset to default
    </button>
</section>
