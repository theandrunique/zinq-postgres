<script lang="ts">
  import type { Message } from "#lib/types";
  import { getAttachmentUrl } from "#lib/utils";
  import MediaGrid from "./MediaGrid.svelte";
  import FileCard from "./FileCard.svelte";

  interface Props {
    message: Message;
  }

  let { message }: Props = $props();

  let images = $derived(
    message.attachments.filter(
      (attachment) => attachment.content_type.startsWith("image/") && getAttachmentUrl(attachment) !== null,
    ),
  );

  let audios = $derived(message.attachments.filter((attachment) => attachment.content_type.startsWith("audio/")));

  let others = $derived(
    message.attachments.filter((attachment) => !images.includes(attachment) && !audios.includes(attachment)),
  );
</script>

{#if message.attachments.length > 0}
  <div class="space-y-2">
    {#if images.length > 0}
      <MediaGrid attachments={images} />
    {/if}
    {#each audios as audio (audio.id)}
      <FileCard attachment={audio} isAudio />
    {/each}
    {#each others as file (file.id)}
      <FileCard attachment={file} />
    {/each}
  </div>
{/if}
