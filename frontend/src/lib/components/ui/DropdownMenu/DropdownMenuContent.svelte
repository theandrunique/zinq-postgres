<script lang="ts">
  import { DropdownMenu } from "bits-ui";
  import { cn } from "#lib/utils";

  let { children, class: className, sideOffset = 4, ...props }: DropdownMenu.ContentProps = $props();

  let contentClasses = $derived(
    cn(
      "dropdown-menu-content z-(--menu-z-index) max-h-(--bits-dropdown-menu-content-available-height) min-w-(--menu-min-width) overflow-y-hidden",
      "rounded-(--menu-radius) p-(--menu-padding) bg-(--menu-background) shadow-(--menu-shadow)",
      "origin-(--bits-dropdown-menu-content-transform-origin)",
      className,
    ),
  );
</script>

<DropdownMenu.Content class={contentClasses} {sideOffset} {...props}>
  {@render children?.()}
</DropdownMenu.Content>

<style>
  :global(.dropdown-menu-content[data-state="open"]) {
    animation: popover-in var(--menu-duration) var(--ease-out);
  }

  :global(.dropdown-menu-content[data-state="closed"]) {
    animation: popover-out var(--menu-duration) var(--ease-out);
  }
</style>
