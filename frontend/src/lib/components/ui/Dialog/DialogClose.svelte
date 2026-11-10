<script lang="ts">
  import { Dialog } from "bits-ui";
  import { cn } from "#lib/utils";
  import { X } from "@lucide/svelte";

  let { class: className, ...rest }: Dialog.CloseProps = $props();

  let cls = $derived(
    cn(
      "absolute top-(--dialog-close-offset) right-(--dialog-close-offset) cursor-pointer",
      "rounded-(--button-radius) bg-(--button-background-icon) p-(--button-padding-y) text-(--button-text-icon)",
      "hover:bg-(--button-background-icon-hover) focus-visible:focus-ring",
      className,
    ),
  );
</script>

<Dialog.Close class={cls} {...rest}>
  <X class="h-(--button-icon-size) w-(--button-icon-size)" />
</Dialog.Close>
