<script lang="ts">
  import { QueryClientProvider } from "@tanstack/svelte-query";
  import { Tooltip } from "bits-ui";
  import "#lib/styles/index.css";
  import AuthGuard from "#lib/components/AuthGuard.svelte";
  import { auth } from "#lib/auth.svelte.ts";
  import type { LayoutProps } from "./$types";
  import { queryClient } from "#lib/queries/query-client.ts";

  let { children }: LayoutProps = $props();

  $effect(() => {
    if (!auth.user) queryClient.clear();
  });
</script>

<QueryClientProvider client={queryClient}>
  <Tooltip.Provider>
    <AuthGuard>
      {@render children()}
    </AuthGuard>
  </Tooltip.Provider>
</QueryClientProvider>
