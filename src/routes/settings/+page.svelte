<script>
    import {t} from "svelte-i18n"
    import Header from "@/components/shared/header.svelte"
    import HotkeysSettings from "@/components/shared/hotkeys-settings.svelte"
    import PaletteSettings from "@/components/shared/palette-settings.svelte"
    import LocaleSettings from "@/components/shared/locale-settings.svelte"
</script>

<Header title={$t("Header.Settings")}/>

<main class="flex flex-col flex-1 w-full h-full justify-center">
    <LocaleSettings/>
    <PaletteSettings/>
    <HotkeysSettings/>
</main>
