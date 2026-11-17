import type { ApiMessage, Message, MessageAuthor, MessageTypeDescriptor } from "$lib/types";

export const META_MESSAGE_TYPES = new Set([
  "MEMBER_ADD",
  "MEMBER_REMOVE",
  "MEMBER_LEAVE",
  "CHAT_NAME_UPDATE",
  "CHAT_IMAGE_UPDATE",
  "CHAT_CREATE",
  "CHAT_PINNED_MESSAGE",
  "CHAT_UNPIN_MESSAGE",
]);

export interface RenderableMessage {
  content: string;
  type: string;
  payload?: Record<string, unknown> | null;
  metadata?: Record<string, unknown> | null;
}

export function isMetaMessage(message: { type: string }): boolean {
  return META_MESSAGE_TYPES.has(message.type.toUpperCase());
}

/**
 * Attachments and avatars store an S3 object key, not a public URL. Until a
 * presigned download endpoint exists we can only render absolute URLs.
 */
export function getAttachmentUrl(attachment: { storage_key: string }): string | null {
  return /^https?:\/\//i.test(attachment.storage_key) ? attachment.storage_key : null;
}

export function messageTypeName(value: unknown): string {
  if (typeof value === "string") return value.toUpperCase();

  if (value && typeof value === "object") {
    const descriptor = value as MessageTypeDescriptor;
    if (typeof descriptor.type === "string") return descriptor.type.toUpperCase();
  }

  return "DEFAULT";
}

export function messageMetadata(value: unknown): Record<string, unknown> {
  if (!value || typeof value !== "object") return {};

  const descriptor = value as MessageTypeDescriptor;
  const source = descriptor.payload ?? descriptor.metadata;

  return source && typeof source === "object" ? (source as Record<string, unknown>) : {};
}

function authorName(author: Partial<MessageAuthor> | null | undefined): string | null {
  if (!author) return null;

  return author.display_name || author.username || null;
}

function referencedUserName(value: unknown): string | null {
  if (!value || typeof value !== "object") return null;

  return authorName(value as Partial<MessageAuthor>);
}

function attachmentPreview(message: RenderableMessage & { attachments?: unknown }): string {
  const attachments = message.attachments;

  if (Array.isArray(attachments) && attachments.length > 0) {
    const first = attachments[0] as { filename?: unknown } | undefined;
    const filename = typeof first?.filename === "string" ? first.filename : null;

    if (attachments.length === 1) return filename ?? "Вложение";
    return `${filename ?? "Вложение"} и ещё ${attachments.length - 1}`;
  }

  return "";
}

/**
 * Turns any backend message representation into a human readable one-line text.
 * The user a meta message refers to is embedded into its payload by the backend.
 */
export function renderMessageText(message: RenderableMessage & { attachments?: unknown }): string {
  const type = messageTypeName(message.type);
  const metadata = messageMetadata(message);
  const content = message.content ?? "";

  switch (type) {
    case "DEFAULT":
    case "REPLY":
    case "FORWARD":
      return content || attachmentPreview(message) || "Вложение";

    case "MEMBER_ADD": {
      const name = referencedUserName(metadata.user);
      return name ? `${name} добавлен(а) в чат` : "Участник добавлен в чат";
    }

    case "MEMBER_REMOVE": {
      const name = referencedUserName(metadata.user);
      return name ? `${name} удалён(а) из чата` : "Участник удалён из чата";
    }

    case "MEMBER_LEAVE": {
      const name = referencedUserName(metadata.user);
      return name ? `${name} покинул(а) чат` : "Участник покинул чат";
    }

    case "CHAT_NAME_UPDATE": {
      const name = typeof metadata.new_name === "string" ? metadata.new_name : null;
      return name ? `Название чата изменено на «${name}»` : "Название чата изменено";
    }

    case "CHAT_IMAGE_UPDATE":
      return "Изображение чата обновлено";

    case "CHAT_CREATE": {
      const name = typeof metadata.chat_name === "string" ? metadata.chat_name : null;
      return name ? `Чат «${name}» создан` : "Чат создан";
    }

    case "CHAT_PINNED_MESSAGE":
      return "Сообщение закреплено";

    case "CHAT_UNPIN_MESSAGE":
      return "Сообщение откреплено";

    default:
      return content || attachmentPreview(message) || "Неподдерживаемое сообщение";
  }
}

export function normalizeMessages(dtos: ApiMessage[]): Message[] {
  const normalized: Message[] = dtos.map((dto) => {
    const { type, ...rest } = dto;

    return {
      ...rest,
      type: messageTypeName(type),
      metadata: messageMetadata(type),
      referenced_message: null,
    };
  });

  const byId = new Map(normalized.map((message) => [message.id, message]));

  for (const message of normalized) {
    if (message.type !== "REPLY") continue;

    const referencedId = message.metadata.referenced_message_id;
    if (typeof referencedId === "string") {
      message.referenced_message = byId.get(referencedId) ?? null;
    }
  }

  return normalized;
}

export function normalizeMessage(dto: ApiMessage): Message {
  return normalizeMessages([dto])[0];
}
