<script lang="ts">
  import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { MessageCircle } from "@lucide/svelte";
  import { getDmChat } from "$lib/api/users";
  import { ApiError } from "$lib/api/client";
  import { auth } from "$lib/auth.svelte.ts";
  import { Avatar, Button, Dialog, Loader } from "$lib/components/ui";
  import { upsertChat } from "$lib/queries/messenger/cache.ts";
  import { chatKeys } from "$lib/queries/messenger/query-keys.ts";
  import { userQueryOptions } from "$lib/queries/users/queries.ts";
  import { userProfileModal } from "$services/userProfileModal.svelte.ts";

  const queryClient = useQueryClient();
  const currentUserId = $derived(auth.user?.profile.id ?? "");
  const userId = $derived(userProfileModal.userId ?? "");

  const userQuery = createQuery(() => ({
    ...userQueryOptions(userId),
    enabled: userProfileModal.isOpen && userId.length > 0,
  }));

  let actionError = $state<string | null>(null);

  const dmMutation = createMutation(() => ({
    mutationFn: (targetId: string) => getDmChat(targetId),
    onSuccess: (chat) => {
      upsertChat(queryClient, chat);
      queryClient.setQueryData(chatKeys.detail(chat.id), chat);
      userProfileModal.close();
      void goto(resolve("/chats/[chat_id]", { chat_id: chat.id }));
    },
  }));

  let user = $derived(userQuery.data ?? null);
  let isSelf = $derived(user !== null && user.id === currentUserId);

  let initials = $derived(
    (user?.display_name || user?.username || "")
      .trim()
      .split(/\s+/)
      .filter(Boolean)
      .slice(0, 2)
      .map((part) => part[0]?.toUpperCase() ?? "")
      .join("") || "?",
  );

  let memberSince = $derived(
    user
      ? new Date(user.created_at).toLocaleDateString("en-US", {
          month: "long",
          day: "numeric",
          year: "numeric",
        })
      : "",
  );

  function reset() {
    actionError = null;
    dmMutation.reset();
  }

  async function startChat() {
    if (!user || dmMutation.isPending) return;

    actionError = null;

    try {
      await dmMutation.mutateAsync(user.id);
    } catch (cause) {
      actionError = cause instanceof ApiError ? cause.message : "Couldn't start the chat.";
    }
  }
</script>

<Dialog.Root
  bind:open={() => userProfileModal.isOpen, () => userProfileModal.close()}
  onOpenChange={(value) => !value && reset()}
>
  <Dialog.Portal>
    <Dialog.Overlay />
    <Dialog.Content class="sm:max-w-md">
      <Dialog.Title>Profile</Dialog.Title>

      {#if userQuery.isPending}
        <div class="flex items-center gap-2 py-(--space-2)">
          <Loader size="sm" />
          <span class="text-(length:--font-size-sm) text-(--color-text-muted)">Loading profile…</span>
        </div>
      {:else if userQuery.isError}
        <div class="flex flex-col items-start gap-(--space-1) py-(--space-1)">
          <p class="text-(length:--font-size-sm) text-(--color-text-muted)">Couldn't load the profile.</p>
          <Button variant="secondary" onclick={() => userQuery.refetch()}>Retry</Button>
        </div>
      {:else if user}
        <div class="mt-(--space-1) flex flex-col gap-(--space-2)">
          <div class="flex items-center gap-(--space-2)">
            <Avatar src={user.avatar} alt={user.display_name} fallback={initials} class="h-16 w-16 shrink-0" />

            <div class="min-w-0">
              <p class="truncate text-(length:--font-size-lg) font-semibold text-(--color-text-base)">
                {user.display_name}
              </p>
              <p class="truncate text-(length:--font-size-sm) text-(--color-text-brand)">@{user.username}</p>
            </div>
          </div>

          <div>
            <p class="text-(length:--font-size-xs) font-medium tracking-wide text-(--color-text-subtle) uppercase">
              About
            </p>
            <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-base)">
              {user.bio || "No bio yet."}
            </p>
          </div>

          <p class="text-(length:--font-size-xs) text-(--color-text-subtle)">Member since {memberSince}</p>

          {#if actionError}
            <p
              role="alert"
              class="rounded-(--radius-md) border-(length:--border-width-default) border-(color:--color-border-error) bg-(--color-background-error-subtle) px-(--space-1) py-(--space-05) text-(length:--font-size-sm) text-(--color-text-error)"
            >
              {actionError}
            </p>
          {/if}

          <Button class="w-full" disabled={dmMutation.isPending} onclick={startChat}>
            <MessageCircle class="h-4 w-4" />
            {#if dmMutation.isPending}
              Opening…
            {:else if isSelf}
              Open Saved Messages
            {:else}
              Send message
            {/if}
          </Button>
        </div>
      {/if}

      <Dialog.Close />
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
