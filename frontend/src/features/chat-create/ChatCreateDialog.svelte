<script lang="ts">
  import { createMutation, useQueryClient } from "@tanstack/svelte-query";
  import { Button, Dialog, Input } from "#lib/components/ui";
  import { createChat } from "#lib/api/chats";
  import { chatKeys } from "#lib/api/query-keys";
  import { ApiError } from "#lib/api/client";
  import { auth } from "#lib/auth.svelte.ts";
  import { chatCreateModal } from "../../services/messenger/chatCreateModal.svelte";

  const queryClient = useQueryClient();
  const currentUserId = $derived(auth.user?.profile.id ?? "");

  let name = $state("");
  let membersInput = $state("");
  let error = $state<string | null>(null);

  const mutation = createMutation(() => ({
    mutationFn: (variables: { name: string; members: string[] }) => createChat(variables),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: chatKeys.list() });
    },
  }));

  function reset() {
    name = "";
    membersInput = "";
    error = null;
    mutation.reset();
  }

  function parseMembers(): string[] | null {
    const raw = membersInput
      .split(/[\s,;]+/)
      .map((value) => value.trim())
      .filter(Boolean);

    if (raw.some((value) => !/^\d+$/.test(value))) return null;

    return Array.from(new Set(raw.filter((value) => value !== currentUserId)));
  }

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();

    if (mutation.isPending) return;

    error = null;

    const trimmedName = name.trim();
    const members = parseMembers();

    if (!trimmedName) {
      error = "Введите название чата";
      return;
    }

    if (members === null) {
      error = "ID участников должны быть числами, разделёнными запятыми или пробелами";
      return;
    }

    if (members.length === 0) {
      error = "Добавьте хотя бы одного участника";
      return;
    }

    try {
      await mutation.mutateAsync({ name: trimmedName, members });
      chatCreateModal.close();
      reset();
    } catch (cause) {
      error = cause instanceof ApiError ? cause.message : "Не удалось создать чат";
    }
  }
</script>

<Dialog.Root
  bind:open={() => chatCreateModal.isOpen, () => chatCreateModal.close()}
  onOpenChange={(value) => !value && reset()}
>
  <Dialog.Portal>
    <Dialog.Overlay />
    <Dialog.Content>
      <Dialog.Title>Новый чат</Dialog.Title>

      <form class="flex flex-col gap-(--space-1)" onsubmit={handleSubmit}>
        <div>
          <label
            for="create-chat-name"
            class="mb-(--space-05) block text-(length:--font-size-sm) font-medium text-(--color-text-base)"
          >
            Название
          </label>
          <Input id="create-chat-name" bind:value={name} placeholder="Например, Проект Zinq" autocomplete="off" />
        </div>

        <div>
          <label
            for="create-chat-members"
            class="mb-(--space-05) block text-(length:--font-size-sm) font-medium text-(--color-text-base)"
          >
            ID участников
          </label>
          <Input
            id="create-chat-members"
            bind:value={membersInput}
            placeholder="123456789, 987654321"
            autocomplete="off"
          />
          <p class="mt-(--space-05) text-(length:--font-size-xs) text-(--color-text-subtle)">
            Числовые ID пользователей через запятую или пробел. Вы добавитесь автоматически.
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
          <Button type="button" variant="secondary" onclick={() => chatCreateModal.close()}>Отмена</Button>
          <Button type="submit" disabled={mutation.isPending}>
            {mutation.isPending ? "Создаём…" : "Создать"}
          </Button>
        </div>
      </form>

      <Dialog.Close />
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
