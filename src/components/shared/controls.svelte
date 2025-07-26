<script lang="ts">
    import {nextAudio, pauseAudio, previousAudio, restartAudio, resumeAudio} from "@/lib/player"
    import {cover, currentPath, isPlaying} from "@/stores/audio-store"
    import {Heart, RotateCcw, SkipBack, SkipForward} from "@lucide/svelte"
    import {quadInOut} from "svelte/easing"
    import {fade} from "svelte/transition"
    import CustomCirclePlay from "@/components/icons/custom-circle-play.svelte"
    import CustomCirclePause from "@/components/icons/custom-circle-pause.svelte"
    import {cn} from "@/lib/utils"
</script>

<div class="flex flex-col items-center justify-center gap-y-5" in:fade={{duration: 150, easing: quadInOut}}>
    <article>
        <div class="flex flex-col w-full h-full px-4 gap-y-4">
            {#if $cover}
                <img
                    src={$cover}
                    alt="Cover"
                    class={cn(
                        "object-cover rounded-xl transition-transform duration-300 ease-in-out w-[30rem] sm:w-[40rem]",
                        $isPlaying ? "scale-100" : "scale-95"
                    )}
                />
            {:else}
                <img
                    src="/no-cover.webp"
                    alt="Cover not found"
                    class={cn(
                        "object-cover bg-gray-500/40 rounded-xl transition-transform duration-300 ease-in-out w-[30rem] sm:w-[40rem] animate-pulse",
                        $isPlaying ? "scale-100" : "scale-95"
                    )}
                />
            {/if}

            {@render Menu()}
        </div>
    </article>

    <div>
        <p class="text-sm text-gray-500 mt-2 break-all px-6 sm:px-12 lg:px-24">
            <code class="font-marker-gothic">
                {$currentPath}
            </code>
        </p>
    </div>
</div>

{#snippet Menu()}
    <div class="flex items-center justify-between w-full px-4 sm:px-8">
        <button on:click={restartAudio} aria-label="Restart">
            <RotateCcw />
        </button>

        <div class="flex gap-3">
            <button on:click={previousAudio} aria-label="Previous Audio">
                <SkipBack fill="currentColor" size={42} />
            </button>

            {#if $isPlaying}
                <button on:click={pauseAudio} aria-label="Pause">
                    <CustomCirclePause size={96} />
                </button>
            {:else}
                <button on:click={resumeAudio} aria-label="Resume">
                    <CustomCirclePlay size={96} />
                </button>
            {/if}

            <button on:click={nextAudio} aria-label="Next Audio">
                <SkipForward fill="currentColor" size={42} />
            </button>
        </div>

        <button aria-label="Add to favorite">
            <Heart />
        </button>
    </div>
{/snippet}
