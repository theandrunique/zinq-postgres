export const userKeys = {
  all: ["users"] as const,
  search: (query: string) => [...userKeys.all, "search", query] as const,
  detail: (userId: string) => [...userKeys.all, "detail", userId] as const,
};
