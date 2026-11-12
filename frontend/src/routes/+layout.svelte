<script lang="ts">
  import { Tooltip } from "bits-ui";
  import "#lib/styles/index.css";
  import AuthGuard from "#lib/components/AuthGuard.svelte";
  import type { LayoutProps } from "./$types";

  let { children }: LayoutProps = $props();
</script>

<Tooltip.Provider>
  <AuthGuard>
    {@render children()}
  </AuthGuard>
</Tooltip.Provider>
