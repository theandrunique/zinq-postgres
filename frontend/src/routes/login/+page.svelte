<script lang="ts">
  import { resolve } from "$app/paths";
  import { AuthError, auth } from "#lib/auth.svelte.ts";
  import { Button, Input } from "#lib/components/ui/index.ts";

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

<div class="flex min-h-screen items-center justify-center bg-(--color-background-body) px-(--space-4) py-(--space-5)">
  <div class="w-full max-w-md">
    <div
      class="rounded-(--radius-2xl) border-(length:--border-width-default) border-(color:--color-border-base) bg-(--color-background-surface) p-(--space-4) shadow-(--shadow-elevation-3)"
    >
      <h1 class="text-(length:--font-size-xl) font-semibold tracking-tight text-(--color-text-base)">Вход</h1>
      <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">
        Войдите в аккаунт Zinq, чтобы продолжить
      </p>

      <form class="mt-(--space-2) flex flex-col gap-(--space-1)" onsubmit={handleSubmit}>
        <div>
          <label
            for="username"
            class="mb-(--space-05) block text-(length:--font-size-sm) font-medium text-(--color-text-base)"
          >
            Имя пользователя
          </label>
          <Input id="username" name="username" type="text" autocomplete="username" required bind:value={username} />
        </div>

        <div>
          <label
            for="password"
            class="mb-(--space-05) block text-(length:--font-size-sm) font-medium text-(--color-text-base)"
          >
            Пароль
          </label>
          <Input
            id="password"
            name="password"
            type="password"
            autocomplete="current-password"
            required
            bind:value={password}
          />
        </div>

        {#if showTotp}
          <div>
            <label
              for="totp"
              class="mb-(--space-05) block text-(length:--font-size-sm) font-medium text-(--color-text-base)"
            >
              Код подтверждения
            </label>
            <Input
              id="totp"
              name="totp"
              type="text"
              autocomplete="one-time-code"
              inputmode="numeric"
              bind:value={totp}
            />
          </div>
        {/if}

        {#if error}
          <p
            role="alert"
            class="rounded-(--radius-md) border-(length:--border-width-default) border-(color:--color-border-error) bg-(--color-background-error-subtle) px-(--space-1) py-(--space-05) text-(length:--font-size-sm) text-(--color-text-error)"
          >
            {error}
          </p>
        {/if}

        <Button type="submit" disabled={submitting} class="w-full">
          {submitting ? "Входим…" : "Войти"}
        </Button>
      </form>

      <p class="mt-(--space-2) text-center text-(length:--font-size-sm) text-(--color-text-muted)">
        Нет аккаунта?
        <a
          href={resolve("/register")}
          class="font-medium text-(--color-text-link) transition-colors duration-(--duration-fast) hover:text-(--color-text-link-hover)"
        >
          Зарегистрироваться
        </a>
      </p>
    </div>
  </div>
</div>
