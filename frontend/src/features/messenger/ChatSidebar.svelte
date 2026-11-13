<script lang="ts">
  import { createInfiniteQuery } from "@tanstack/svelte-query";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import { Plus, Users } from "@lucide/svelte";
  import { Avatar, Button, DropdownMenu, Loader } from "#lib/components/ui";
  import { chatsQueryOptions } from "#lib/api/queries";
  import { auth } from "#lib/auth.svelte.ts";
  import { cn } from "#lib/utils";
  import ChatListItem from "./ChatListItem.svelte";
  import { chatCreateModal } from "../../services/messenger/chatCreateModal.svelte";

  interface Props {
    class?: string;
  }

  let { class: className = "" }: Props = $props();

  const currentUserId = $derived(auth.user?.profile.id ?? "");
  const profile = $derived(auth.user?.profile ?? null);
  const initials = $derived(
    (profile?.display_name || profile?.username || "?")
      .trim()
      .split(/\s+/)
      .filter(Boolean)
      .slice(0, 2)
      .map((part) => part[0]?.toUpperCase() ?? "")
      .join("") || "?",
  );

  const chatsQuery = createInfiniteQuery(() => chatsQueryOptions());
  const chats = $derived(chatsQuery.data ?? []);
  const activeChatId = $derived(page.params.chat_id ?? null);

  let sentinel = $state<HTMLDivElement | null>(null);

  $effect(() => {
    if (!sentinel || !chatsQuery.hasNextPage || chatsQuery.isFetchingNextPage) return;

    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry?.isIntersecting) void chatsQuery.fetchNextPage();
      },
      { rootMargin: "160px" },
    );

    observer.observe(sentinel);

    return () => observer.disconnect();
  });

  function openChat(chatId: string) {
    void goto(resolve("/chats/[chat_id]", { chat_id: chatId }));
  }

  async function handleSignout() {
    await auth.signout();
    void goto(resolve("/login"));
  }
</script>

<aside
  class={cn(
    "relative h-full w-full shrink-0 flex-col border-r border-(--color-border-base) bg-(--color-background-surface) md:flex md:w-80",
    className,
  )}
>
  <header
    class="flex shrink-0 items-center justify-between gap-(--space-1) border-b border-(--color-border-base) px-(--space-2) py-(--space-1)"
  >
    <h1 class="text-(length:--font-size-lg) font-semibold">Чаты</h1>

    {#if profile}
      <DropdownMenu.Root>
        <DropdownMenu.Trigger>
          {#snippet child({ props })}
            <button
              {...props}
              type="button"
              class="rounded-(--avatar-radius) focus-visible:focus-ring"
              aria-label="Меню пользователя"
            >
              <Avatar src={profile.avatar} alt={profile.display_name} fallback={initials} class="h-8 w-8" />
            </button>
          {/snippet}
        </DropdownMenu.Trigger>
        <DropdownMenu.Portal>
          <DropdownMenu.Content align="end">
            <DropdownMenu.Item onSelect={() => goto(resolve("/profile"))}>Профиль</DropdownMenu.Item>
            <DropdownMenu.Item onSelect={handleSignout}>Выйти</DropdownMenu.Item>
          </DropdownMenu.Content>
        </DropdownMenu.Portal>
      </DropdownMenu.Root>
    {/if}
  </header>

  <div class="min-h-0 flex-1 overflow-y-auto">
    {#if chatsQuery.isPending}
      <div class="flex items-center justify-center gap-(--space-1) py-(--space-4)">
        <Loader size="sm" />
        <span class="text-(length:--font-size-sm) text-(--color-text-muted)">Загрузка чатов…</span>
      </div>
    {:else if chatsQuery.isError}
      <div class="flex flex-col items-center gap-(--space-1) px-(--space-2) py-(--space-4) text-center">
        <p class="text-(length:--font-size-sm) text-(--color-text-muted)">Не удалось загрузить чаты</p>
        <Button variant="secondary" onclick={() => chatsQuery.refetch()}>Повторить</Button>
      </div>
    {:else if chats.length === 0}
      <div class="flex flex-col items-center gap-(--space-1) px-(--space-2) py-(--space-4) text-center">
        <Users class="h-8 w-8 text-(--color-text-subtle)" />
        <p class="text-(length:--font-size-sm) text-(--color-text-muted)">Пока нет ни одного чата</p>
        <p class="text-(length:--font-size-xs) text-(--color-text-subtle)">Создайте групповой чат кнопкой ниже</p>
      </div>
    {:else}
      {#each chats as chat (chat.id)}
        <ChatListItem {chat} {currentUserId} selected={chat.id === activeChatId} onclick={() => openChat(chat.id)} />
      {/each}
    {/if}

    <div bind:this={sentinel} class="h-px w-full"></div>

    {#if chatsQuery.isFetchingNextPage}
      <div class="flex justify-center py-(--space-2)">
        <Loader size="sm" />
      </div>
    {/if}
  </div>

  <Button
    class="absolute right-(--space-2) bottom-(--space-2) h-12 w-12 rounded-(--radius-full) shadow-(--shadow-elevation-3)"
    onclick={() => chatCreateModal.open()}
  >
    <Plus class="size-6" />
  </Button>
</aside>
