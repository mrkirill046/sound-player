<script lang="ts">
    import {pauseAudio, restartAudio, resumeAudio} from "@/lib/player"
    import {cover, currentPath} from "@/stores/audio-store"
    import {quadInOut} from "svelte/easing"
    import {fade} from "svelte/transition"
</script>

<div class="flex flex-col items-center justify-center" in:fade={{duration: 150, easing: quadInOut}}>
    <article>
        {#if $cover}
            <img src={$cover} alt="Cover" class="object-cover rounded-xl w-48 h-48" />
        {:else}
            <div></div>
        {/if}
    </article>

    <div class="flex gap-3">
        <button
            on:click={resumeAudio}
            class="bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded transition"
            aria-label="Resume"
        >
            ▶️ Старт
        </button>
        <button
            on:click={pauseAudio}
            class="bg-yellow-500 hover:bg-yellow-600 text-white px-4 py-2 rounded transition"
            aria-label="Pause"
        >
            ⏸ Пауза
        </button>
        <button
            on:click={restartAudio}
            class="bg-red-500 hover:bg-red-600 text-white px-4 py-2 rounded transition"
            aria-label="Restart"
        >
            🔁 Заново
        </button>
    </div>

    <div>
        <p class="text-sm text-gray-500 mt-2 break-all">
            Текущий файл: <code>{$currentPath}</code>
        </p>
    </div>
</div>
