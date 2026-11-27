import { api } from "./client";
import type {
  ApiMessage,
  Chat,
  ChatsPage,
  CreateChatRequest,
  MessageAcksResponse,
  MessagesCountResponse,
  SendMessageRequest,
} from "$lib/types";

export interface GetChatsParams {
  cursor?: string | null;
  limit?: number;
}

export interface GetMessagesParams {
  before?: string | null;
  limit?: number;
}

export async function getChats(params: GetChatsParams = {}): Promise<ChatsPage> {
  const { data } = await api.get<ChatsPage>("/users/@me/chats", {
    params: {
      cursor: params.cursor ?? undefined,
      limit: params.limit ?? 20,
    },
  });

  return data;
}

export async function getChat(chatId: string): Promise<Chat> {
  const { data } = await api.get<Chat>(`/chats/${chatId}`);
  return data;
}

export async function getMessages(chatId: string, params: GetMessagesParams = {}): Promise<ApiMessage[]> {
  const { data } = await api.get<ApiMessage[]>(`/chats/${chatId}/messages`, {
    params: {
      before: params.before ?? undefined,
      limit: params.limit ?? 50,
    },
  });

  return data;
}

export async function sendMessage(chatId: string, request: SendMessageRequest): Promise<ApiMessage> {
  const { data } = await api.post<ApiMessage>(`/chats/${chatId}/messages`, request);
  return data;
}

export interface AckMessageAck {
  messageId: string;
  /** When the message was actually seen, ISO 8601. */
  ackedAt: string;
}

export interface AckMessagesParams {
  /** Moves the read pointer forward; omitted for per-message acks. */
  lastReadMessageId?: string;
  /** Individual messages seen in the viewport. */
  acks?: AckMessageAck[];
}

export async function ackMessages(chatId: string, params: AckMessagesParams = {}): Promise<void> {
  await api.put(`/chats/${chatId}/messages/acks`, {
    last_read_message_id: params.lastReadMessageId,
    acks: (params.acks ?? []).map(({ messageId, ackedAt }) => ({
      message_id: messageId,
      acked_at: ackedAt,
    })),
  });
}

export async function getMessageAcks(chatId: string, messageId: string): Promise<MessageAcksResponse> {
  const { data } = await api.get<MessageAcksResponse>(`/chats/${chatId}/messages/${messageId}/acks`);

  return data;
}

export async function countMessages(
  chatId: string,
  from?: string | null,
  to?: string | null,
): Promise<MessagesCountResponse> {
  const { data } = await api.get<MessagesCountResponse>(`/chats/${chatId}/messages/count`, {
    params: {
      from: from ?? undefined,
      to: to ?? undefined,
    },
  });

  return data;
}

export async function createChat(request: CreateChatRequest): Promise<void> {
  await api.post("/users/@me/chats", request);
}
