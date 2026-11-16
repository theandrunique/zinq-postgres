export const chatKeys = {
  all: ["chats"] as const,
  list: () => [...chatKeys.all, "list"] as const,
  detail: (chatId: string) => [...chatKeys.all, "detail", chatId] as const,
  messages: (chatId: string) => [...chatKeys.all, "messages", chatId] as const,
  unread: (chatId: string) => [...chatKeys.all, "unread", chatId] as const,
  acks: (chatId: string, messageId: string) => [...chatKeys.all, "acks", chatId, messageId] as const,
};
