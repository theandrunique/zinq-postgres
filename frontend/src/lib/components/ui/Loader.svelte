<script lang="ts">
  import { cn } from "#lib/utils";

  let { class: className, size = "md" }: { class?: string; size?: "sm" | "md" | "lg" } = $props();

  const sizes = {
    sm: "h-(--loader-size-small) w-(--loader-size-small)",
    md: "h-(--loader-size-medium) w-(--loader-size-medium)",
    lg: "h-(--loader-size-large) w-(--loader-size-large)",
  };
</script>

<div
  class={cn(
    "animate-loader rounded-full",
    "border-(length:--loader-border-width) border-(color:--loader-track-color) border-t-(color:--loader-indicator-color)",
    sizes[size],
    className,
  )}
></div>
