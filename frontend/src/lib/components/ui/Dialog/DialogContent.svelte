<script lang="ts">
  import { Dialog } from "bits-ui";
  import { cn } from "#lib/utils";

  let { children, class: className, ...rest }: Dialog.ContentProps = $props();

  let cls = $derived(
    cn(
      "dialog-content fixed left-[50%] top-[50%] z-(--dialog-z-index) translate-x-[-50%]",
      "w-full max-w-[calc(100%_-_2rem)] p-(--dialog-padding) rounded-(--dialog-radius) bg-(--dialog-background) text-(--dialog-text) shadow-(--dialog-shadow)",
      "translate-y-[calc(-50%_+_var(--bits-dialog-nested-count,_0)_*_-1_*_var(--dialog-nested-offset))]",
      "transition-all duration-(--dialog-duration) sm:max-w-(--dialog-max-width)",
      className,
    ),
  );
</script>

<Dialog.Content
  class={cls}
  {...rest}
  style="transform: scale(calc(1 - var(--bits-dialog-nested-count, 0) * var(--dialog-nested-scale)));
         filter: blur(calc(var(--bits-dialog-nested-count, 0) * var(--dialog-nested-blur)));"
>
  {@render children?.()}
</Dialog.Content>

<style>
  :global(.dialog-content[data-state="open"]) {
    animation: popover-in var(--dialog-duration) var(--ease-out);
  }

  :global(.dialog-content[data-state="closed"]) {
    animation: popover-out var(--dialog-duration) var(--ease-out);
  }
</style>
