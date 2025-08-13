<script lang="ts">
    import {currentPalette, type Palette, palettes, setPallette} from "@/stores/palette-store"
    import {get} from "svelte/store"
    import {t} from "svelte-i18n"

    let paletteMap = get(palettes)
    let selected = get(currentPalette)

    palettes.subscribe((value) => (paletteMap = value))
    currentPalette.subscribe((value) => (selected = value))

    function applyTheme(theme: string) {
        setPallette(theme as Palette)
    }

    function resetToDefaults() {
        setPallette("wintry")
    }
</script>

<section class="space-y-6 p-6 px-12">
    <h2 class="text-4xl font-bold font-marker-gothic text-center">{$t("Settings.Themes")}</h2>

    <div class="overflow-x-auto rounded-lg shadow">
        <table class="w-full border-collapse bg-surface-100-900 rounded-lg text-center">
            <thead>
                <tr class="bg-surface-300-700">
                    <th class="p-3 text-sm font-semibold font-marker-gothic">{$t("Settings.Theme")}</th>
                    <th class="p-3 text-sm font-semibold font-marker-gothic">{$t("Settings.Colors")}</th>
                    <th class="p-3 text-sm font-semibold font-marker-gothic">{$t("Settings.Action")}</th>
                </tr>
            </thead>

            <tbody>
                {#each Object.entries(paletteMap) as [theme, colors]}
                    <tr
                        class="border-b border-surface-50-950 hover:bg-surface-200-800 transition-colors duration-300 ease-in-out"
                    >
                        <td class="p-3 text-sm font-marker-gothic capitalize">{theme}</td>

                        <td class="p-3 flex gap-1 justify-center -space-x-2">
                            {#each colors as color}
                                <span class="w-4 h-4 rounded-full" style="background-color: {color}"></span>
                            {/each}
                        </td>

                        <td class="p-3">
                            <button
                                class="px-2 py-1 bg-secondary-50-950/60 rounded text-sm transition-colors duration-300 ease-in-out hover:bg-secondary-50-950/100 font-marker-gothic"
                                class:selected={selected === theme}
                                on:click={() => applyTheme(theme)}
                            >
                                {selected === theme ? $t("Settings.Current") : $t("Settings.Apply")}
                            </button>
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
        {$t("Settings.Reset")}
    </button>
</section>
