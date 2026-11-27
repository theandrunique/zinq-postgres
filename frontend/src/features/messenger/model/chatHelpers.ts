import type { Chat, ChatMember } from "$lib/types";

export function getOtherMember(chat: Chat, currentUserId: string): ChatMember | null {
  return chat.members.find((member) => member.user_id !== currentUserId) ?? null;
}

export function isSavedMessages(chat: Chat, currentUserId: string): boolean {
  if (chat.type !== "DM") return false;

  return chat.members.filter((member) => member.user_id !== currentUserId).length === 0;
}

export function getChatDisplayName(chat: Chat, currentUserId: string): string {
  if (chat.type === "DM") {
    if (isSavedMessages(chat, currentUserId)) return "Saved messages";

    const other = getOtherMember(chat, currentUserId);
    return other?.display_name || other?.username || "Direct chat";
  }

  if (chat.name) return chat.name;

  const names = chat.members
    .filter((member) => member.user_id !== currentUserId)
    .map((member) => member.display_name || member.username);

  return names.length > 0 ? names.join(", ") : "Group chat";
}

export function getChatAvatar(chat: Chat, currentUserId: string): string | null {
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
