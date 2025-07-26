<script>
    import Header from "@/components/shared/header.svelte"
    import HotkeysSettings from "@/components/shared/hotkeys-settings.svelte"
    import PaletteSettings from "@/components/shared/palette-settings.svelte"
</script>

<Header title="Settings" />

<main class="flex flex-col flex-1 w-full h-full justify-center">
    <PaletteSettings />
    <HotkeysSettings />
</main>
