<script lang="ts">
  import { File, Play } from "@lucide/svelte";
  import type { Attachment } from "#lib/types";
  import { getAttachmentUrl } from "#lib/utils";

  interface Props {
    attachment: Attachment;
    isAudio?: boolean;
  }

  let { attachment, isAudio = false }: Props = $props();

  let url = $derived(getAttachmentUrl(attachment));

  function formatFileSize(size: number): string {
    if (size < 1024) return `${size} Б`;
    if (size < 1024 * 1024) return `${(size / 1024).toFixed(1)} КБ`;
    return `${(size / (1024 * 1024)).toFixed(1)} МБ`;
  }

  function handleClick() {
    if (url) window.open(url, "_blank");
  }
</script>

<button
  type="button"
  disabled={!url}
  class="flex min-w-48 items-center gap-2.5 text-left enabled:cursor-pointer"
  onclick={handleClick}
>
  <div class="rounded-full bg-(--color-background-interactive) p-3">
    {#if isAudio}
      <Play class="h-5 w-5 text-(--color-text-base)" fill="currentColor" />
    {:else}
      <File class="h-5 w-5 text-(--color-text-base)" />
    {/if}
  </div>
  <div class="flex min-w-0 flex-col overflow-hidden">
    <div class="truncate text-sm font-medium">
      {attachment.filename}
    </div>
    <div class="text-xs opacity-70">
      {formatFileSize(attachment.size)}
    </div>
  </div>
</button>
