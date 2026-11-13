<script lang="ts">
  import type { Attachment } from "#lib/types";
  import { getAttachmentUrl } from "#lib/utils";

  interface Props {
    attachment: Attachment;
    class?: string;
  }

  let { attachment, class: className = "" }: Props = $props();

  let url = $derived(getAttachmentUrl(attachment));
  let isImage = $derived(attachment.content_type.startsWith("image/"));
  let isVideo = $derived(attachment.content_type.startsWith("video/"));
</script>

{#if url && isImage}
  <img src={url} alt={attachment.filename} class="{className} rounded-lg object-cover" />
{:else if url && isVideo}
  <div class="{className} overflow-hidden rounded-lg">
    <video controls src={url} class="h-full w-full object-cover">
      <track kind="captions" />
    </video>
  </div>
{/if}
