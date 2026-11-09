<script lang="ts">
  import { resolve } from "$app/paths";
  import { AuthError, auth } from "#lib/auth.svelte.ts";

  let username = $state("");
  let display_name = $state("");
  let email = $state("");
  let password = $state("");
  let submitting = $state(false);
  let error = $state<string | null>(null);
  let fieldErrors = $state<Record<string, string[]>>({});

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();

    if (submitting) return;

    submitting = true;
    error = null;
    fieldErrors = {};

    try {
      await auth.signup({ username, display_name, email, password });
    } catch (cause) {
      if (cause instanceof AuthError) {
        fieldErrors = cause.errors ?? {};
        error = cause.message;
      } else {
        error =
          cause instanceof Error && cause.message ? cause.message : "Не удалось создать аккаунт. Попробуйте ещё раз.";
      }
    } finally {
      submitting = false;
    }
  }
</script>

<svelte:head>
  <title>Zinq • Register</title>
</svelte:head>

<div class="flex min-h-screen items-center justify-center bg-slate-950 px-4 py-10">
  <div class="w-full max-w-md">
    <div class="rounded-2xl border border-slate-800 bg-slate-900 p-8 shadow-xl shadow-black/30">
      <h1 class="text-2xl font-semibold tracking-tight text-white">Регистрация</h1>
      <p class="mt-1 text-sm text-slate-400">Создайте аккаунт, чтобы начать работу с Zinq</p>

      <form class="mt-6 space-y-5" onsubmit={handleSubmit}>
        <div>
          <label for="username" class="mb-1.5 block text-sm font-medium text-slate-300"> Имя пользователя </label>
          <input
            id="username"
            name="username"
            type="text"
            autocomplete="username"
            required
            bind:value={username}
            class="w-full rounded-lg border border-slate-700 bg-slate-950 px-3 py-2 text-sm text-slate-100 placeholder-slate-500 transition outline-none focus:border-sky-500 focus:ring-2 focus:ring-sky-500/30"
          />
          {#if fieldErrors.username?.[0]}
            <p class="mt-1.5 text-sm text-red-400">{fieldErrors.username[0]}</p>
          {/if}
        </div>

        <div>
          <label for="display_name" class="mb-1.5 block text-sm font-medium text-slate-300"> Отображаемое имя </label>
          <input
            id="display_name"
            name="display_name"
            type="text"
            autocomplete="name"
            required
            bind:value={display_name}
            class="w-full rounded-lg border border-slate-700 bg-slate-950 px-3 py-2 text-sm text-slate-100 placeholder-slate-500 transition outline-none focus:border-sky-500 focus:ring-2 focus:ring-sky-500/30"
          />
          {#if fieldErrors.display_name?.[0]}
            <p class="mt-1.5 text-sm text-red-400">
              {fieldErrors.display_name[0]}
            </p>
          {/if}
        </div>

        <div>
          <label for="email" class="mb-1.5 block text-sm font-medium text-slate-300"> Email </label>
          <input
            id="email"
            name="email"
            type="email"
            autocomplete="email"
            required
            bind:value={email}
            class="w-full rounded-lg border border-slate-700 bg-slate-950 px-3 py-2 text-sm text-slate-100 placeholder-slate-500 transition outline-none focus:border-sky-500 focus:ring-2 focus:ring-sky-500/30"
          />
          {#if fieldErrors.email?.[0]}
            <p class="mt-1.5 text-sm text-red-400">{fieldErrors.email[0]}</p>
          {/if}
        </div>

        <div>
          <label for="password" class="mb-1.5 block text-sm font-medium text-slate-300"> Пароль </label>
          <input
            id="password"
            name="password"
            type="password"
            autocomplete="new-password"
            required
            bind:value={password}
            class="w-full rounded-lg border border-slate-700 bg-slate-950 px-3 py-2 text-sm text-slate-100 placeholder-slate-500 transition outline-none focus:border-sky-500 focus:ring-2 focus:ring-sky-500/30"
          />
          {#if fieldErrors.password?.[0]}
            <p class="mt-1.5 text-sm text-red-400">{fieldErrors.password[0]}</p>
          {/if}
        </div>

        {#if error}
          <p role="alert" class="rounded-lg border border-red-500/40 bg-red-500/10 px-3 py-2 text-sm text-red-400">
            {error}
          </p>
        {/if}

        <button
          type="submit"
          disabled={submitting}
          class="w-full rounded-lg bg-sky-500 px-4 py-2 text-sm font-semibold text-slate-950 transition hover:bg-sky-400 focus:ring-2 focus:ring-sky-400/50 focus:outline-none disabled:cursor-not-allowed disabled:opacity-60"
        >
          {submitting ? "Создаём аккаунт…" : "Создать аккаунт"}
        </button>
      </form>

      <p class="mt-6 text-center text-sm text-slate-400">
        Уже есть аккаунт?
        <a href={resolve("/login")} class="font-medium text-sky-400 transition hover:text-sky-300"> Войти </a>
      </p>
    </div>
  </div>
</div>
