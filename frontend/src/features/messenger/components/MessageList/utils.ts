import type { Message } from "#lib/types";
import { isMetaMessage } from "#lib/utils/message";

export { isMetaMessage };

export function shouldGroup(curr: Message, prev: Message): boolean {
  if (isMetaMessage(curr) || isMetaMessage(prev)) return false;
  if (curr.author_id !== prev.author_id) return false;

  const diff = new Date(curr.created_at).getTime() - new Date(prev.created_at).getTime();
  return diff < 5 * 60 * 1000;
}

export function groupMessagesByAuthor(messages: Message[]): Message[][] {
  if (messages.length === 0) return [];

  const groups: Message[][] = [];
  let currentGroup = [messages[0]];

  for (let i = 1; i < messages.length; i++) {
    if (shouldGroup(messages[i], messages[i - 1])) {
      currentGroup.push(messages[i]);
    } else {
      groups.push(currentGroup);
      currentGroup = [messages[i]];
    }
  }

  groups.push(currentGroup);

  return groups;
}

export function getDateKey(iso: string): string {
  const date = new Date(iso);

  return `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()}`;
}

export function groupMessagesByDate(messages: Message[]): [string, Message[]][] {
  const map = new Map<string, Message[]>();

  for (const message of messages) {
    const key = getDateKey(message.created_at);
    const group = map.get(key) ?? [];

    group.push(message);
    map.set(key, group);
  }

  return Array.from(map.entries());
}

export function formatMessageTime(iso: string): string {
  return new Date(iso).toLocaleTimeString("ru-RU", {
    hour: "2-digit",
    minute: "2-digit",
  });
}

export function formatDateSeparator(iso: string): string {
  const date = new Date(iso);
  const now = new Date();

  if (date.toDateString() === now.toDateString()) return "Сегодня";

  const yesterday = new Date(now);
  yesterday.setDate(yesterday.getDate() - 1);

  if (date.toDateString() === yesterday.toDateString()) return "Вчера";

  return date.toLocaleDateString("ru-RU", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}
