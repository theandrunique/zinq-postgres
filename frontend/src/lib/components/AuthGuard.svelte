<script lang="ts">
  import { untrack, type Snippet } from "svelte";
  import { browser } from "$app/env";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import { auth } from "#lib/auth.svelte.ts";
  import { Button, Loader } from "#lib/components/ui/index.ts";

  let {
    children,
    publicPaths = ["/login", "/register", "/dev/components"],
  }: { children: Snippet; publicPaths?: string[] } = $props();

  const isPublic = $derived(publicPaths.includes(page.url.pathname));

  let checking = $state(untrack(() => !publicPaths.includes(page.url.pathname)));
  let authError = $state<string | null>(null);

  async function checkSession(): Promise<void> {
    if (!browser) return;

    if (isPublic) {
      checking = false;
      authError = null;
      void auth.getUser().catch(() => {});
      return;
    }

    checking = true;
    authError = null;

    try {
      const user = await auth.getUser();

      if (!user) {
        void goto(resolve("/login"));
      } else {
        checking = false;
      }
    } catch {
      checking = false;
      authError = "Не удалось проверить сессию. Проверьте подключение и попробуйте снова.";
    }
  }

  $effect(() => {
    // `isPublic` is read inside `checkSession`, so the effect re-runs on navigation.
    void checkSession();
  });

  $effect(() => {
    if (!browser || checking || authError) return;

    if (!auth.user && !isPublic) {
      void goto(resolve("/login"));
    } else if (auth.user && !auth.user.expired && isPublic) {
      void goto(resolve("/"));
    }
  });

  function retry() {
    void checkSession();
  }
</script>

{#if authError}
  <div
    class="flex min-h-screen flex-col items-center justify-center gap-(--space-2) bg-(--color-background-body) px-(--space-4) text-center"
  >
    <p class="text-(length:--font-size-sm) text-(--color-text-muted)">{authError}</p>
    <Button onclick={retry}>Повторить</Button>
  </div>
{:else if checking}
  <div
    class="flex min-h-screen items-center justify-center gap-(--space-1) bg-(--color-background-body) px-(--space-4)"
  >
    <Loader size="sm" />
    <p class="text-(length:--font-size-sm) text-(--color-text-muted)">Проверка аутентификации…</p>
  </div>
{:else}
  {@render children()}
{/if}
