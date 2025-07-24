<script>
    import {cn} from "@/lib/utils"

    export let size = 24
</script>

<svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    class={cn($$props.class, "lucide-icon lucide")}
    width={size}
    height={size}
    fill="currentColor"
>
    <defs>
        <mask id="play-mask">
            <rect width="24" height="24" fill="white" />
            <polygon points="10,8 16,12 10,16" fill="black" />
        </mask>
    </defs>

    <circle cx="12" cy="12" r="10" mask="url(#play-mask)" />
</svg>
