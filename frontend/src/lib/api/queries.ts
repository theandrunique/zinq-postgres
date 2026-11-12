import type { InfiniteData } from "@tanstack/svelte-query";
import type { ApiMessage, Chat, ChatsPage } from "#lib/types";
import { compareIds, nextId } from "#lib/utils/id";
import { countMessages, getChats, getMessages } from "./chats";
import { chatKeys } from "./query-keys";

export const MESSAGES_PAGE_SIZE = 50;
export const CHATS_PAGE_SIZE = 20;

export function selectChats(data: InfiniteData<ChatsPage, string | null>): Chat[] {
  const seen = new Set<string>();
  const result: Chat[] = [];

  for (const page of data.pages) {
    for (const chat of page.chats) {
      if (seen.has(chat.id)) continue;

      seen.add(chat.id);
      result.push(chat);
    }
  }

  return result;
}

export function selectMessages(data: InfiniteData<ApiMessage[], string | null>): ApiMessage[] {
  const seen = new Set<string>();
  const result: ApiMessage[] = [];

  for (const page of data.pages) {
    for (const message of page) {
      if (seen.has(message.id)) continue;

      seen.add(message.id);
      result.push(message);
    }
  }

  return result.sort((left, right) => compareIds(left.id, right.id));
}

export function chatsQueryOptions() {
  return {
    queryKey: chatKeys.list(),
    queryFn: ({ pageParam }: { pageParam: string | null }) => getChats({ cursor: pageParam, limit: CHATS_PAGE_SIZE }),
    initialPageParam: null as string | null,
    getNextPageParam: (lastPage: ChatsPage) => lastPage.next_cursor ?? undefined,
    select: selectChats,
  };
}

export function messagesQueryOptions(chatId: string) {
  return {
    queryKey: chatKeys.messages(chatId),
    queryFn: ({ pageParam }: { pageParam: string | null }) =>
      getMessages(chatId, { before: pageParam, limit: MESSAGES_PAGE_SIZE }),
    initialPageParam: null as string | null,
    getNextPageParam: (lastPage: ApiMessage[]) =>
      lastPage.length >= MESSAGES_PAGE_SIZE ? lastPage[lastPage.length - 1].id : undefined,
    select: selectMessages,
  };
}

export interface UnreadRange {
  from?: string;
  to: string;
}

export function getUnreadRange(chat: Chat, currentUserId: string): UnreadRange | null {
  const lastMessage = chat.last_message;

  if (!lastMessage) return null;
  if (lastMessage.author_id === currentUserId) return null;
  if (chat.last_read_message_id && compareIds(chat.last_read_message_id, lastMessage.message_id) >= 0) {
    return null;
  }

  return {
    from: chat.last_read_message_id ? nextId(chat.last_read_message_id) : undefined,
    to: lastMessage.message_id,
  };
}

export function unreadQueryOptions(chat: Chat, currentUserId: string) {
  const range = getUnreadRange(chat, currentUserId);

  return {
    queryKey: chatKeys.unread(chat.id),
    queryFn: () => countMessages(chat.id, range?.from, range?.to),
    enabled: range !== null,
    staleTime: 60_000,
  };
}
