import { getUser, searchUsers } from "$lib/api/users";

import { userKeys } from "./query-keys";

export const USER_SEARCH_LIMIT = 20;

export function searchUsersQueryOptions(query: string) {
  const normalized = query.trim();

  return {
    queryKey: userKeys.search(normalized),
    queryFn: () => searchUsers(normalized, USER_SEARCH_LIMIT),
    enabled: normalized.length > 0,
    staleTime: 30_000,
  };
}

export function userQueryOptions(userId: string) {
  return {
    queryKey: userKeys.detail(userId),
    queryFn: () => getUser(userId),
    enabled: userId.length > 0,
  };
}
