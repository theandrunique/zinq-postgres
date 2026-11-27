<script lang="ts">
  import { Dialog } from "$lib/components/ui";
  import { auth } from "$lib/auth.svelte.ts";
  import type { PublicUser } from "$lib/types";
  import { userProfileModal } from "$services/userProfileModal.svelte.ts";
  import { userSearchModal } from "$services/userSearchModal.svelte.ts";
  import UserSearchInput from "./UserSearchInput.svelte";

  const currentUserId = $derived(auth.user?.profile.id ?? "");
  const excludeIds = $derived(currentUserId ? [currentUserId] : []);

  function handleSelect(user: PublicUser) {
    userSearchModal.close();
    userProfileModal.open(user.id);
  }
</script>

<Dialog.Root bind:open={() => userSearchModal.isOpen, () => userSearchModal.close()}>
  <Dialog.Portal>
    <Dialog.Overlay />
    <Dialog.Content class="sm:max-w-md">
      <Dialog.Title>Search users</Dialog.Title>

      <div class="mt-(--space-1)">
        <UserSearchInput autofocus {excludeIds} placeholder="Search by username…" onSelect={handleSelect} />
      </div>

      <p class="mt-(--space-1) text-(length:--font-size-xs) text-(--color-text-subtle)">
        Find someone by username to open their profile and start a chat.
      </p>

      <Dialog.Close />
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
