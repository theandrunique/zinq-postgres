import type { Chat } from "$lib/types";
import { isMetaMessage, renderMessageText } from "../../model/messageHelpers";

export function formatTime(iso: string): string {
  const date = new Date(iso);
  const now = new Date();

  const isToday =
    date.getDate() === now.getDate() && date.getMonth() === now.getMonth() && date.getFullYear() === now.getFullYear();

  const startOfWeek = new Date(now.getTime());
  startOfWeek.setDate(now.getDate() - now.getDay());
  startOfWeek.setHours(0, 0, 0, 0);

  if (isToday) {
    return date.toLocaleTimeString("ru-RU", {
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  if (date >= startOfWeek) {
    return date.toLocaleDateString("ru-RU", { weekday: "short" });
  }

  return date.toLocaleDateString("ru-RU");
}

export function getMessagePreview(chat: Chat, currentUserId: string): string | null {
  const lastMessage = chat.last_message;
  if (!lastMessage) return null;

  const text = renderMessageText({
    content: lastMessage.content,
    type: lastMessage.message_type.type,
    payload: lastMessage.message_type.payload,
    metadata: lastMessage.message_type.metadata,
  });

  if (chat.type !== "GROUP_DM") return text;

  if (isMetaMessage({ type: lastMessage.message_type.type })) return text;

  if (lastMessage.author_id === currentUserId) return `Вы: ${text}`;

  const authorName = lastMessage.author?.display_name || lastMessage.author?.username;

  return authorName ? `${authorName}: ${text}` : text;
}
