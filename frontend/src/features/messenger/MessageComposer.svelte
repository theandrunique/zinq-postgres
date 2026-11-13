<script lang="ts">
  import { SendHorizontal } from "@lucide/svelte";
  import { Button, Textarea } from "#lib/components/ui";

  interface Props {
    onSend: (content: string) => Promise<void>;
    disabled?: boolean;
  }

  let { onSend, disabled = false }: Props = $props();

  let value = $state("");
  let sending = $state(false);

  let canSend = $derived(value.trim().length > 0 && !sending && !disabled);

  async function send() {
    const content = value.trim();

    if (!content || sending || disabled) return;

    sending = true;

    try {
      await onSend(content);
      value = "";
    } catch {
      // The parent component renders the error.
    } finally {
      sending = false;
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key !== "Enter" || event.shiftKey || event.isComposing) return;

    event.preventDefault();
    void send();
  }
</script>

<form
  class="flex shrink-0 items-end gap-(--space-1) border-t border-(color:--color-border-base) bg-(--color-background-surface) p-(--space-1) md:p-(--space-2)"
  onsubmit={(event) => {
    event.preventDefault();
    void send();
  }}
>
  <Textarea
    bind:value
    placeholder="Написать сообщение…"
    rows={1}
    disabled={disabled || sending}
    class="max-h-40 min-h-10 resize-none py-(--space-05)"
    onkeydown={handleKeydown}
  />

  <Button type="submit" disabled={!canSend} aria-label="Отправить" class="h-10 w-10 shrink-0">
    <SendHorizontal class="h-5 w-5" />
  </Button>
</form>
