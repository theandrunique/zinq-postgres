<script lang="ts">
  import { ArrowLeft, Users } from "@lucide/svelte";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { Avatar, Button } from "#lib/components/ui";
  import type { Chat } from "#lib/types";
  import {
    getChatAvatar,
    getChatAvatarFallback,
    getChatDisplayName,
    getOtherMember,
    isSavedMessages,
  } from "#lib/utils/chat";

  interface Props {
    chat: Chat;
    currentUserId: string;
  }

  let { chat, currentUserId }: Props = $props();

  let saved = $derived(isSavedMessages(chat, currentUserId));
  let displayName = $derived(getChatDisplayName(chat, currentUserId));
  let avatarSrc = $derived(saved ? "/bookmark.png" : getChatAvatar(chat, currentUserId));
  let fallback = $derived(avatarSrc ? "" : getChatAvatarFallback(chat, currentUserId));

  let subtitle = $derived.by(() => {
    if (chat.type === "GROUP_DM") {
      const count = chat.members.length;
      return `${count} ${count === 1 ? "участник" : count < 5 ? "участника" : "участников"}`;
    }

    const other = getOtherMember(chat, currentUserId);
    return other ? `@${other.username}` : "";
  });
</script>

<header
  class="flex h-14 shrink-0 items-center gap-(--space-1) border-b border-(color:--color-border-base) bg-(--color-background-surface) px-(--space-1) md:px-(--space-2)"
>
  <Button
    variant="icon"
    type="button"
    aria-label="Назад к чатам"
    class="md:hidden"
    onclick={() => goto(resolve("/chats"))}
  >
    <ArrowLeft class="h-5 w-5" />
  </Button>

  <Avatar src={avatarSrc} alt={displayName} {fallback} class="h-9 w-9" />

  <div class="min-w-0 flex-1">
    <div class="flex items-center gap-1">
      {#if chat.type === "GROUP_DM"}
        <Users class="h-4 w-4 shrink-0 text-(--color-text-muted)" />
      {/if}
      <h2 class="truncate text-(length:--font-size-md) font-semibold">{displayName}</h2>
    </div>
    {#if subtitle}
      <p class="truncate text-(length:--font-size-xs) text-(--color-text-muted)">{subtitle}</p>
    {/if}
  </div>
</header>
