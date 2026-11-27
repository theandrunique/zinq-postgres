<script lang="ts">
  import { resolve } from "$app/paths";
  import { auth, type UserSessionLifetime } from "$lib/auth.svelte.ts";
  import { Avatar, Button } from "$lib/components/ui/index.ts";
  import { appVersion, versionLabel } from "$lib/version.ts";

  const SESSION_LABELS: Record<UserSessionLifetime, string> = {
    WEEK: "1 week",
    MONTH: "1 month",
    MONTH3: "3 months",
    MONTH6: "6 months",
    MONTH12: "12 months",
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

  const createdAt = $derived(profile ? new Date(profile.created_at).toLocaleString("en-US") : "—");

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

<div
  class="min-h-screen bg-(--color-background-body) px-(--space-4) py-(--space-5) text-(--color-text-base) sm:px-(--space-3)"
>
  <div class="mx-auto w-full max-w-3xl">
    {#if profile}
      <header class="mb-(--space-3) flex flex-wrap items-center justify-between gap-(--space-2)">
        <div>
          <h1 class="text-(length:--font-size-2xl) font-semibold tracking-tight text-(--color-text-base)">Profile</h1>
          <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">You're signed in to Zinq</p>
        </div>

        <div class="flex items-center gap-(--space-1)">
          <a
            href={resolve("/chats")}
            class="rounded-(--button-radius) px-(--button-padding-x) py-(--button-padding-y) text-(length:--font-size-sm) font-medium text-(--color-text-link) hover:text-(--color-text-link-hover)"
          >
            Back to chats
          </a>
          <Button variant="secondary" disabled={signingOut} onclick={handleSignout}>
            {signingOut ? "Signing out…" : "Sign out"}
          </Button>
        </div>
      </header>

      <section
        class="rounded-(--radius-2xl) border-(length:--border-width-default) border-(color:--color-border-base) bg-(--color-background-surface) p-(--space-2) shadow-(--shadow-elevation-3) sm:p-(--space-3)"
      >
        <div class="flex items-center gap-(--space-2)">
          <Avatar src={profile.avatar} alt={`Avatar of ${profile.display_name}`} fallback={initials} />

          <div class="min-w-0">
            <p class="truncate text-(length:--font-size-lg) font-semibold text-(--color-text-base)">
              {profile.display_name}
            </p>
            <p class="truncate text-(length:--font-size-sm) text-(--color-text-brand)">@{profile.username}</p>
          </div>
        </div>

        <dl class="mt-(--space-3) grid grid-cols-1 gap-x-(--space-3) gap-y-(--space-2) sm:grid-cols-2">
          <div>
            <dt class="text-(length:--font-size-xs) font-medium tracking-wide text-(--color-text-subtle) uppercase">
              Display name
            </dt>
            <dd class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-base)">
              {profile.display_name}
            </dd>
          </div>

          <div>
            <dt class="text-(length:--font-size-xs) font-medium tracking-wide text-(--color-text-subtle) uppercase">
              Username
            </dt>
            <dd class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-base)">@{profile.username}</dd>
          </div>

          <div>
            <dt class="text-(length:--font-size-xs) font-medium tracking-wide text-(--color-text-subtle) uppercase">
              ID
            </dt>
            <dd class="mt-(--space-05) font-mono text-(length:--font-size-sm) text-(--color-text-muted)">
              {profile.id}
            </dd>
          </div>

          <div>
            <dt class="text-(length:--font-size-xs) font-medium tracking-wide text-(--color-text-subtle) uppercase">
              Email
            </dt>
            <dd
              class="mt-(--space-05) flex flex-wrap items-center gap-(--space-1) text-(length:--font-size-sm) text-(--color-text-base)"
            >
              <span class="break-all">{profile.email}</span>
              {#if profile.email_verified}
                <span
                  class="rounded-(--radius-full) border-(length:--border-width-default) border-(color:--color-border-success) bg-(--color-background-success-subtle) px-(--space-1) py-(--space-05) text-(length:--font-size-xs) font-medium text-(--color-text-success)"
                >
                  Verified
                </span>
              {:else}
                <span
                  class="rounded-(--radius-full) border-(length:--border-width-default) border-(color:--color-border-warn) bg-(--color-background-warn-subtle) px-(--space-1) py-(--space-05) text-(length:--font-size-xs) font-medium text-(--color-text-warn)"
                >
                  Not verified
                </span>
              {/if}
            </dd>
          </div>

          <div>
            <dt class="text-(length:--font-size-xs) font-medium tracking-wide text-(--color-text-subtle) uppercase">
              Session lifetime
            </dt>
            <dd class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-base)">{sessionLabel}</dd>
          </div>

          <div>
            <dt class="text-(length:--font-size-xs) font-medium tracking-wide text-(--color-text-subtle) uppercase">
              Registered
            </dt>
            <dd class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-base)">{createdAt}</dd>
          </div>

          <div class="sm:col-span-2">
            <dt class="text-(length:--font-size-xs) font-medium tracking-wide text-(--color-text-subtle) uppercase">
              About
            </dt>
            <dd class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-base)">
              {profile.bio || "—"}
            </dd>
          </div>
        </dl>
      </section>
    {:else}
      <p class="text-(length:--font-size-sm) text-(--color-text-muted)">Profile unavailable.</p>
    {/if}

    <footer class="mt-(--space-3) text-center text-(length:--font-size-xs) text-(--color-text-subtle)">
      {versionLabel} ({appVersion.commit})
    </footer>
  </div>
</div>
