<script lang="ts">
  import { resolve } from "$app/paths";
  import { AuthError, auth } from "#lib/auth.svelte.ts";

  let username = $state("");
  let password = $state("");
  let totp = $state("");
  let showTotp = $state(false);
  let submitting = $state(false);
  let error = $state<string | null>(null);

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();

    if (submitting) return;

    submitting = true;
    error = null;

    try {
      await auth.signin({
        username,
        password,
        totp: showTotp && totp.trim() ? totp.trim() : undefined,
      });
    } catch (cause) {
      if (cause instanceof AuthError && cause.requiresTotp) {
        showTotp = true;
        error = "Введите код из приложения-аутентификатора";
      } else {
        error = cause instanceof Error && cause.message ? cause.message : "Не удалось войти. Попробуйте ещё раз.";
      }
    } finally {
      submitting = false;
    }
  }
</script>

<svelte:head>
  <title>Zinq • Login</title>
</svelte:head>

<div class="flex min-h-screen items-center justify-center bg-slate-950 px-4 py-10">
  <div class="w-full max-w-md">
    <div class="rounded-2xl border border-slate-800 bg-slate-900 p-8 shadow-xl shadow-black/30">
      <h1 class="text-2xl font-semibold tracking-tight text-white">Вход</h1>
      <p class="mt-1 text-sm text-slate-400">Войдите в аккаунт Zinq, чтобы продолжить</p>

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
        </div>

        <div>
          <label for="password" class="mb-1.5 block text-sm font-medium text-slate-300"> Пароль </label>
          <input
            id="password"
            name="password"
            type="password"
            autocomplete="current-password"
            required
            bind:value={password}
            class="w-full rounded-lg border border-slate-700 bg-slate-950 px-3 py-2 text-sm text-slate-100 placeholder-slate-500 transition outline-none focus:border-sky-500 focus:ring-2 focus:ring-sky-500/30"
          />
        </div>

        {#if showTotp}
          <div>
            <label for="totp" class="mb-1.5 block text-sm font-medium text-slate-300"> Код подтверждения </label>
            <input
              id="totp"
              name="totp"
              type="text"
              autocomplete="one-time-code"
              inputmode="numeric"
              bind:value={totp}
              class="w-full rounded-lg border border-slate-700 bg-slate-950 px-3 py-2 text-sm text-slate-100 placeholder-slate-500 transition outline-none focus:border-sky-500 focus:ring-2 focus:ring-sky-500/30"
            />
          </div>
        {/if}

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
          {submitting ? "Входим…" : "Войти"}
        </button>
      </form>

      <p class="mt-6 text-center text-sm text-slate-400">
        Нет аккаунта?
        <a href={resolve("/register")} class="font-medium text-sky-400 transition hover:text-sky-300">
          Зарегистрироваться
        </a>
      </p>
    </div>
  </div>
</div>
