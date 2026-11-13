import { io, type Socket } from "socket.io-client";
import type { ApiMessage, GatewayEvent, HelloEvent } from "#lib/types";
import { ackMessages } from "#lib/api/chats";
import {
  appendMessage,
  applyMessageAck,
  bumpUnread,
  invalidateChat,
  markChatRead,
  patchLastMessageIfCurrent,
  patchMessage,
  removeMessageEverywhere,
  updateLastMessage,
  upsertChat,
} from "#lib/api/cache";
import { queryClient } from "#lib/queries/query-client.ts";
import { chatKeys } from "#lib/api/query-keys";
import { auth } from "#lib/auth.svelte.ts";
import { compareIds } from "#lib/utils/id";
import { chatSession } from "#lib/state/chat-session.svelte";

const ACK_DEBOUNCE_MS = 500;

let socket: Socket | null = null;
let refreshingToken = false;
const ackTimers: Record<string, ReturnType<typeof setTimeout>> = {};
const ackInFlight: Record<string, true> = {};
const ackQueued: Record<string, string> = {};

function isPageVisible(): boolean {
  return typeof document === "undefined" || document.visibilityState === "visible";
}

function scheduleAck(chatId: string, messageId: string): void {
  const existing = ackTimers[chatId];
  if (existing) clearTimeout(existing);

  const timer = setTimeout(() => {
    delete ackTimers[chatId];
    void acknowledgeChat(chatId, messageId);
  }, ACK_DEBOUNCE_MS);

  ackTimers[chatId] = timer;
}

export async function acknowledgeChat(chatId: string, messageId: string): Promise<void> {
  const chat = queryClient.getQueryData<{ last_read_message_id: string | null }>(chatKeys.detail(chatId));

  if (chat?.last_read_message_id && compareIds(chat.last_read_message_id, messageId) >= 0) {
    return;
  }

  if (ackInFlight[chatId]) {
    const queued = ackQueued[chatId];

    if (!queued || compareIds(messageId, queued) > 0) {
      ackQueued[chatId] = messageId;
    }

    return;
  }

  ackInFlight[chatId] = true;

  try {
    await ackMessages(chatId, messageId);
    markChatRead(queryClient, chatId, messageId);
  } catch (error) {
    console.warn("Failed to acknowledge messages", error);
  } finally {
    delete ackInFlight[chatId];

    const queued = ackQueued[chatId];
    delete ackQueued[chatId];

    if (queued && compareIds(queued, messageId) > 0) {
      void acknowledgeChat(chatId, queued);
    }
  }
}

function handleMessageCreate(message: ApiMessage): void {
  const currentUserId = auth.user?.profile.id ?? "";

  appendMessage(queryClient, message);
  updateLastMessage(queryClient, message);

  if (message.author_id === currentUserId) return;

  if (chatSession.activeChatId === message.chat_id) {
    if (isPageVisible()) {
      scheduleAck(message.chat_id, message.id);
    } else {
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
  for (const timer of Object.values(ackTimers)) clearTimeout(timer);

  for (const chatId of Object.keys(ackTimers)) delete ackTimers[chatId];

  if (socket) {
    socket.disconnect();
    socket = null;
  }
}
