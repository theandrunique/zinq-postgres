<script lang="ts">
  import "./layout.css";
  import AuthGuard from "#lib/components/AuthGuard.svelte";
  import type { LayoutProps } from "./$types";

  let { children }: LayoutProps = $props();
</script>

<AuthGuard>
  {@render children()}
</AuthGuard>
