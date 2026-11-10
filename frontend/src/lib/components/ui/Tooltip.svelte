<script lang="ts">
  import { Tooltip } from "bits-ui";
  import type { Snippet } from "svelte";
  import { cn } from "#lib/utils";

  type Props = Tooltip.RootProps & {
    trigger: Snippet;
    content: Snippet;
    triggerProps?: Tooltip.TriggerProps;
  };

  let { trigger, content, triggerProps, ...rest }: Props = $props();
</script>

<Tooltip.Provider>
  <Tooltip.Root {...rest}>
    <Tooltip.Trigger {...triggerProps}>
      {@render trigger()}
    </Tooltip.Trigger>
    <Tooltip.Portal>
      <Tooltip.Content
        side="top"
        class={cn(
          "tooltip z-(--tooltip-z-index) max-w-(--tooltip-max-width) overflow-hidden",
          "rounded-(--tooltip-radius) px-(--tooltip-padding-x) py-(--tooltip-padding-y)",
          "bg-(--tooltip-background) text-(length:--tooltip-font-size) text-(--tooltip-text) shadow-(--tooltip-shadow)",
        )}
        sideOffset={8}
        collisionPadding={8}
      >
        {@render content()}
      </Tooltip.Content>
    </Tooltip.Portal>
  </Tooltip.Root>
</Tooltip.Provider>

<style>
  :global(.tooltip) {
    animation: popover-in var(--tooltip-duration) var(--ease-out);
  }

  :global(.tooltip[data-state="closed"]) {
    animation: popover-out var(--tooltip-duration) var(--ease-out);
  }
</style>
