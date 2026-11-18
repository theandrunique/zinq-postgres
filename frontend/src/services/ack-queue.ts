import type { Chat } from "$lib/types";
import { ackMessages } from "$lib/api/chats";
import { markChatRead } from "$lib/queries/messenger/cache.ts";
import { chatKeys } from "$lib/queries/messenger/query-keys.ts";
import { queryClient } from "$lib/queries/query-client.ts";
import { compareIds, maxId } from "$lib/utils/id";

const ACK_DEBOUNCE_MS = 300;
const ACK_RETRY_DELAY_MS = 2_000;
const ACK_MAX_RETRIES = 3;
const ACK_MAX_BATCH = 100;

interface ChatAckState {
  /** Messages that were fully visible and still need an individual ack. */
  pending: Set<string>;
  /** Messages already acknowledged during this session. */
  sent: Set<string>;
  /** Read pointer to advance once the newest message is visible. */
  pointer: string | null;
  timer: ReturnType<typeof setTimeout> | null;
  inFlight: boolean;
  retries: number;
}

const states = new Map<string, ChatAckState>();
let flushListenersInstalled = false;

function getState(chatId: string): ChatAckState {
  let state = states.get(chatId);

  if (!state) {
    state = {
      pending: new Set(),
      sent: new Set(),
      pointer: null,
      timer: null,
      inFlight: false,
      retries: 0,
    };

    states.set(chatId, state);
  }

  installFlushListeners();

  return state;
}

function currentLastRead(chatId: string): string | null {
  const chat = queryClient.getQueryData<Chat>(chatKeys.detail(chatId));

  return chat?.last_read_message_id ?? null;
}

function isNewer(messageId: string, baseline: string | null): boolean {
  return !baseline || compareIds(messageId, baseline) > 0;
}

function installFlushListeners(): void {
  if (flushListenersInstalled || typeof window === "undefined") return;

  flushListenersInstalled = true;

  const flushAll = () => {
    for (const chatId of states.keys()) {
      void flushChatAcks(chatId);
    }
  };

  document.addEventListener("visibilitychange", () => {
    if (document.visibilityState === "hidden") flushAll();
  });

  window.addEventListener("pagehide", flushAll);
}

function scheduleFlush(chatId: string): void {
  const state = getState(chatId);

  if (state.timer) clearTimeout(state.timer);

  state.timer = setTimeout(() => {
    state.timer = null;
    void flushChatAcks(chatId);
  }, ACK_DEBOUNCE_MS);
}

/** A message was fully visible for long enough to be considered read. */
export function ackMessageVisible(chatId: string, messageId: string): void {
  if (!isNewer(messageId, currentLastRead(chatId))) return;

  const state = getState(chatId);

  if (state.pending.has(messageId) || state.sent.has(messageId)) return;

  state.pending.add(messageId);

  if (state.pending.size >= ACK_MAX_BATCH) {
    void flushChatAcks(chatId);
    return;
  }

  scheduleFlush(chatId);
}

/**
 * The newest message is visible, so everything above it is read and the read
 * pointer can jump forward. The backend fills individual acks for the range.
 */
export function ackReachedBottom(chatId: string, newestMessageId: string): void {
  if (!isNewer(newestMessageId, currentLastRead(chatId))) return;

  const state = getState(chatId);

  state.pointer = maxId([state.pointer, newestMessageId]);
  void flushChatAcks(chatId);
}

export async function flushChatAcks(chatId: string): Promise<void> {
  const state = states.get(chatId);

  if (!state || state.inFlight) return;

  const lastRead = currentLastRead(chatId);
  const pointer = state.pointer && isNewer(state.pointer, lastRead) ? state.pointer : null;
  const acks = [...state.pending].filter((messageId) => isNewer(messageId, lastRead));

  state.pointer = null;
  state.pending.clear();

  if (!pointer && acks.length === 0) return;

  // The pointer covers the whole range, individual acks are redundant.
  const batch = pointer ? [] : acks.slice(0, ACK_MAX_BATCH);
  const rest = acks.slice(batch.length);

  if (state.timer) {
    clearTimeout(state.timer);
    state.timer = null;
  }

  state.inFlight = true;

  try {
    await ackMessages(chatId, {
      lastReadMessageId: pointer ?? undefined,
      acks: batch,
    });

    state.retries = 0;

    for (const messageId of batch) state.sent.add(messageId);

    for (const messageId of rest) state.pending.add(messageId);

    if (pointer) {
      markChatRead(queryClient, chatId, pointer);

      // Everything up to the pointer is covered by the read pointer.
      for (const messageId of state.sent) {
        if (compareIds(messageId, pointer) <= 0) state.sent.delete(messageId);
      }
    }
  } catch (error) {
    console.warn("Failed to acknowledge messages", error);

    if (state.retries < ACK_MAX_RETRIES) {
      state.retries += 1;

      for (const messageId of acks) state.pending.add(messageId);
      state.pointer = maxId([state.pointer, pointer]);

      state.timer = setTimeout(() => {
        state.timer = null;
        void flushChatAcks(chatId);
      }, ACK_RETRY_DELAY_MS);
    }
  } finally {
    state.inFlight = false;

    if (!state.timer && (state.pointer || state.pending.size > 0)) {
      scheduleFlush(chatId);
    }
  }
}
