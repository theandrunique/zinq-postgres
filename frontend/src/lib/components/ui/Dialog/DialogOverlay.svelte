<script lang="ts">
  import { Dialog } from "bits-ui";
  import { cn } from "#lib/utils";

  let { class: className, ...rest }: Dialog.OverlayProps = $props();

  let cls = $derived(
    cn(
      "dialog-overlay data-nested:hidden fixed inset-0 z-(--dialog-overlay-z-index)",
      "bg-(--dialog-overlay-background) backdrop-blur-(--dialog-overlay-blur)",
      className,
    ),
  );
</script>

<Dialog.Overlay class={cls} {...rest} />

<style>
  :global(.dialog-overlay[data-state="open"]) {
    animation: fade-in var(--dialog-duration) var(--ease-out);
  }

  :global(.dialog-overlay[data-state="closed"]) {
    animation: fade-out var(--dialog-duration) var(--ease-out);
  }
</style>
