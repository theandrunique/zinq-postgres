export type ChatType = "DM" | "GROUP_DM";

export interface ChatMember {
  user_id: string;
  username: string;
  display_name: string;
  avatar: string | null;
  permissions: string | null;
}

/**
 * The backend serializes message types differently in different places:
 * - `MessageSchema.type` is a plain string, with the variant data in `payload`;
 * - `LastMessageInfoSchema.message_type` is the domain enum, with data in `metadata`.
 */
export interface MessageTypeDescriptor {
  type: string;
  payload?: Record<string, unknown> | null;
  metadata?: Record<string, unknown> | null;
}

export interface LastMessageInfo {
  message_id: string;
  author_id: string;
  content: string;
  message_type: MessageTypeDescriptor;
  edited_at: string | null;
  created_at: string;
}

export interface Chat {
  id: string;
  owner_id: string | null;
  name: string | null;
  description: string | null;
  image: string | null;
  type: ChatType;
  permissions: string;
  created_at: string;
  members: ChatMember[];
  last_message: LastMessageInfo | null;
  last_read_message_id: string | null;
  max_read_message_id: string | null;
}

export interface ChatsPage {
  chats: Chat[];
  next_cursor: string | null;
  has_more: boolean;
}

export interface Attachment {
  id: string;
  message_id: string;
  chat_id: string;
  filename: string;
  content_type: string;
  size: number;
  storage_key: string;
  created_at: string;
}

export interface ApiMessage {
  id: string;
  chat_id: string;
  author_id: string;
  content: string;
  created_at: string;
  edited_at: string | null;
  type: MessageTypeDescriptor;
  attachments: Attachment[];
}

export interface Message extends Omit<ApiMessage, "type"> {
  type: string;
  metadata: Record<string, unknown>;
  author: ChatMember | null;
  referenced_message: Message | null;
}

export interface MessageAck {
  user_id: string;
  timestamp: string;
}

export interface MessageAcksResponse {
  acks: MessageAck[];
}

export interface MessagesCountResponse {
  count: number;
}

export interface CreateChatRequest {
  name: string;
  members: string[];
}

export interface SendMessageRequest {
  content: string;
  referenced_message_id?: string;
  attachments?: { uploaded_filename: string; filename: string }[];
}

export type GatewayEvent =
  | { type: "MessageCreate"; payload: { message: ApiMessage } }
  | { type: "MessageUpdate"; payload: { message: ApiMessage } }
  | { type: "MessageDelete"; payload: { message_id: string } }
  | { type: "ChatCreate"; payload: { chat: Chat } }
  | { type: "ChatMemberAdd"; payload: { chat_id: string; member: ChatMember } }
  | { type: "ChatMemberRemove"; payload: { chat_id: string; member: ChatMember } }
  | {
      type: "MessageAck";
      payload: { user_id: string; chat_id: string; last_read_message_id: string };
    };

export interface HelloEvent {
  user_id: string;
  session_id: string;
}
