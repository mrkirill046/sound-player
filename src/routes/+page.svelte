<script lang="ts">
    import Header from "@/components/shared/header.svelte"
    import Player from "@/components/shared/player.svelte"
</script>

<Header />

<main class="flex flex-col">
    <section class="flex flex-col h-full w-full justify-center items-center flex-1">
        <Player />
    </section>
</main>
