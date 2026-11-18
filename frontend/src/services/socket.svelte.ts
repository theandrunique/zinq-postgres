import { io, type Socket } from "socket.io-client";
import type { ApiMessage, GatewayEvent, HelloEvent } from "$lib/types";
import {
  appendMessage,
  applyMessageAck,
  bumpUnread,
  invalidateChat,
  patchLastMessageIfCurrent,
  patchMessage,
  removeMessageEverywhere,
  updateLastMessage,
  upsertChat,
} from "$lib/queries/messenger/cache.ts";
import { queryClient } from "$lib/queries/query-client.ts";
import { chatKeys } from "$lib/queries/messenger/query-keys.ts";
import { auth } from "$lib/auth.svelte.ts";
import { chatSession } from "$lib/state/chat-session.svelte.ts";

const LIST_INVALIDATE_DEBOUNCE_MS = 1_000;

let socket: Socket | null = null;
let refreshingToken = false;
let listInvalidateTimer: ReturnType<typeof setTimeout> | null = null;

function isPageVisible(): boolean {
  return typeof document === "undefined" || document.visibilityState === "visible";
}

/**
 * A message arrived for a chat that is not in the loaded pages (for example,
 * an old chat the user has not scrolled to yet). The list has to be refetched
 * for the chat to jump to the top, but events are coalesced to avoid a request
 * storm when several such chats receive messages at once.
 */
function scheduleChatListInvalidate(): void {
  if (listInvalidateTimer) return;

  listInvalidateTimer = setTimeout(() => {
    listInvalidateTimer = null;
    void queryClient.invalidateQueries({ queryKey: chatKeys.list() });
  }, LIST_INVALIDATE_DEBOUNCE_MS);
}

function handleMessageCreate(message: ApiMessage): void {
  const currentUserId = auth.user?.profile.id ?? "";

  appendMessage(queryClient, message);

  const knownChat = updateLastMessage(queryClient, message);

  if (!knownChat) {
    scheduleChatListInvalidate();
  }

  if (message.author_id === currentUserId) return;

  if (chatSession.activeChatId === message.chat_id) {
    // Visible messages are acknowledged by the message list itself; hidden
    // tabs need an explicit counter bump.
    if (!isPageVisible()) {
      bumpUnread(queryClient, message.chat_id);
    }
    return;
  }

  bumpUnread(queryClient, message.chat_id);
}

function handleEvent(event: GatewayEvent): void {
  const currentUserId = auth.user?.profile.id;

  if (!currentUserId) return;

  switch (event.type) {
    case "MessageCreate":
      handleMessageCreate(event.payload.message);
      break;

    case "MessageUpdate":
      patchMessage(queryClient, event.payload.message);
      patchLastMessageIfCurrent(queryClient, event.payload.message);
      break;

    case "MessageDelete":
      removeMessageEverywhere(queryClient, event.payload.message_id);
      void queryClient.invalidateQueries({ queryKey: chatKeys.list() });
      break;

    case "ChatCreate":
      upsertChat(queryClient, event.payload.chat);
      queryClient.setQueryData(chatKeys.detail(event.payload.chat.id), event.payload.chat);
      break;

    case "ChatMemberAdd":
    case "ChatMemberRemove":
      invalidateChat(queryClient, event.payload.chat_id);
      break;

    case "MessageAck":
      applyMessageAck(queryClient, event.payload, currentUserId);
      break;
  }
}

function updateReconnectToken(): void {
  const token = auth.user?.access_token;

  if (token && socket) {
    socket.io.opts.query = { access_token: token };
  }
}

async function handleConnectError(error: Error): Promise<void> {
  const reason = error.message ?? String(error);

  if (refreshingToken || !/token|auth|unauthorized|forbidden/i.test(reason)) return;

  refreshingToken = true;

  try {
    const user = await auth.refresh();

    if (socket) {
      socket.io.opts.query = { access_token: user.access_token };
      socket.connect();
    }
  } catch {
    // The auth manager handles invalid sessions by clearing the stored user.
  } finally {
    refreshingToken = false;
  }
}

export async function connectSocket(): Promise<void> {
  if (socket?.connected || socket?.active) return;

  const token = await auth.getAccessToken();
  if (!token) return;

  disconnectSocket();

  socket = io({
    path: "/socket.io",
    transports: ["websocket", "polling"],
    query: { access_token: token },
    reconnection: true,
    reconnectionAttempts: Infinity,
  });

  socket.on("hello", (hello: HelloEvent) => {
    console.info(`Socket.IO connected as ${hello.user_id}`);
  });

  socket.on("event", (event: GatewayEvent) => {
    handleEvent(event);
  });

  socket.io.on("reconnect_attempt", updateReconnectToken);

  socket.on("connect_error", (error) => {
    void handleConnectError(error);
  });
}

export function disconnectSocket(): void {
  if (listInvalidateTimer) {
    clearTimeout(listInvalidateTimer);
    listInvalidateTimer = null;
  }

  if (socket) {
    socket.disconnect();
    socket = null;
  }
}
