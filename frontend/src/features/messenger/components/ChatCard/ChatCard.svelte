<script lang="ts">
  import { Check, CheckCheck, Users } from "@lucide/svelte";
  import { Avatar } from "#lib/components/ui";
  import type { Chat } from "#lib/types";
  import { getChatAvatar, getChatAvatarFallback, getChatDisplayName, isSavedMessages } from "#lib/utils/chat";
  import { compareIds } from "#lib/utils/id";
  import { cn } from "#lib/utils";
  import { formatTime, getMessagePreview } from "./utils";

  interface Props {
    chat: Chat;
    currentUserId: string;
    unreadCount?: number;
    selected?: boolean;
    onclick?: () => void;
  }

  let { chat, currentUserId, unreadCount = 0, selected = false, onclick }: Props = $props();

  let saved = $derived(isSavedMessages(chat, currentUserId));
  let displayName = $derived(getChatDisplayName(chat, currentUserId));
  let avatarSrc = $derived(saved ? "/bookmark.png" : getChatAvatar(chat, currentUserId));
  let avatarFallback = $derived(avatarSrc ? "" : getChatAvatarFallback(chat, currentUserId));

  let isOwnLastMessage = $derived(chat.last_message?.author_id === currentUserId);

  let isMessageRead = $derived.by(() => {
    if (!chat.last_message || !chat.max_read_message_id) return false;
    return compareIds(chat.max_read_message_id, chat.last_message.message_id) >= 0;
  });

  let lastMessagePreview = $derived(getMessagePreview(chat, currentUserId));

  let timestamp = $derived.by(() => {
    if (!chat.last_message) return "";
    return formatTime(chat.last_message.created_at);
  });
</script>

<button
  type="button"
  class={cn(
    "flex w-full items-center gap-3 px-2 py-2.5 text-left transition-colors duration-100",
    "enabled:hover:bg-(--color-background-interactive-hover) enabled:active:bg-(--color-background-interactive-active)",
    selected && "bg-(--color-background-interactive-active)",
  )}
  {onclick}
>
  <div class="shrink-0">
    {#if saved}
      <Avatar src="/bookmark.png" alt="Избранное" fallback="" class="h-10 w-10" />
    {:else}
      <Avatar src={avatarSrc} alt={displayName} fallback={avatarFallback} class="h-10 w-10" />
    {/if}
  </div>

  <div class="min-w-0 flex-1">
    <div class="flex items-center justify-between gap-2">
      <div class="flex min-w-0 items-center gap-1 font-semibold">
        {#if chat.type === "GROUP_DM"}
          <Users class="h-4 w-4 shrink-0 text-(--color-text-muted)" />
        {/if}
        <span class="truncate text-sm text-(--color-text-base)">
          {displayName}
        </span>
      </div>

      <div class="flex shrink-0 items-center gap-1">
        {#if isOwnLastMessage}
          {#if isMessageRead}
            <CheckCheck class="h-4 w-4 text-(--color-text-brand)" />
          {:else}
            <Check class="h-4 w-4 text-(--color-text-brand)" />
          {/if}
        {/if}

        {#if timestamp}
          <span class="text-xs text-(--color-text-muted)">{timestamp}</span>
        {/if}
      </div>
    </div>

    <div class="flex items-center justify-between gap-2">
      <div class="truncate text-sm text-(--color-text-muted)">
        {#if lastMessagePreview}
          {lastMessagePreview}
        {:else}
          <span class="italic">Нет сообщений</span>
        {/if}
      </div>

      {#if unreadCount > 0}
        <div
          class="ml-1 inline-flex h-4 min-w-4 shrink-0 items-center justify-center rounded-full bg-(--color-background-brand) px-1 text-[10px] leading-none font-semibold whitespace-nowrap text-(--color-text-on-brand)"
        >
          {unreadCount > 99 ? "99+" : unreadCount}
        </div>
      {/if}
    </div>
  </div>
</button>
