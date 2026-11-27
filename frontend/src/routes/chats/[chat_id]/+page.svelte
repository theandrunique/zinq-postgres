<script lang="ts">
  import { createInfiniteQuery, createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query";
  import { page } from "$app/state";
  import { MessageCircle } from "@lucide/svelte";
  import { Button, Loader } from "$lib/components/ui";
  import { appendMessage, updateLastMessage } from "$lib/queries/messenger/cache.ts";
  import { ApiError } from "$lib/api/client";
  import { getChat, sendMessage } from "$lib/api/chats";
  import { messagesQueryOptions } from "$lib/queries/messenger/queries.ts";
  import { chatKeys } from "$lib/queries/messenger/query-keys.ts";
  import { auth } from "$lib/auth.svelte.ts";
  import { ackMessageVisible, ackReachedBottom, flushChatAcks } from "$services/ack-queue.ts";
  import { chatSession } from "$lib/state/chat-session.svelte.ts";
  import { userProfileModal } from "$services/userProfileModal.svelte.ts";
  import { getChatDisplayName } from "$features/messenger/model/chatHelpers.ts";
  import { normalizeMessages } from "$features/messenger/model/messageHelpers.ts";
  import { ChatHeader, MessageComposer, MessageList } from "$features/messenger";

  const queryClient = useQueryClient();

  const chatId = $derived(page.params.chat_id ?? "");
  const currentUserId = $derived(auth.user?.profile.id ?? "");

  const chatQuery = createQuery(() => ({
    queryKey: chatKeys.detail(chatId),
    queryFn: () => getChat(chatId),
    enabled: chatId.length > 0,
  }));

  const messagesQuery = createInfiniteQuery(() => ({
    ...messagesQueryOptions(chatId),
    enabled: chatId.length > 0,
  }));

  const chat = $derived(chatQuery.data ?? null);
  const rawMessages = $derived(messagesQuery.data ?? []);
  const messages = $derived(chat ? normalizeMessages(rawMessages) : []);
  const displayName = $derived(chat ? getChatDisplayName(chat, currentUserId) : "Chat");

  let sendError = $state<string | null>(null);

  const sendMutation = createMutation(() => ({
    mutationFn: (content: string) => sendMessage(chatId, { content }),
    onSuccess: (message) => {
      appendMessage(queryClient, message);
      updateLastMessage(queryClient, message);
    },
  }));

  $effect(() => {
    chatSession.setActiveChat(chatId || null);

    return () => chatSession.setActiveChat(null);
  });

  $effect(() => {
    const id = chatId;

    return () => void flushChatAcks(id);
  });

  function handleMessageVisible(messageId: string) {
    ackMessageVisible(chatId, messageId);
  }

  function handleReachedBottom(messageId: string) {
    ackReachedBottom(chatId, messageId);
  }

  function handleAuthorClick(userId: string) {
    userProfileModal.open(userId);
  }

  async function handleSend(content: string) {
    sendError = null;

    try {
      await sendMutation.mutateAsync(content);
    } catch (cause) {
      sendError = cause instanceof ApiError ? cause.message : "Couldn't send the message.";
      throw cause;
    }
  }

  function handleLoadOlder() {
    if (!messagesQuery.hasNextPage || messagesQuery.isFetchingNextPage) return;

    void messagesQuery.fetchNextPage();
  }
</script>

<svelte:head>
  <title>Zinq • {displayName}</title>
</svelte:head>

{#if chatQuery.isPending}
  <div class="flex h-full items-center justify-center gap-(--space-1)">
    <Loader size="md" />
    <span class="text-(length:--font-size-sm) text-(--color-text-muted)">Loading chat…</span>
  </div>
{:else if chatQuery.isError}
  <div class="flex h-full flex-col items-center justify-center gap-(--space-1) px-(--space-4) text-center">
    <p class="text-(length:--font-size-sm) text-(--color-text-muted)">
      {chatQuery.error instanceof ApiError ? chatQuery.error.message : "Couldn't load the chat."}
    </p>
    <Button variant="secondary" onclick={() => chatQuery.refetch()}>Retry</Button>
  </div>
{:else if chat}
  <div class="flex h-full min-h-0 flex-col">
    <ChatHeader {chat} {currentUserId} />

    <div class="min-h-0 flex-1">
      {#if messagesQuery.isPending}
        <div class="flex h-full items-center justify-center gap-(--space-1)">
          <Loader size="sm" />
          <span class="text-(length:--font-size-sm) text-(--color-text-muted)">Loading messages…</span>
        </div>
      {:else if messagesQuery.isError}
        <div class="flex h-full flex-col items-center justify-center gap-(--space-1) px-(--space-4) text-center">
          <p class="text-(length:--font-size-sm) text-(--color-text-muted)">Couldn't load messages.</p>
          <Button variant="secondary" onclick={() => messagesQuery.refetch()}>Retry</Button>
        </div>
      {:else if messages.length === 0}
        <div class="flex h-full flex-col items-center justify-center gap-(--space-1) px-(--space-4) text-center">
          <MessageCircle class="h-10 w-10 text-(--color-text-subtle)" />
          <p class="text-(length:--font-size-sm) text-(--color-text-muted)">No messages yet. Be the first to write!</p>
        </div>
      {:else}
        {#key chatId}
          <MessageList
            {messages}
            {currentUserId}
            lastReadMessageId={chat.last_read_message_id ?? ""}
            maxReadMessageId={chat.max_read_message_id ?? ""}
            onMessageVisible={handleMessageVisible}
            onReachedBottom={handleReachedBottom}
            onLoadOlder={handleLoadOlder}
            onAuthorClick={handleAuthorClick}
          />
        {/key}
      {/if}
    </div>

    {#if messagesQuery.isFetchingNextPage}
      <div class="flex justify-center border-t border-(color:--color-border-subtle) py-(--space-05)">
        <Loader size="sm" />
      </div>
    {/if}

    {#if sendError}
      <p
        role="alert"
        class="border-t border-(color:--color-border-error) bg-(--color-background-error-subtle) px-(--space-2) py-(--space-05) text-(length:--font-size-sm) text-(--color-text-error)"
      >
        {sendError}
      </p>
    {/if}

    <MessageComposer onSend={handleSend} disabled={sendMutation.isPending} />
  </div>
{:else}
  <div class="flex h-full items-center justify-center">
    <p class="text-(length:--font-size-sm) text-(--color-text-muted)">Chat not found.</p>
  </div>
{/if}
