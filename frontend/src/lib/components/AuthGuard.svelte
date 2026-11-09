<script lang="ts">
  import { untrack, type Snippet } from "svelte";
  import { browser } from "$app/env";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import { auth } from "#lib/auth.svelte.ts";

  let { children, publicPaths = ["/login", "/register"] }: { children: Snippet; publicPaths?: string[] } = $props();

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
  <div class="flex min-h-screen flex-col items-center justify-center gap-4 bg-slate-950 px-4 text-center">
    <p class="text-sm text-slate-300">{authError}</p>
    <button
      type="button"
      class="rounded-lg bg-sky-500 px-4 py-2 text-sm font-semibold text-slate-950 transition hover:bg-sky-400 focus:ring-2 focus:ring-sky-400/50 focus:outline-none"
      onclick={retry}
    >
      Повторить
    </button>
  </div>
{:else if checking}
  <div class="flex min-h-screen items-center justify-center bg-slate-950 px-4">
    <p class="animate-pulse text-sm text-slate-400">Проверка аутентификации…</p>
  </div>
{:else}
  {@render children()}
{/if}
