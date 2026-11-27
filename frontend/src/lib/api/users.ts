import type { Chat, PublicUser } from "$lib/types";

import { api } from "./client";

export async function searchUsers(query: string, limit = 20): Promise<PublicUser[]> {
  const { data } = await api.get<PublicUser[]>("/users/search", {
    params: { q: query, limit },
  });

  return data;
}

export async function getUser(userId: string): Promise<PublicUser> {
  const { data } = await api.get<PublicUser>(`/users/${userId}`);
  return data;
}

export async function getDmChat(userId: string): Promise<Chat> {
  const { data } = await api.get<Chat>(`/users/@me/dms/${userId}`);
  return data;
}
