<script lang="ts">
  import { Dialog } from "bits-ui";
  import { cn } from "#lib/utils";

  let { children, class: className, ...rest }: Dialog.TitleProps = $props();

  let cls = $derived(
    cn(
      "mb-(--dialog-title-margin-bottom) text-(length:--dialog-title-font-size) font-(weight:--dialog-title-font-weight)",
      className,
    ),
  );
</script>

<Dialog.Title class={cls} {...rest}>
  {@render children?.()}
</Dialog.Title>
