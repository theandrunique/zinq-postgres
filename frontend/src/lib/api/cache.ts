import type { InfiniteData, QueryClient } from "@tanstack/svelte-query";
import type { ApiMessage, Chat, ChatsPage, LastMessageInfo, MessagesCountResponse } from "#lib/types";
import { maxId } from "#lib/utils/id";
import { chatKeys } from "./query-keys";

export type ChatsData = InfiniteData<ChatsPage, string | null>;
export type MessagesData = InfiniteData<ApiMessage[], string | null>;

export function toLastMessageInfo(message: ApiMessage): LastMessageInfo {
  return {
    message_id: message.id,
    author_id: message.author_id,
    content: message.content,
    message_type: message.type,
    edited_at: message.edited_at,
    created_at: message.created_at,
  };
}

export function patchChat(client: QueryClient, chatId: string, updater: (chat: Chat) => Chat): void {
  client.setQueryData<ChatsData>(chatKeys.list(), (old) => {
    if (!old) return old;

    return {
      ...old,
      pages: old.pages.map((page) => ({
        ...page,
        chats: page.chats.map((chat) => (chat.id === chatId ? updater(chat) : chat)),
      })),
    };
  });

  client.setQueryData<Chat>(chatKeys.detail(chatId), (old) => (old ? updater(old) : old));
}

export function touchChat(client: QueryClient, chatId: string): void {
  client.setQueryData<ChatsData>(chatKeys.list(), (old) => {
    if (!old || old.pages.length === 0) return old;

    const moved = old.pages.flatMap((page) => page.chats).find((chat) => chat.id === chatId);

    if (!moved) return old;

    const pages = old.pages.map((page) => ({
      ...page,
      chats: page.chats.filter((chat) => chat.id !== chatId),
    }));

    pages[0] = {
      ...pages[0],
      chats: [moved, ...pages[0].chats],
    };

    return { ...old, pages };
  });
}

export function upsertChat(client: QueryClient, chat: Chat): void {
  const data = client.getQueryData<ChatsData>(chatKeys.list());

  if (!data) {
    void client.invalidateQueries({ queryKey: chatKeys.list() });
    return;
  }

  const exists = data.pages.some((page) => page.chats.some((item) => item.id === chat.id));

  if (exists) {
    patchChat(client, chat.id, (current) => ({ ...current, ...chat }));
    touchChat(client, chat.id);
    return;
  }

  client.setQueryData<ChatsData>(chatKeys.list(), (old) => {
    if (!old || old.pages.length === 0) return old;

    const pages = [...old.pages];
    pages[0] = { ...pages[0], chats: [chat, ...pages[0].chats] };

    return { ...old, pages };
  });
}

export function updateLastMessage(client: QueryClient, message: ApiMessage): void {
  patchChat(client, message.chat_id, (chat) => ({
    ...chat,
    last_message: toLastMessageInfo(message),
  }));
  touchChat(client, message.chat_id);
}

export function patchLastMessageIfCurrent(client: QueryClient, message: ApiMessage): void {
  patchChat(client, message.chat_id, (chat) =>
    chat.last_message?.message_id === message.id ? { ...chat, last_message: toLastMessageInfo(message) } : chat,
  );
}

export function appendMessage(client: QueryClient, message: ApiMessage): void {
  client.setQueryData<MessagesData>(chatKeys.messages(message.chat_id), (old) => {
    if (!old) return old;
    if (old.pages.some((page) => page.some((item) => item.id === message.id))) return old;

    const pages = old.pages.length > 0 ? [...old.pages] : [[]];
    pages[0] = [message, ...pages[0]];

    return { ...old, pages };
  });
}

export function patchMessage(client: QueryClient, message: ApiMessage): void {
  client.setQueryData<MessagesData>(chatKeys.messages(message.chat_id), (old) => {
    if (!old) return old;

    let found = false;

    const pages = old.pages.map((page) =>
      page.map((item) => {
        if (item.id !== message.id) return item;

        found = true;
        return message;
      }),
    );

    return found ? { ...old, pages } : old;
  });
}

export function removeMessage(client: QueryClient, chatId: string, messageId: string): void {
  client.setQueryData<MessagesData>(chatKeys.messages(chatId), (old) => {
    if (!old) return old;

    return {
      ...old,
      pages: old.pages.map((page) => page.filter((item) => item.id !== messageId)),
    };
  });
}

export function removeMessageEverywhere(client: QueryClient, messageId: string): void {
  client.setQueriesData<MessagesData>({ queryKey: [...chatKeys.all, "messages"] }, (old) => {
    if (!old) return old;

    let found = false;

    const pages = old.pages.map((page) =>
      page.filter((item) => {
        if (item.id !== messageId) return true;

        found = true;
        return false;
      }),
    );

    return found ? { ...old, pages } : old;
  });
}

export function setUnread(client: QueryClient, chatId: string, count: number): void {
  client.setQueryData<MessagesCountResponse>(chatKeys.unread(chatId), (old) => (old ? { ...old, count } : old));
}

export function bumpUnread(client: QueryClient, chatId: string, delta = 1): void {
  client.setQueryData<MessagesCountResponse>(chatKeys.unread(chatId), (old) =>
    old ? { ...old, count: Math.max(0, old.count + delta) } : old,
  );
}

export function markChatRead(client: QueryClient, chatId: string, lastReadMessageId: string): void {
  patchChat(client, chatId, (chat) => ({
    ...chat,
    last_read_message_id: maxId([chat.last_read_message_id, lastReadMessageId]),
  }));
  setUnread(client, chatId, 0);
}

export function applyMessageAck(
  client: QueryClient,
  ack: { user_id: string; chat_id: string; last_read_message_id: string },
  currentUserId: string,
): void {
  if (ack.user_id === currentUserId) {
    markChatRead(client, ack.chat_id, ack.last_read_message_id);
    return;
  }

  patchChat(client, ack.chat_id, (chat) => ({
    ...chat,
    max_read_message_id: maxId([chat.max_read_message_id, ack.last_read_message_id]),
  }));
}

export function invalidateChat(client: QueryClient, chatId: string): void {
  void client.invalidateQueries({ queryKey: chatKeys.detail(chatId) });
  void client.invalidateQueries({ queryKey: chatKeys.list() });
}
