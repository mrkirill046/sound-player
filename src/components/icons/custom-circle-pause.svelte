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
        <mask id="pause-mask">
            <rect fill="white" height="24" width="24"/>
            <rect fill="black" height="8" width="2" x="9" y="8"/>
            <rect fill="black" height="8" width="2" x="13" y="8"/>
        </mask>
    </defs>

    <circle cx="12" cy="12" mask="url(#pause-mask)" r="10"/>
</svg>
