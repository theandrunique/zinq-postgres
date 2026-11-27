<script lang="ts">
  import { page } from "$app/state";
  import { connectSocket, disconnectSocket } from "$services/socket.svelte.ts";
  import { cn } from "$lib/utils";
  import type { LayoutProps } from "./$types";
  import { ChatSidebar } from "$features/messenger";
  import { ChatCreateModal } from "$features/chat-create";
  import { UserProfileDialog } from "$features/user-profile";
  import { UserSearchDialog } from "$features/user-search";

  let { children }: LayoutProps = $props();

  const hasChat = $derived(Boolean(page.params.chat_id));

  $effect(() => {
    void connectSocket();

    return () => disconnectSocket();
  });
</script>

<div class="flex h-screen w-full overflow-hidden bg-(--color-background-body) text-(--color-text-base)">
  <ChatSidebar class={hasChat ? "hidden md:flex" : "flex"} />

  <main class={cn("min-w-0 flex-1 overflow-hidden", !hasChat && "hidden md:block")}>
    {@render children()}
  </main>

  <ChatCreateModal />
  <UserSearchDialog />
  <UserProfileDialog />
</div>
