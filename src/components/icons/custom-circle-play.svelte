<script>
    import {cn} from "@/lib/utils"

    export let size = 24
</script>

<svg
        class={cn($$props.class, "lucide-icon lucide")}
        fill="currentColor"
        height={size}
        viewBox="0 0 24 24"
        width={size}
        xmlns="http://www.w3.org/2000/svg"
>
    <defs>
        <mask id="play-mask">
            <rect fill="white" height="24" width="24"/>
            <polygon fill="black" points="10,8 16,12 10,16"/>
        </mask>
    </defs>

    <circle cx="12" cy="12" mask="url(#play-mask)" r="10"/>
</svg>
