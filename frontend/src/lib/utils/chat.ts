import type { Chat, ChatMember } from "#lib/types";

export function getActiveMembers(chat: Chat): ChatMember[] {
  return chat.members;
}

export function getOtherMember(chat: Chat, currentUserId: string): ChatMember | null {
  return chat.members.find((member) => member.user_id !== currentUserId) ?? null;
}

export function isSavedMessages(chat: Chat, currentUserId: string): boolean {
  if (chat.type !== "DM") return false;

  return chat.members.filter((member) => member.user_id !== currentUserId).length === 0;
}

export function getChatDisplayName(chat: Chat, currentUserId: string): string {
  if (chat.type === "DM") {
    if (isSavedMessages(chat, currentUserId)) return "Избранное";

    const other = getOtherMember(chat, currentUserId);
    return other?.display_name || other?.username || "Личный чат";
  }

  if (chat.name) return chat.name;

  const names = chat.members
    .filter((member) => member.user_id !== currentUserId)
    .map((member) => member.display_name || member.username);

  return names.length > 0 ? names.join(", ") : "Групповой чат";
}

export function getChatAvatar(chat: Chat, currentUserId: string): string | null {
  if (isSavedMessages(chat, currentUserId)) return "/bookmark.png";
  if (chat.image) return chat.image;

  if (chat.type === "DM") {
    return getOtherMember(chat, currentUserId)?.avatar ?? null;
  }

  return null;
}

export function getChatAvatarFallback(chat: Chat, currentUserId: string): string {
  const name = getChatDisplayName(chat, currentUserId).trim();

  return name.charAt(0).toUpperCase() || "?";
}
