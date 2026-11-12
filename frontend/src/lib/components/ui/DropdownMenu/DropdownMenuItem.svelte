<script lang="ts">
  import { DropdownMenu } from "bits-ui";
  import { cn } from "#lib/utils";

  let { children, class: className, ...rest }: DropdownMenu.ItemProps = $props();

  let itemClasses = $derived(
    cn(
      "relative flex cursor-pointer select-none items-center gap-(--menu-item-gap)",
      "rounded-(--menu-item-radius) px-(--menu-item-padding-x) py-(--menu-item-padding-y)",
      "text-(length:--menu-item-font-size) text-(--menu-item-text) outline-none",
      "hover:bg-(--menu-item-background-hover) focus:bg-(--menu-item-background-hover) data-[highlighted]:bg-(--menu-item-background-hover)",
      "data-[disabled]:cursor-not-allowed data-[disabled]:text-(--menu-item-text-disabled) data-[disabled]:opacity-(--menu-item-opacity-disabled)",
      "data-[disabled]:hover:bg-transparent data-[disabled]:focus:bg-transparent data-[disabled]:data-[highlighted]:bg-transparent",
      className,
    ),
  );
</script>

<DropdownMenu.Item class={itemClasses} {...rest}>
  {@render children?.()}
</DropdownMenu.Item>
