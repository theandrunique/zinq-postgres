<script lang="ts">
  import { createMutation, useQueryClient } from "@tanstack/svelte-query";
  import { X } from "@lucide/svelte";
  import { Avatar, Button, Dialog, Input } from "$lib/components/ui";
  import { createChat } from "$lib/api/chats";
  import { ApiError } from "$lib/api/client";
  import { auth } from "$lib/auth.svelte.ts";
  import { chatKeys } from "$lib/queries/messenger/query-keys.ts";
  import type { PublicUser } from "$lib/types";
  import { chatCreateModal } from "$services/chatCreateModal.svelte.ts";
  import { UserSearchInput } from "$features/user-search";

  const queryClient = useQueryClient();

  const currentUser = $derived(auth.user?.profile ?? null);
  const currentUserId = $derived(currentUser?.id ?? "");

  const currentUserInitials = $derived(
    (currentUser?.display_name || currentUser?.username || "")
      .trim()
      .split(/\s+/)
      .filter(Boolean)
      .slice(0, 2)
      .map((part) => part[0]?.toUpperCase() ?? "")
      .join("") || "?",
  );

  const excludeIds = $derived([
    ...(currentUserId ? [currentUserId] : []),
    ...chatCreateModal.members.map((member) => member.id),
  ]);

  let error = $state<string | null>(null);

  const mutation = createMutation(() => ({
    mutationFn: (variables: { name: string; members: string[] }) => createChat(variables),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: chatKeys.list() });
      chatCreateModal.reset();
      chatCreateModal.close();
    },
  }));

  function handleSelect(user: PublicUser) {
    chatCreateModal.addMember(user);
  }

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();

    if (mutation.isPending) return;

    error = null;

    const trimmedName = chatCreateModal.name.trim();

    if (!trimmedName) {
      error = "Enter a chat name.";
      return;
    }

    try {
      await mutation.mutateAsync({
        name: trimmedName,
        members: chatCreateModal.members.map((member) => member.id),
      });
    } catch (cause) {
      error = cause instanceof ApiError ? cause.message : "Couldn't create the chat.";
    }
  }
</script>

<Dialog.Root bind:open={() => chatCreateModal.isOpen, () => chatCreateModal.close()}>
  <Dialog.Portal>
    <Dialog.Overlay />
    <Dialog.Content>
      <Dialog.Title>New chat</Dialog.Title>

      <form class="flex flex-col gap-(--space-1)" onsubmit={handleSubmit}>
        <div>
          <label
            for="create-chat-name"
            class="mb-(--space-05) block text-(length:--font-size-sm) font-medium text-(--color-text-base)"
          >
            Name
          </label>
          <Input
            id="create-chat-name"
            bind:value={chatCreateModal.name}
            placeholder="For example, Project Zinq"
            autocomplete="off"
          />
        </div>

        <div>
          <label
            for="create-chat-members"
            class="mb-(--space-05) block text-(length:--font-size-sm) font-medium text-(--color-text-base)"
          >
            Members
          </label>

          <div class="mb-(--space-05) flex flex-wrap gap-(--space-05)">
            {#if currentUser}
              <span
                class="inline-flex items-center gap-1.5 rounded-(--radius-full) border border-(color:--color-border-base) bg-(--color-background-interactive) py-0.5 pr-2 pl-0.5 text-(length:--font-size-sm)"
              >
                <Avatar
                  src={currentUser.avatar}
                  alt={currentUser.display_name}
                  fallback={currentUserInitials}
                  class="h-6 w-6 shrink-0"
                />
                <span class="max-w-40 truncate">{currentUser.display_name}</span>
                <span class="text-(length:--font-size-xs) text-(--color-text-subtle)">You</span>
              </span>
            {/if}

            {#each chatCreateModal.members as member (member.id)}
              <span
                class="inline-flex items-center gap-1.5 rounded-(--radius-full) border border-(color:--color-border-base) bg-(--color-background-interactive) py-0.5 pr-1 pl-0.5 text-(length:--font-size-sm)"
              >
                <Avatar
                  src={member.avatar}
                  alt={member.display_name}
                  fallback={member.display_name.charAt(0).toUpperCase() || "?"}
                  class="h-6 w-6 shrink-0"
                />
                <span class="max-w-40 truncate">{member.display_name}</span>
                <button
                  type="button"
                  class="rounded-(--radius-full) p-0.5 text-(--color-text-muted) hover:bg-(--color-background-interactive-hover) hover:text-(--color-text-base)"
                  title={`Remove ${member.display_name}`}
                  onclick={() => chatCreateModal.removeMember(member.id)}
                >
                  <X class="h-3.5 w-3.5" />
                </button>
              </span>
            {/each}
          </div>

          <UserSearchInput
            id="create-chat-members"
            bind:query={chatCreateModal.query}
            {excludeIds}
            placeholder="Search by username…"
            onSelect={handleSelect}
          />

          <p class="mt-(--space-05) text-(length:--font-size-xs) text-(--color-text-subtle)">
            You are added automatically. Searching is optional.
          </p>
        </div>

        {#if error}
          <p
            role="alert"
            class="rounded-(--radius-md) border-(length:--border-width-default) border-(color:--color-border-error) bg-(--color-background-error-subtle) px-(--space-1) py-(--space-05) text-(length:--font-size-sm) text-(--color-text-error)"
          >
            {error}
          </p>
        {/if}

        <div class="mt-(--space-1) flex justify-end gap-(--space-1)">
          <Button type="button" variant="secondary" onclick={() => chatCreateModal.close()}>Cancel</Button>
          <Button type="submit" disabled={mutation.isPending}>
            {mutation.isPending ? "Creating…" : "Create"}
          </Button>
        </div>
      </form>

      <Dialog.Close />
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
