<script lang="ts">
  import { Check, CheckCheck } from "@lucide/svelte";
  import { cn } from "#lib/utils";
  import { idIsGreaterOrEqual } from "#lib/utils/id";
  import type { Message } from "#lib/types";
  import { formatMessageTime } from "./utils";

  interface Props {
    message: Message;
    isOwnMessage: boolean;
    maxReadMessageId?: string;
    class?: string;
  }

  let { message, isOwnMessage, maxReadMessageId = "", class: className = "" }: Props = $props();

  let isEdited = $derived(message.edited_at !== null);
  let isRead = $derived(Boolean(maxReadMessageId) && idIsGreaterOrEqual(maxReadMessageId, message.id));
</script>

<div
  class={cn(
    "flex h-5 items-center justify-end text-[12px] whitespace-nowrap opacity-70",
    isOwnMessage ? "text-(--color-text-on-brand)" : "text-(--color-text-muted)",
    className,
  )}
>
  <span class="mr-1">
    {#if isEdited}изм.{/if}
    {formatMessageTime(message.created_at)}
  </span>
  {#if isOwnMessage}
    {#if isRead}
      <CheckCheck class="h-4 w-4" />
    {:else}
      <Check class="h-4 w-4" />
    {/if}
  {/if}
</div>
