let activeChatId = $state<string | null>(null);

export const chatSession = {
  get activeChatId(): string | null {
    return activeChatId;
  },
  setActiveChat(chatId: string | null): void {
    activeChatId = chatId;
  },
};
