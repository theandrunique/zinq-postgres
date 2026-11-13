import type { Chat } from "#lib/types";
import { isMetaMessage, renderMessageText } from "#lib/utils";

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

  const text = renderMessageText(
    {
      content: lastMessage.content,
      type: lastMessage.message_type.type,
      payload: lastMessage.message_type.payload,
      metadata: lastMessage.message_type.metadata,
    },
    chat.members,
  );

  if (chat.type !== "GROUP_DM") return text;

  if (isMetaMessage({ type: lastMessage.message_type.type })) return text;

  const author = chat.members.find((member) => member.user_id === lastMessage.author_id);

  if (lastMessage.author_id === currentUserId) return `Вы: ${text}`;
  if (!author) return text;

  return `${author.display_name || author.username}: ${text}`;
}
