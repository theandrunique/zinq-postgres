<script lang="ts">
  import type { HTMLTextareaAttributes } from "svelte/elements";
  import { cn } from "#lib/utils";

  type Props = HTMLTextareaAttributes & {
    value?: string;
  };

  let { value = $bindable(""), class: className = "", ...rest }: Props = $props();
</script>

<textarea
  bind:value
  class={cn(
    "w-full px-(--input-padding-x) py-(--input-padding-y)",
    "min-h-(--textarea-min-height) resize-y",
    "rounded-(--input-radius) border-(length:--input-border-width) border-(color:--input-border)",
    "bg-(--input-background) text-(length:--input-font-size) font-(weight:--input-font-weight) text-(--input-text)",
    "placeholder:font-(weight:--input-font-weight-placeholder) placeholder:text-(--input-text-placeholder)",
    "hover:border-(color:--input-border-hover) hover:bg-(--input-background-hover)",
    "focus:border-(color:--input-border-focus) focus:bg-(--input-background-focus) focus:outline-(length:--input-outline-width) focus:outline-offset-0 focus:outline-(color:--input-border-focus) focus:hover:bg-(--input-background-focus)",
    "aria-invalid:border-(color:--input-border-error)",
    "disabled:cursor-not-allowed disabled:border-(color:--input-border-disabled) disabled:bg-(--input-background-disabled) disabled:text-(--input-text-disabled) disabled:opacity-(--input-opacity-disabled)",
    className,
  )}
  {...rest}></textarea>
