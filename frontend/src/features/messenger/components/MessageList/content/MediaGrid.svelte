<script lang="ts">
  import type { Attachment } from "#lib/types";
  import { getAttachmentUrl } from "#lib/utils";
  import { MediaGrid, type ImageMeta } from "../../MediaGrid";

  interface Props {
    attachments: Attachment[];
  }

  let { attachments }: Props = $props();

  let images = $derived.by(() => {
    const result: ImageMeta[] = [];

    for (const attachment of attachments) {
      if (!attachment.content_type.startsWith("image/")) continue;

      const src = getAttachmentUrl(attachment);
      if (!src) continue;

      result.push({ src, width: 4, height: 3 });
    }

    return result;
  });
</script>

{#if images.length > 0}
  <MediaGrid {images} rowHeight={150} maxHeight={260} class="max-w-md" />
{/if}
