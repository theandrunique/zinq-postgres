<script lang="ts">
  import { createQuery } from "@tanstack/svelte-query";
  import ChatCard from "#lib/components/ChatCard/ChatCard.svelte";
  import { getUnreadRange, unreadQueryOptions } from "#lib/api/queries";
  import type { Chat } from "#lib/types";

  interface Props {
    chat: Chat;
    currentUserId: string;
    selected?: boolean;
    onclick?: () => void;
  }

  let { chat, currentUserId, selected = false, onclick }: Props = $props();

  let hasUnread = $derived(getUnreadRange(chat, currentUserId) !== null);

  const unreadQuery = createQuery(() => unreadQueryOptions(chat, currentUserId));

  let unreadCount = $derived(hasUnread ? (unreadQuery.data?.count ?? 0) : 0);
</script>

<ChatCard {chat} {currentUserId} {selected} {unreadCount} {onclick} />
