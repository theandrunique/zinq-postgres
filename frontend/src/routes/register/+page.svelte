<script lang="ts">
  import { resolve } from "$app/paths";
  import { AuthError, auth } from "#lib/auth.svelte.ts";
  import { Button, Input } from "#lib/components/ui/index.ts";

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

<div class="flex min-h-screen items-center justify-center bg-(--color-background-body) px-(--space-4) py-(--space-5)">
  <div class="w-full max-w-md">
    <div
      class="rounded-(--radius-2xl) border-(length:--border-width-default) border-(color:--color-border-base) bg-(--color-background-surface) p-(--space-4) shadow-(--shadow-elevation-3)"
    >
      <h1 class="text-(length:--font-size-xl) font-semibold tracking-tight text-(--color-text-base)">Регистрация</h1>
      <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">
        Создайте аккаунт, чтобы начать работу с Zinq
      </p>

      <form class="mt-(--space-2) flex flex-col gap-(--space-1)" onsubmit={handleSubmit}>
        <div>
          <label
            for="username"
            class="mb-(--space-05) block text-(length:--font-size-sm) font-medium text-(--color-text-base)"
          >
            Имя пользователя
          </label>
          <Input
            id="username"
            name="username"
            type="text"
            autocomplete="username"
            required
            aria-invalid={Boolean(fieldErrors.username?.[0])}
            bind:value={username}
          />
          {#if fieldErrors.username?.[0]}
            <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-error)">
              {fieldErrors.username[0]}
            </p>
          {/if}
        </div>

        <div>
          <label
            for="display_name"
            class="mb-(--space-05) block text-(length:--font-size-sm) font-medium text-(--color-text-base)"
          >
            Отображаемое имя
          </label>
          <Input
            id="display_name"
            name="display_name"
            type="text"
            autocomplete="name"
            required
            aria-invalid={Boolean(fieldErrors.display_name?.[0])}
            bind:value={display_name}
          />
          {#if fieldErrors.display_name?.[0]}
            <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-error)">
              {fieldErrors.display_name[0]}
            </p>
          {/if}
        </div>

        <div>
          <label
            for="email"
            class="mb-(--space-05) block text-(length:--font-size-sm) font-medium text-(--color-text-base)"
          >
            Email
          </label>
          <Input
            id="email"
            name="email"
            type="email"
            autocomplete="email"
            required
            aria-invalid={Boolean(fieldErrors.email?.[0])}
            bind:value={email}
          />
          {#if fieldErrors.email?.[0]}
            <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-error)">{fieldErrors.email[0]}</p>
          {/if}
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
            autocomplete="new-password"
            required
            aria-invalid={Boolean(fieldErrors.password?.[0])}
            bind:value={password}
          />
          {#if fieldErrors.password?.[0]}
            <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-error)">
              {fieldErrors.password[0]}
            </p>
          {/if}
        </div>

        {#if error}
          <p
            role="alert"
            class="rounded-(--radius-md) border-(length:--border-width-default) border-(color:--color-border-error) bg-(--color-background-error-subtle) px-(--space-1) py-(--space-05) text-(length:--font-size-sm) text-(--color-text-error)"
          >
            {error}
          </p>
        {/if}

        <Button type="submit" disabled={submitting} class="w-full">
          {submitting ? "Создаём аккаунт…" : "Создать аккаунт"}
        </Button>
      </form>

      <p class="mt-(--space-2) text-center text-(length:--font-size-sm) text-(--color-text-muted)">
        Уже есть аккаунт?
        <a
          href={resolve("/login")}
          class="font-medium text-(--color-text-link) transition-colors duration-(--duration-fast) hover:text-(--color-text-link-hover)"
        >
          Войти
        </a>
      </p>
    </div>
  </div>
</div>
