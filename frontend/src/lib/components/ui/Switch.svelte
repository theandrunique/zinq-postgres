<script lang="ts">
  import { Switch } from "bits-ui";
  import { cn } from "#lib/utils";

  let { checked = $bindable(false), class: className, ...rest }: Switch.RootProps = $props();

  let rootClasses = $derived(
    cn(
      "peer inline-flex shrink-0 cursor-pointer items-center",
      "h-(--switch-height) w-(--switch-width) rounded-(--switch-radius)",
      "border-(length:--switch-border-width) border-(color:--switch-track-border)",
      "bg-(--switch-track-background) transition-colors duration-(--duration-fast)",
      "enabled:hover:bg-(--switch-track-background-hover)",
      "data-[state=checked]:border-(color:--switch-track-border-checked)",
      "data-[state=checked]:bg-(--switch-track-background-checked)",
      "data-[state=checked]:enabled:hover:bg-(--switch-track-background-checked-hover)",
      "focus-visible:focus-ring",
      "data-[disabled]:cursor-not-allowed data-[disabled]:opacity-(--switch-opacity-disabled)",
      className,
    ),
  );

  let thumbClasses = $derived(
    cn(
      "pointer-events-none block rounded-full",
      "h-(--switch-thumb-size) w-(--switch-thumb-size)",
      "bg-(--switch-thumb-background) shadow-(--switch-thumb-shadow) ring-0",
      "transition-transform duration-(--duration-fast)",
      "data-[state=unchecked]:translate-x-(--switch-thumb-offset)",
      "data-[state=checked]:translate-x-(--switch-thumb-offset-checked)",
      "data-[state=checked]:bg-(--switch-thumb-background-checked)",
    ),
  );
</script>

<Switch.Root bind:checked class={rootClasses} {...rest}>
  <Switch.Thumb class={thumbClasses} />
</Switch.Root>
