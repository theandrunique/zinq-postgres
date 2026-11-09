<script lang="ts">
  import { auth, type SessionLifetime } from "#lib/auth.svelte.ts";
  import { appVersion, versionLabel } from "#lib/version.ts";

  const SESSION_LABELS: Record<SessionLifetime, string> = {
    WEEK: "1 неделя",
    MONTH: "1 месяц",
    MONTH3: "3 месяца",
    MONTH6: "6 месяцев",
    MONTH12: "12 месяцев",
  };

  const profile = $derived(auth.user?.profile ?? null);

  const initials = $derived(
    (profile?.display_name || profile?.username || "")
      .trim()
      .split(/\s+/)
      .filter(Boolean)
      .slice(0, 2)
      .map((part) => part[0]?.toUpperCase() ?? "")
      .join("") || "?",
  );

  const createdAt = $derived(profile ? new Date(profile.created_at).toLocaleString("ru-RU") : "—");

  const sessionLabel = $derived(profile ? (SESSION_LABELS[profile.sessions_ttl] ?? "—") : "—");

  let signingOut = $state(false);

  async function handleSignout() {
    if (signingOut) return;

    signingOut = true;

    try {
      await auth.signout();
    } finally {
      signingOut = false;
    }
  }
</script>

<svelte:head>
  <title>Zinq • Profile</title>
</svelte:head>

<div class="min-h-screen bg-slate-950 px-4 py-10 text-slate-100 sm:px-6">
  <div class="mx-auto w-full max-w-3xl">
    {#if profile}
      <header class="mb-8 flex flex-wrap items-center justify-between gap-4">
        <div>
          <h1 class="text-3xl font-semibold tracking-tight text-white">Профиль</h1>
          <p class="mt-1 text-sm text-slate-400">Вы вошли в аккаунт Zinq</p>
        </div>

        <button
          type="button"
          class="rounded-lg border border-slate-700 px-4 py-2 text-sm font-medium text-slate-200 transition hover:border-red-500/60 hover:text-red-300 disabled:cursor-not-allowed disabled:opacity-60"
          disabled={signingOut}
          onclick={handleSignout}
        >
          {signingOut ? "Выходим…" : "Выйти"}
        </button>
      </header>

      <section class="rounded-2xl border border-slate-800 bg-slate-900 p-6 shadow-xl shadow-black/20 sm:p-8">
        <div class="flex items-center gap-4">
          {#if profile.avatar}
            <img
              src={profile.avatar}
              alt="Аватар {profile.display_name}"
              class="h-16 w-16 rounded-full object-cover ring-2 ring-slate-800"
            />
          {:else}
            <div
              class="flex h-16 w-16 shrink-0 items-center justify-center rounded-full bg-sky-500 text-xl font-semibold text-slate-950"
            >
              {initials}
            </div>
          {/if}

          <div class="min-w-0">
            <p class="truncate text-lg font-semibold text-white">
              {profile.display_name}
            </p>
            <p class="truncate text-sm text-sky-400">@{profile.username}</p>
          </div>
        </div>

        <dl class="mt-8 grid grid-cols-1 gap-x-8 gap-y-5 sm:grid-cols-2">
          <div>
            <dt class="text-xs font-medium tracking-wide text-slate-500 uppercase">Отображаемое имя</dt>
            <dd class="mt-1 text-sm text-slate-200">{profile.display_name}</dd>
          </div>

          <div>
            <dt class="text-xs font-medium tracking-wide text-slate-500 uppercase">Имя пользователя</dt>
            <dd class="mt-1 text-sm text-slate-200">@{profile.username}</dd>
          </div>

          <div>
            <dt class="text-xs font-medium tracking-wide text-slate-500 uppercase">ID</dt>
            <dd class="mt-1 font-mono text-sm text-slate-400">{profile.id}</dd>
          </div>

          <div>
            <dt class="text-xs font-medium tracking-wide text-slate-500 uppercase">Email</dt>
            <dd class="mt-1 flex flex-wrap items-center gap-2 text-sm text-slate-200">
              <span class="break-all">{profile.email}</span>
              {#if profile.email_verified}
                <span
                  class="rounded-full border border-emerald-500/40 bg-emerald-500/10 px-2 py-0.5 text-xs font-medium text-emerald-400"
                >
                  Подтверждён
                </span>
              {:else}
                <span
                  class="rounded-full border border-amber-500/40 bg-amber-500/10 px-2 py-0.5 text-xs font-medium text-amber-400"
                >
                  Не подтверждён
                </span>
              {/if}
            </dd>
          </div>

          <div>
            <dt class="text-xs font-medium tracking-wide text-slate-500 uppercase">Срок сессии</dt>
            <dd class="mt-1 text-sm text-slate-200">{sessionLabel}</dd>
          </div>

          <div>
            <dt class="text-xs font-medium tracking-wide text-slate-500 uppercase">Зарегистрирован</dt>
            <dd class="mt-1 text-sm text-slate-200">{createdAt}</dd>
          </div>

          <div class="sm:col-span-2">
            <dt class="text-xs font-medium tracking-wide text-slate-500 uppercase">О себе</dt>
            <dd class="mt-1 text-sm text-slate-200">
              {profile.bio || "—"}
            </dd>
          </div>
        </dl>
      </section>
    {:else}
      <p class="text-sm text-slate-400">Профиль недоступен.</p>
    {/if}

    <footer class="mt-10 text-center text-xs text-slate-500">
      {versionLabel} ({appVersion.commit})
    </footer>
  </div>
</div>
