<script lang="ts">
  import { cn } from "#lib/utils";
  import { Avatar } from "bits-ui";

  let {
    src,
    alt,
    fallback,
    class: className,
    ...rest
  }: Avatar.RootProps & {
    src: string | null | undefined;
    alt: string;
    fallback: string;
  } = $props();

  let rootClasses = $derived(
    cn(
      "h-(--avatar-size) w-(--avatar-size) rounded-(--avatar-radius)",
      "border-(length:--avatar-border-width) border-(color:--avatar-border)",
      "bg-(--avatar-background) text-(length:--avatar-font-size) font-(weight:--avatar-font-weight) text-(--avatar-text) uppercase",
      className,
    ),
  );
</script>

<Avatar.Root delayMs={200} class={rootClasses} {...rest}>
  <div class="flex h-full w-full items-center justify-center overflow-hidden rounded-(--avatar-radius)">
    <Avatar.Image {src} {alt} class="h-full w-full object-cover" />
    <Avatar.Fallback>
      {fallback}
    </Avatar.Fallback>
  </div>
</Avatar.Root>
