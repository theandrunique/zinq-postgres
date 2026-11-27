<script lang="ts">
  import { MessageSquare } from "@lucide/svelte";
</script>

<svelte:head>
  <title>Zinq • Chats</title>
</svelte:head>

<div class="flex h-full flex-col items-center justify-center gap-(--space-1) px-(--space-4) text-center">
  <MessageSquare class="h-12 w-12 text-(--color-text-subtle)" />
  <h2 class="text-(length:--font-size-lg) font-semibold">Select a chat</h2>
  <p class="max-w-sm text-(length:--font-size-sm) text-(--color-text-muted)">
    Open an existing chat on the left or create a new one with the plus button.
  </p>
</div>
