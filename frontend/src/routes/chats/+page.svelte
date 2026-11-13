<script lang="ts">
  import { MessageSquare } from "@lucide/svelte";
</script>

<svelte:head>
  <title>Zinq • Чаты</title>
</svelte:head>

<div class="flex h-full flex-col items-center justify-center gap-(--space-1) px-(--space-4) text-center">
  <MessageSquare class="h-12 w-12 text-(--color-text-subtle)" />
  <h2 class="text-(length:--font-size-lg) font-semibold">Выберите чат</h2>
  <p class="max-w-sm text-(length:--font-size-sm) text-(--color-text-muted)">
    Откройте существующий чат слева или создайте новый кнопкой с плюсом.
  </p>
</div>
