<script lang="ts">
  import { ArrowLeft, Bookmark, Users } from "@lucide/svelte";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { Avatar, Button } from "$lib/components/ui";
  import type { Chat } from "$lib/types";
  import { userProfileModal } from "$services/userProfileModal.svelte.ts";
  import {
    getChatAvatar,
    getChatAvatarFallback,
    getChatDisplayName,
    getOtherMember,
    isSavedMessages,
  } from "./model/chatHelpers";

  interface Props {
    chat: Chat;
    currentUserId: string;
  }

  let { chat, currentUserId }: Props = $props();

  let saved = $derived(isSavedMessages(chat, currentUserId));
  let other = $derived(getOtherMember(chat, currentUserId));
  let displayName = $derived(getChatDisplayName(chat, currentUserId));
  let avatarSrc = $derived(getChatAvatar(chat, currentUserId));
  let fallback = $derived(avatarSrc ? "" : getChatAvatarFallback(chat, currentUserId));
  let canOpenProfile = $derived(chat.type === "DM" && !saved && other !== null);

  let subtitle = $derived.by(() => {
    if (chat.type === "GROUP_DM") {
      const count = chat.members.length;
      return `${count} ${count === 1 ? "member" : "members"}`;
    }

    return other ? `@${other.username}` : "";
  });
</script>

{#snippet chatIdentity()}
  {#if saved}
    <div
      class="flex h-9 w-9 shrink-0 items-center justify-center rounded-(--avatar-radius) border border-(color:--avatar-border) bg-(--color-background-brand) text-(--color-text-on-brand)"
      title="Saved messages"
    >
      <Bookmark class="h-4 w-4" fill="currentColor" />
    </div>
  {:else}
    <Avatar src={avatarSrc} alt={displayName} {fallback} class="h-9 w-9" />
  {/if}

  <div class="min-w-0 flex-1">
    <div class="flex items-center gap-1">
      {#if chat.type === "GROUP_DM"}
        <Users class="h-4 w-4 shrink-0 text-(--color-text-muted)" />
      {/if}
      <h2 class="truncate text-(length:--font-size-md) font-semibold">{displayName}</h2>
    </div>
    {#if subtitle}
      <p class="truncate text-(length:--font-size-xs) text-(--color-text-muted)">{subtitle}</p>
    {/if}
  </div>
{/snippet}

<header
  class="flex h-14 shrink-0 items-center gap-(--space-1) border-b border-(--color-border-base) bg-(--color-background-surface) px-(--space-1) md:px-(--space-2)"
>
  <Button variant="icon" class="md:hidden" onclick={() => goto(resolve("/chats"))}>
    <ArrowLeft class="h-5 w-5" />
  </Button>

  {#if canOpenProfile && other}
    <button
      type="button"
      class="flex min-w-0 flex-1 items-center gap-(--space-1) rounded-(--radius-md) text-left focus-visible:focus-ring"
      title={`Open @${other.username}'s profile`}
      onclick={() => userProfileModal.open(other.user_id)}
    >
      {@render chatIdentity()}
    </button>
  {:else}
    <div class="flex min-w-0 flex-1 items-center gap-(--space-1)">
      {@render chatIdentity()}
    </div>
  {/if}
</header>
