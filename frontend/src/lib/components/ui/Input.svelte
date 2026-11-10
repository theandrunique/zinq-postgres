<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";
  import { cn } from "#lib/utils";

  type Props = HTMLInputAttributes & {
    value?: string;
  };

  let { value = $bindable(""), class: className = "", ...rest }: Props = $props();

  let classes = $derived(
    cn(
      "h-(--input-height) w-full px-(--input-padding-x) py-(--input-padding-y)",
      "rounded-(--input-radius) border-(length:--input-border-width) border-(color:--input-border)",
      "bg-(--input-background) text-(length:--input-font-size) font-(weight:--input-font-weight) text-(--input-text)",
      "placeholder:font-(weight:--input-font-weight-placeholder) placeholder:text-(--input-text-placeholder)",
      "hover:bg-(--input-background-hover) hover:border-(color:--input-border-hover)",
      "focus:border-(color:--input-border-focus) focus:outline-(length:--input-outline-width) focus:outline-(color:--input-border-focus) focus:outline-offset-0",
      "aria-invalid:border-(color:--input-border-error)",
      "disabled:cursor-not-allowed disabled:border-(color:--input-border-disabled) disabled:bg-(--input-background-disabled) disabled:text-(--input-text-disabled) disabled:opacity-(--input-opacity-disabled)",
      className,
    ),
  );
</script>

<input bind:value class={classes} {...rest} />
