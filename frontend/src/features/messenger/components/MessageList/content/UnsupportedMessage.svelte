<script lang="ts">
  import type { Message } from "#lib/types";

  interface Props {
    message: Message;
  }

  let { message }: Props = $props();
</script>

<div class="text-xs italic opacity-50">
  Неподдерживаемый тип сообщения: {message.type}
</div>
