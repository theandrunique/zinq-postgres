<script lang="ts">
  import { Tooltip } from "bits-ui";
  import type { Snippet } from "svelte";
  import { cn } from "#lib/utils";

  type Props = Tooltip.RootProps & {
    trigger: Snippet;
    content: Snippet;
    triggerProps?: Tooltip.TriggerProps;
    contentProps?: Tooltip.ContentProps;
  };

  let { trigger, content, triggerProps, contentProps, ...rest }: Props = $props();

  let contentClasses = $derived(
    cn(
      "tooltip z-(--tooltip-z-index) max-w-(--tooltip-max-width) overflow-hidden break-words",
      "rounded-(--tooltip-radius) px-(--tooltip-padding-x) py-(--tooltip-padding-y)",
      "bg-(--tooltip-background) text-(length:--tooltip-font-size) font-(weight:--tooltip-font-weight) leading-(--tooltip-line-height) text-(--tooltip-text-color) shadow-(--tooltip-shadow)",
      contentProps?.class,
    ),
  );
</script>

<Tooltip.Root {...rest}>
  <Tooltip.Trigger {...triggerProps}>
    {@render trigger()}
  </Tooltip.Trigger>
  <Tooltip.Portal>
    <Tooltip.Content side="top" sideOffset={8} collisionPadding={8} {...contentProps} class={contentClasses}>
      {@render content()}
    </Tooltip.Content>
  </Tooltip.Portal>
</Tooltip.Root>

<style>
  :global(.tooltip) {
    animation: popover-in var(--tooltip-duration) var(--ease-out);
  }

  :global(.tooltip[data-state="closed"]) {
    animation: popover-out var(--tooltip-duration) var(--ease-out);
  }
</style>
