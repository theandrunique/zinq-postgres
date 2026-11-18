<script lang="ts">
  import type { Message } from "$lib/types";
  import { compareIds } from "$lib/utils/id";
  import { groupMessagesByAuthor, groupMessagesByDate } from "./utils";
  import DateDivider from "./DateDivider.svelte";
  import MessageGroup from "./MessageGroup.svelte";
  import ScrollButton from "./ScrollButton.svelte";
  import { createScrollManager } from "./scroll-manager.svelte.ts";

  interface Props {
    messages: Message[];
    currentUserId: string;
    lastReadMessageId?: string;
    maxReadMessageId?: string;
    onMessageVisible?: (messageId: string) => void;
    onReachedBottom?: (newestMessageId: string) => void;
    onLoadAroundMessage?: (targetId: string) => Promise<void>;
    onLoadOlder?: () => void;
  }

  let {
    messages,
    currentUserId,
    lastReadMessageId = "",
    maxReadMessageId = "",
    onMessageVisible,
    onReachedBottom,
    onLoadAroundMessage,
    onLoadOlder,
  }: Props = $props();

  let dateGroups = $derived(groupMessagesByDate(messages));
  let containerEl = $state<HTMLElement | null>(null);

  let manager = $state(createScrollManager());
  let initialScrolled = $state(false);
  let prevMessagesLength = $state(0);

  $effect(() => {
    if (containerEl) manager.setContainer(containerEl);
  });

  $effect(() => {
    if (lastReadMessageId) manager.setLastReadId(lastReadMessageId);
  });

  $effect(() => {
    return () => manager.destroy();
  });

  $effect(() => {
    if (initialScrolled || !containerEl) return;
    if (messages.length === 0) return;

    const el = containerEl;
    const newestId = messages[messages.length - 1].id;

    const raf = requestAnimationFrame(() => {
      if (lastReadMessageId && manager.hasElement(lastReadMessageId)) {
        manager.scrollToMessage(lastReadMessageId, { behavior: "instant" });
      } else {
        // The anchor is not among the loaded messages, so it is either older
        // than the whole page (everything is unread) or missing entirely.
        const firstUnread = messages.find((message) => compareIds(message.id, lastReadMessageId) > 0);

        if (lastReadMessageId && firstUnread) {
          manager.scrollToMessage(firstUnread.id, { behavior: "instant" });
        } else {
          manager.scrollToBottom("instant");
        }
      }

      manager.updateButtonState();
      initialScrolled = true;

      // Short chats and anchors close to the end are fully visible right away,
      // so there is nothing left unread.
      if (isAtBottom(el)) onReachedBottom?.(newestId);
    });

    return () => cancelAnimationFrame(raf);
  });

  $effect(() => {
    const len = messages.length;

    if (prevMessagesLength > 0 && len > prevMessagesLength && containerEl) {
      manager.scrollToBottomOnNewMessage();

      if (isAtBottom(containerEl)) {
        const newestId = messages[len - 1]?.id;
        if (newestId) onReachedBottom?.(newestId);
      }
    }

    prevMessagesLength = len;
  });

  $effect(() => {
    const el = containerEl;

    if (!el) return;

    function onScroll() {
      if (!el) return;

      if (isAtBottom(el)) {
        const newestId = messages[messages.length - 1]?.id;
        if (newestId) onReachedBottom?.(newestId);
      }

      if (el.scrollTop < 200) {
        onLoadOlder?.();
      }
    }

    el.addEventListener("scroll", onScroll);

    return () => el.removeEventListener("scroll", onScroll);
  });

  // Messages are observed only after the initial scroll, otherwise the page
  // could ack everything that was visible at the top for a single frame.
  $effect(() => {
    const el = containerEl;
    const count = messages.length;

    if (!el || !initialScrolled || count === 0) return;

    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (!entry.isIntersecting || !isFullyVisible(entry)) continue;

          const node = entry.target as HTMLElement;
          const messageId = node.dataset.messageId;

          if (!messageId || node.dataset.messageAuthorId === currentUserId) continue;

          onMessageVisible?.(messageId);
        }
      },
      { root: el, threshold: [0, 0.5, 1] },
    );

    const nodes = el.querySelectorAll<HTMLElement>("[data-message-id]");
    for (const node of nodes) observer.observe(node);

    return () => observer.disconnect();
  });

  function isAtBottom(el: HTMLElement): boolean {
    return el.scrollHeight - el.scrollTop - el.clientHeight < 100;
  }

  function isFullyVisible(entry: IntersectionObserverEntry): boolean {
    if (entry.intersectionRatio >= 0.999) return true;

    // Messages taller than the viewport can never reach ratio 1, so treat a
    // message that covers the whole visible area as read.
    const root = entry.rootBounds;
    const rect = entry.boundingClientRect;

    return !!root && rect.height > 0 && rect.top <= root.top + 1 && rect.bottom >= root.bottom - 1;
  }

  async function handleNavigateToMessage(fromId: string, toId: string) {
    if (manager.hasElement(toId)) {
      manager.navigateToReference(fromId, toId);
      return;
    }

    if (onLoadAroundMessage) {
      await onLoadAroundMessage(toId);
      manager.navigateToReference(fromId, toId);
    }
  }
</script>

<div class="relative h-full">
  <div bind:this={containerEl} class="h-full overflow-y-auto" style="overflow-anchor: auto">
    <div class="my-2 flex flex-col gap-2 px-4">
      {#each dateGroups as [dateKey, dateMessages] (dateKey)}
        <div class="flex flex-col gap-2">
          <DateDivider timestamp={dateMessages[0].created_at} />
          {#each groupMessagesByAuthor(dateMessages) as group (group[0].id)}
            <MessageGroup
              {group}
              {currentUserId}
              {maxReadMessageId}
              messageAction={manager.messageAction}
              onNavigateToMessage={handleNavigateToMessage}
            />
          {/each}
        </div>
      {/each}
    </div>
  </div>
  <ScrollButton {manager} />
</div>
