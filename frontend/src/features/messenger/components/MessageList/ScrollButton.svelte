<script lang="ts">
  import { ChevronDown } from "@lucide/svelte";
  import { cn } from "#lib/utils";
  import type { ScrollManager } from "./scroll-manager.svelte.ts";

  interface Props {
    manager: ScrollManager;
  }

  let { manager }: Props = $props();

  let buttonState = $derived(manager.buttonState);

  let label = $derived.by(() => {
    if (buttonState === "toBottom") return "Вниз";
    if (buttonState === "toLastRead") return "К непрочитанным";
    if (buttonState === "goBack") return "Назад";
    return "";
  });
</script>

{#if buttonState !== "hidden"}
  <button
    type="button"
    onclick={() => {
      if (buttonState === "toBottom") manager.scrollToBottom();
      else if (buttonState === "toLastRead") manager.scrollToLastRead();
      else if (buttonState === "goBack") manager.goBack();
    }}
    class={cn(
      "absolute right-4 bottom-4 z-20 flex h-10 w-10 items-center justify-center rounded-full shadow-(--shadow-elevation-3) transition-all duration-200 active:scale-95",
      "bg-(--color-background-surface-raised) enabled:hover:bg-(--color-background-interactive-hover) enabled:active:bg-(--color-background-interactive-active)",
    )}
    aria-label={label}
    title={label}
  >
    <ChevronDown class={cn("h-5 w-5", buttonState === "goBack" && "rotate-180")} />
  </button>
{/if}
