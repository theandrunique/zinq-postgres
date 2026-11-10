<script lang="ts">
  import { Select, type WithoutChild } from "bits-ui";
  import { cn } from "#lib/utils";
  import { Check, ChevronDown } from "@lucide/svelte";

  type Props = Omit<WithoutChild<Select.RootProps>, "type" | "value" | "onValueChange"> & {
    type?: "single";
    value?: string;
    onValueChange?: (value: string) => void;
    placeholder?: string;
    items: { value: string; label: string; disabled?: boolean }[];
  };
  let { value = $bindable(), items, placeholder, type = "single", ...rest }: Props = $props();

  let triggerClasses = $derived(
    cn(
      "flex h-(--select-trigger-height) w-full cursor-pointer items-center justify-between",
      "rounded-(--select-trigger-radius) border-(length:--input-border-width) border-(color:--select-trigger-border)",
      "px-(--select-trigger-padding-x) py-(--select-trigger-padding-y)",
      "bg-(--select-trigger-background) text-(length:--select-trigger-font-size) text-(--select-trigger-text)",
      "hover:bg-(--select-trigger-background-hover) hover:border-(color:--select-trigger-border-hover)",
      "focus:border-(color:--select-trigger-border-focus) focus:outline-(length:--input-outline-width) focus:outline-(color:--select-trigger-border-focus) focus:outline-offset-0",
      "data-[state=open]:border-(color:--select-trigger-border-open) data-[state=open]:outline-(length:--input-outline-width) data-[state=open]:outline-(color:--select-trigger-border-open) data-[state=open]:outline-offset-0",
      "data-[state=open]:[&_svg]:rotate-180",
      "disabled:cursor-not-allowed disabled:bg-(--select-trigger-background-disabled) disabled:opacity-(--select-trigger-opacity-disabled)",
      "data-[placeholder]:font-(weight:--input-font-weight-placeholder) data-[placeholder]:text-(--select-trigger-text-placeholder)",
    ),
  );

  let contentClasses = $derived(
    cn(
      "select-content z-(--menu-z-index) max-h-(--bits-select-content-available-height) min-w-(--bits-select-anchor-width)",
      "rounded-(--menu-radius) p-(--menu-padding) bg-(--menu-background) shadow-(--menu-shadow)",
      "origin-(--bits-select-content-transform-origin)",
    ),
  );

  let itemClasses = $derived(
    cn(
      "relative flex cursor-pointer select-none items-center justify-between gap-(--menu-item-gap)",
      "rounded-(--menu-item-radius) px-(--menu-item-padding-x) py-(--menu-item-padding-y)",
      "text-(length:--menu-item-font-size) text-(--menu-item-text) outline-none",
      "hover:bg-(--menu-item-background-hover) focus:bg-(--menu-item-background-hover) data-[highlighted]:bg-(--menu-item-background-hover)",
      "data-[disabled]:pointer-events-none data-[disabled]:opacity-(--menu-item-opacity-disabled)",
    ),
  );
</script>

<Select.Root bind:value {items} {type} {...rest}>
  <Select.Trigger class={triggerClasses}>
    <Select.Value {placeholder} />
    <span class="shrink-0 text-(--select-trigger-fill) transition-transform duration-(--duration-slow)">
      <ChevronDown class="h-(--button-icon-size) w-(--button-icon-size)" />
    </span>
  </Select.Trigger>

  <Select.Portal>
    <Select.Content class={contentClasses} sideOffset={4} collisionPadding={5}>
      {#each items as { value, label, disabled } (value)}
        <Select.Item {value} {label} {disabled} class={itemClasses}>
          {#snippet children({ selected })}
            {label}
            {#if selected}
              <Check class="h-(--button-icon-size) w-(--button-icon-size)" />
            {/if}
          {/snippet}
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Portal>
</Select.Root>

<style>
  :global(.select-content[data-state="open"]) {
    animation: popover-in var(--menu-duration) var(--ease-out);
  }

  :global(.select-content[data-state="closed"]) {
    animation: popover-out var(--menu-duration) var(--ease-out);
  }
</style>
