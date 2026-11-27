<script lang="ts">
  import { Avatar } from "$lib/components/ui";
  import type { Message } from "$lib/types";
  import { isMetaMessage } from "./utils";
  import MessageBubble from "./MessageBubble.svelte";
  import MetaMessage from "./content/MetaMessage.svelte";

  interface Props {
    group: Message[];
    currentUserId: string;
    maxReadMessageId?: string;
    messageAction: (node: HTMLElement, id: string) => { destroy: () => void };
    onNavigateToMessage?: (fromId: string, toId: string) => void;
    onAuthorClick?: (userId: string) => void;
  }

  let {
    group,
    currentUserId,
    maxReadMessageId = "",
    messageAction,
    onNavigateToMessage,
    onAuthorClick,
  }: Props = $props();

  let firstMessage = $derived(group[0]);
  let isOwn = $derived(firstMessage.author_id === currentUserId);
  let isMeta = $derived(isMetaMessage(firstMessage));
  let author = $derived(firstMessage.author);
  let authorName = $derived(author?.display_name || author?.username || "User");
</script>

{#if isMeta}
  {#each group as message (message.id)}
    <div use:messageAction={message.id}>
      <MetaMessage {message} />
    </div>
  {/each}
{:else}
  <div class="flex w-full gap-2">
    {#if !isOwn}
      <div class="sticky bottom-2 shrink-0 self-end text-[0]">
        <button
          type="button"
          class="rounded-(--avatar-radius) transition-opacity duration-100 hover:opacity-80 focus-visible:focus-ring"
          title={`Open ${authorName}'s profile`}
          onclick={() => onAuthorClick?.(firstMessage.author_id)}
        >
          <Avatar src={author?.avatar} alt={authorName} fallback={authorName.charAt(0).toUpperCase()} class="h-8 w-8" />
        </button>
      </div>
    {/if}
    <div class="flex min-w-0 flex-1 flex-col gap-0.5">
      {#each group as message, i (message.id)}
        <MessageBubble
          {message}
          isOwnMessage={isOwn}
          isFirstInGroup={i === 0}
          isLastInGroup={i === group.length - 1}
          {maxReadMessageId}
          {messageAction}
          {onNavigateToMessage}
          {onAuthorClick}
        />
      {/each}
    </div>
  </div>
{/if}
