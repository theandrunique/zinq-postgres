<script lang="ts">
  import { Avatar } from "$lib/components/ui";
  import { renderMessageText } from "../../../model/messageHelpers.ts";
  import type { Message } from "$lib/types";

  interface Props {
    message: Message;
  }

  let { message }: Props = $props();

  let text = $derived(renderMessageText(message));
  let imageUrl = $derived.by(() => {
    if (message.type !== "CHAT_IMAGE_UPDATE") return null;

    const value = message.metadata.new_image;
    return typeof value === "string" && /^https?:\/\//i.test(value) ? value : null;
  });
</script>

<div
  data-message-id={message.id}
  data-message-author-id={message.author_id}
  class="mb-2 text-center text-sm text-(--color-text-muted)"
>
  {#if imageUrl}
    <div class="flex flex-col items-center gap-2">
      <div class="opacity-80">{text}</div>
      <Avatar src={imageUrl} alt="Chat image" fallback="" class="h-24 w-24" />
    </div>
  {:else}
    <div class="opacity-80">{text}</div>
  {/if}
</div>
