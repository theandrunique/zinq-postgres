<script lang="ts">
  import { resolve } from "$app/paths";
  import { Plus, Settings, Trash2 } from "@lucide/svelte";
  import {
    Avatar,
    Button,
    Dialog,
    DropdownMenu,
    Input,
    Loader,
    Select,
    Slider,
    Switch,
    Textarea,
    Tooltip,
  } from "#lib/components/ui/index.ts";

  const sections = [
    { id: "foundations", label: "Foundations" },
    { id: "button", label: "Button" },
    { id: "input", label: "Input" },
    { id: "textarea", label: "Textarea" },
    { id: "select", label: "Select" },
    { id: "switch", label: "Switch" },
    { id: "slider", label: "Slider" },
    { id: "avatar", label: "Avatar" },
    { id: "loader", label: "Loader" },
    { id: "tooltip", label: "Tooltip" },
    { id: "dialog", label: "Dialog" },
    { id: "dropdown-menu", label: "DropdownMenu" },
    { id: "playground", label: "Playground" },
  ];

  const roles = [
    { value: "developer", label: "Разработчик" },
    { value: "designer", label: "Дизайнер" },
    { value: "manager", label: "Менеджер" },
    { value: "unavailable", label: "Недоступно", disabled: true },
  ];

  const demoAvatar =
    "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='96' height='96'%3E%3Cdefs%3E%3ClinearGradient id='g' x1='0' y1='0' x2='1' y2='1'%3E%3Cstop offset='0' stop-color='%239147ff'/%3E%3Cstop offset='1' stop-color='%23fa1ed2'/%3E%3C/linearGradient%3E%3C/defs%3E%3Crect width='96' height='96' fill='url(%23g)'/%3E%3Ctext x='50%25' y='58%25' text-anchor='middle' font-family='sans-serif' font-size='40' font-weight='700' fill='white'%3EZ%3C/text%3E%3C/svg%3E";

  const semanticGroups = [
    {
      title: "Background",
      tokens: [
        "--color-background-body",
        "--color-background-surface",
        "--color-background-surface-raised",
        "--color-background-surface-sunken",
        "--color-background-surface-hover",
        "--color-background-interactive",
        "--color-background-interactive-hover",
        "--color-background-interactive-active",
        "--color-background-control",
        "--color-background-overlay",
        "--color-background-brand",
        "--color-background-brand-hover",
        "--color-background-brand-active",
        "--color-background-disabled",
        "--color-background-error",
        "--color-background-error-subtle",
        "--color-background-warn",
        "--color-background-warn-subtle",
        "--color-background-success",
        "--color-background-success-subtle",
        "--color-background-info",
        "--color-background-info-subtle",
      ],
    },
    {
      title: "Text",
      tokens: [
        "--color-text-base",
        "--color-text-muted",
        "--color-text-subtle",
        "--color-text-inverse",
        "--color-text-disabled",
        "--color-text-placeholder",
        "--color-text-brand",
        "--color-text-link",
        "--color-text-link-hover",
        "--color-text-on-brand",
        "--color-text-error",
        "--color-text-warn",
        "--color-text-success",
        "--color-text-info",
      ],
    },
    {
      title: "Border",
      tokens: [
        "--color-border-base",
        "--color-border-strong",
        "--color-border-subtle",
        "--color-border-focus",
        "--color-border-brand",
        "--color-border-error",
        "--color-border-warn",
        "--color-border-success",
        "--color-border-info",
        "--color-border-disabled",
      ],
    },
    {
      title: "Fill",
      tokens: ["--color-fill-base", "--color-fill-muted", "--color-fill-brand"],
    },
  ];

  const palettes = ["grey", "purple", "red", "orange", "green", "blue"];
  const paletteSteps = Array.from({ length: 15 }, (_, index) => index + 1);
  const fontSizes = ["xs", "sm", "md", "lg", "xl", "2xl", "3xl"];
  const spaces = ["05", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
  const radii = ["none", "sm", "md", "lg", "xl", "2xl", "full"];
  const shadows = ["elevation-1", "elevation-2", "elevation-3", "elevation-4", "elevation-5", "popover"];

  const allTokens = [
    ...semanticGroups.flatMap((group) => group.tokens),
    ...palettes.flatMap((palette) => paletteSteps.map((step) => `--color-${palette}-${step}`)),
    ...fontSizes.map((size) => `--font-size-${size}`),
    ...spaces.map((space) => `--space-${space}`),
    ...radii.map((radius) => `--radius-${radius}`),
    ...shadows.map((shadow) => `--shadow-${shadow}`),
  ];

  const demoCardClass =
    "rounded-(--radius-lg) border-(length:--border-width-default) border-(color:--color-border-base) bg-(--color-background-surface) p-(--space-2)";
  const demoLabelClass = "mb-(--space-1) font-mono text-(length:--font-size-xs) text-(--color-text-muted)";
  const fieldLabelClass = "mb-(--space-05) block text-(length:--font-size-sm) font-medium text-(--color-text-base)";

  let computed = $state<Record<string, string>>({});

  $effect(() => {
    const root = getComputedStyle(document.documentElement);
    const next: Record<string, string> = {};
    for (const token of allTokens) next[token] = root.getPropertyValue(token).trim();
    computed = next;
  });

  function contrastColor(value: string): string {
    const hex = value.match(/^#([0-9a-f]{6})$/i);
    if (!hex) return "var(--color-white)";

    const [r, g, b] = [0, 2, 4].map((index) => parseInt(hex[1].slice(index, index + 2), 16));
    const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;

    return luminance > 0.6 ? "var(--color-grey-1)" : "var(--color-white)";
  }

  let inputValue = $state("zinq");
  let textareaValue = $state("Привет, Zinq!");

  let selectEmpty = $state<string | undefined>(undefined);
  let selectValue = $state("designer");

  let switchBasic = $state(false);
  let switchChecked = $state(true);
  let switchDisabled = $state(false);
  let switchCheckedDisabled = $state(true);

  let sliderBasic = $state(40);
  let sliderCustom = $state(3);
  let sliderDisabled = $state(70);

  let name = $state("");
  let bio = $state("");
  let role = $state("developer");
  let notifications = $state(true);
  let volume = $state(65);

  const playgroundState = $derived({ name, bio, role, notifications, volume });

  function resetPlayground() {
    name = "";
    bio = "";
    role = "developer";
    notifications = true;
    volume = 65;
  }
</script>

<div class="min-h-screen bg-(--color-background-body) text-(--color-text-base)">
  <header class="border-b border-(color:--color-border-base) bg-(--color-background-surface)">
    <div
      class="mx-auto flex max-w-6xl flex-wrap items-center justify-between gap-(--space-2) px-(--space-3) py-(--space-2)"
    >
      <div>
        <h1 class="text-(length:--font-size-xl) font-semibold tracking-tight">UI Kit</h1>
        <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">
          Витрина компонентов, токенов и состояний
        </p>
      </div>

      <div class="flex flex-wrap items-center gap-(--space-1)">
        <a
          href={resolve("/")}
          class="rounded-(--button-radius) px-(--button-padding-x) py-(--button-padding-y) text-(length:--font-size-sm) font-medium text-(--color-text-link) hover:text-(--color-text-link-hover)"
        >
          Назад
        </a>
      </div>
    </div>
  </header>

  <div class="mx-auto grid max-w-6xl gap-(--space-3) px-(--space-3) py-(--space-3) lg:grid-cols-[220px_1fr]">
    <aside class="lg:sticky lg:top-(--space-2) lg:self-start">
      <nav aria-label="Разделы" class="flex flex-wrap gap-(--space-05) lg:flex-col">
        {#each sections as section (section.id)}
          <a
            href={`#${section.id}`}
            class="rounded-(--radius-md) px-(--space-1) py-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted) hover:bg-(--color-background-interactive-hover) hover:text-(--color-text-base)"
          >
            {section.label}
          </a>
        {/each}
      </nav>
    </aside>

    <main class="flex min-w-0 flex-col gap-(--space-4)">
      <section id="foundations" class="scroll-mt-(--space-3)">
        <h2 class="text-(length:--font-size-lg) font-semibold">Foundations</h2>
        <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">
          Семантические токены и шкалы. Значения обновляются при смене темы.
        </p>

        <div class="mt-(--space-2) flex flex-col gap-(--space-2)">
          {#each semanticGroups as group (group.title)}
            <div
              class="rounded-(--radius-xl) border-(length:--border-width-default) border-(color:--color-border-base) bg-(--color-background-surface) p-(--space-2)"
            >
              <h3 class="text-(length:--font-size-md) font-semibold">{group.title}</h3>
              <div class="mt-(--space-1) grid gap-(--space-1) sm:grid-cols-2 lg:grid-cols-3">
                {#each group.tokens as token (token)}
                  <div
                    class="rounded-(--radius-md) border-(length:--border-width-default) border-(color:--color-border-subtle) p-(--space-1)"
                  >
                    <div class="h-8 w-full rounded-(--radius-sm)" style={`background: var(${token})`}></div>
                    <p
                      class="mt-(--space-05) font-mono text-(length:--font-size-xs) break-all text-(--color-text-muted)"
                    >
                      {token}
                    </p>
                    <p class="font-mono text-(length:--font-size-xs) break-all text-(--color-text-subtle)">
                      {computed[token] ?? "…"}
                    </p>
                  </div>
                {/each}
              </div>
            </div>
          {/each}

          <div
            class="rounded-(--radius-xl) border-(length:--border-width-default) border-(color:--color-border-base) bg-(--color-background-surface) p-(--space-2)"
          >
            <h3 class="text-(length:--font-size-md) font-semibold">Palette</h3>
            <div class="mt-(--space-1) flex flex-col gap-(--space-1)">
              {#each palettes as palette (palette)}
                <div>
                  <p class="mb-(--space-05) font-mono text-(length:--font-size-xs) text-(--color-text-muted)">
                    {palette}
                  </p>
                  <div class="grid grid-cols-[repeat(15,minmax(0,1fr))] gap-px overflow-hidden rounded-(--radius-sm)">
                    {#each paletteSteps as step (step)}
                      {@const token = `--color-${palette}-${step}`}
                      <div
                        class="flex h-7 items-center justify-center text-(length:--font-size-xs) font-medium"
                        style={`background: var(${token}); color: ${contrastColor(computed[token] ?? "")}`}
                        title={`${token}: ${computed[token] ?? ""}`}
                      >
                        {step}
                      </div>
                    {/each}
                  </div>
                </div>
              {/each}
            </div>
          </div>

          <div
            class="rounded-(--radius-xl) border-(length:--border-width-default) border-(color:--color-border-base) bg-(--color-background-surface) p-(--space-2)"
          >
            <h3 class="text-(length:--font-size-md) font-semibold">Typography</h3>
            <div class="mt-(--space-1) flex flex-col">
              {#each fontSizes as size (size)}
                {@const token = `--font-size-${size}`}
                <div
                  class="flex flex-wrap items-baseline justify-between gap-(--space-1) border-b border-(color:--color-border-subtle) py-(--space-1) last:border-b-0"
                >
                  <span style={`font-size: var(${token})`}>Zinq UI</span>
                  <span class="font-mono text-(length:--font-size-xs) text-(--color-text-muted)">
                    {token}: {computed[token] ?? "…"}
                  </span>
                </div>
              {/each}
            </div>
          </div>

          <div
            class="rounded-(--radius-xl) border-(length:--border-width-default) border-(color:--color-border-base) bg-(--color-background-surface) p-(--space-2)"
          >
            <h3 class="text-(length:--font-size-md) font-semibold">Spacing</h3>
            <div class="mt-(--space-1) flex flex-col gap-(--space-1)">
              {#each spaces as space (space)}
                {@const token = `--space-${space}`}
                <div class="flex items-center gap-(--space-1)">
                  <span class="w-24 shrink-0 font-mono text-(length:--font-size-xs) text-(--color-text-muted)">
                    {token}
                  </span>
                  <div
                    class="h-3 rounded-(--radius-sm) bg-(--color-background-brand)"
                    style={`width: var(${token})`}
                  ></div>
                  <span class="font-mono text-(length:--font-size-xs) text-(--color-text-subtle)">
                    {computed[token] ?? "…"}
                  </span>
                </div>
              {/each}
            </div>
          </div>

          <div
            class="rounded-(--radius-xl) border-(length:--border-width-default) border-(color:--color-border-base) bg-(--color-background-surface) p-(--space-2)"
          >
            <h3 class="text-(length:--font-size-md) font-semibold">Radius</h3>
            <div class="mt-(--space-1) flex flex-wrap gap-(--space-2)">
              {#each radii as radius (radius)}
                {@const token = `--radius-${radius}`}
                <div class="text-center">
                  <div
                    class="h-16 w-16 border-(length:--border-width-default) border-(color:--color-border-strong) bg-(--color-background-surface-raised)"
                    style={`border-radius: var(${token})`}
                  ></div>
                  <p class="mt-(--space-05) font-mono text-(length:--font-size-xs) text-(--color-text-muted)">
                    {token}
                  </p>
                </div>
              {/each}
            </div>
          </div>

          <div
            class="rounded-(--radius-xl) border-(length:--border-width-default) border-(color:--color-border-base) bg-(--color-background-surface) p-(--space-2)"
          >
            <h3 class="text-(length:--font-size-md) font-semibold">Shadow</h3>
            <div class="mt-(--space-1) grid gap-(--space-2) sm:grid-cols-2 lg:grid-cols-3">
              {#each shadows as shadow (shadow)}
                {@const token = `--shadow-${shadow}`}
                <div
                  class="rounded-(--radius-lg) bg-(--color-background-surface-raised) p-(--space-2)"
                  style={`box-shadow: var(${token})`}
                >
                  <p class="font-mono text-(length:--font-size-xs) text-(--color-text-muted)">{token}</p>
                </div>
              {/each}
            </div>
          </div>
        </div>
      </section>

      <section id="button" class="scroll-mt-(--space-3)">
        <h2 class="text-(length:--font-size-lg) font-semibold">Button</h2>
        <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">
          Варианты, иконки и disabled-состояния.
        </p>

        <div class="mt-(--space-2) grid gap-(--space-2) sm:grid-cols-2 lg:grid-cols-3">
          <div class={demoCardClass}>
            <p class={demoLabelClass}>variant="primary"</p>
            <Button>Primary</Button>
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>variant="secondary"</p>
            <Button variant="secondary">Secondary</Button>
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>variant="icon"</p>
            <Button variant="icon" aria-label="Настройки">
              <Settings class="h-(--button-icon-size) w-(--button-icon-size)" />
            </Button>
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>variant="destructive"</p>
            <Button variant="destructive">
              <Trash2 class="h-(--button-icon-size) w-(--button-icon-size)" />
              Удалить
            </Button>
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>с иконкой</p>
            <Button>
              <Plus class="h-(--button-icon-size) w-(--button-icon-size)" />
              Создать
            </Button>
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>disabled</p>
            <div class="flex flex-wrap gap-(--space-1)">
              <Button disabled>Disabled</Button>
              <Button variant="secondary" disabled>Disabled</Button>
            </div>
          </div>
        </div>
      </section>

      <section id="input" class="scroll-mt-(--space-3)">
        <h2 class="text-(length:--font-size-lg) font-semibold">Input</h2>
        <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">Поля ввода и состояния.</p>

        <div class="mt-(--space-2) grid gap-(--space-2) sm:grid-cols-2">
          <div class={demoCardClass}>
            <p class={demoLabelClass}>default / hover / focus</p>
            <Input placeholder="Введите имя" />
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>со значением</p>
            <Input bind:value={inputValue} />
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>disabled</p>
            <Input placeholder="Disabled" disabled />
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>invalid</p>
            <label for="demo-invalid" class={fieldLabelClass}>Email</label>
            <Input id="demo-invalid" placeholder="name@example.com" aria-invalid="true" />
            <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-error)">Некорректный email</p>
          </div>
        </div>
      </section>

      <section id="textarea" class="scroll-mt-(--space-3)">
        <h2 class="text-(length:--font-size-lg) font-semibold">Textarea</h2>
        <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">Многострочное поле ввода.</p>

        <div class="mt-(--space-2) grid gap-(--space-2) sm:grid-cols-2">
          <div class={demoCardClass}>
            <p class={demoLabelClass}>default / hover / focus</p>
            <Textarea placeholder="Расскажите о себе" />
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>со значением</p>
            <Textarea bind:value={textareaValue} />
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>disabled</p>
            <Textarea placeholder="Disabled" disabled />
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>invalid</p>
            <Textarea placeholder="Ошибка" aria-invalid="true" />
            <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-error)">Заполните поле</p>
          </div>
        </div>
      </section>

      <section id="select" class="scroll-mt-(--space-3)">
        <h2 class="text-(length:--font-size-lg) font-semibold">Select</h2>
        <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">
          Выпадающий список с disabled-пунктом.
        </p>

        <div class="mt-(--space-2) grid gap-(--space-2) sm:grid-cols-2">
          <div class={demoCardClass}>
            <p class={demoLabelClass}>placeholder</p>
            <Select items={roles} placeholder="Выберите роль" bind:value={selectEmpty} />
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>со значением</p>
            <Select items={roles} bind:value={selectValue} />
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>disabled</p>
            <Select items={roles} placeholder="Disabled" disabled />
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>disabled-пункт</p>
            <Select items={roles} bind:value={selectValue} />
            <p class="mt-(--space-05) text-(length:--font-size-xs) text-(--color-text-subtle)">
              В списке есть недоступный пункт.
            </p>
          </div>
        </div>
      </section>

      <section id="switch" class="scroll-mt-(--space-3)">
        <h2 class="text-(length:--font-size-lg) font-semibold">Switch</h2>
        <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">Переключатели и состояния.</p>

        <div class="mt-(--space-2) grid gap-(--space-2) sm:grid-cols-2">
          <div class={demoCardClass}>
            <p class={demoLabelClass}>unchecked</p>
            <div class="flex items-center gap-(--space-1)">
              <Switch bind:checked={switchBasic} />
              <span class="text-(length:--font-size-sm) text-(--color-text-muted)">
                {switchBasic ? "Включено" : "Выключено"}
              </span>
            </div>
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>checked</p>
            <div class="flex items-center gap-(--space-1)">
              <Switch bind:checked={switchChecked} />
              <span class="text-(length:--font-size-sm) text-(--color-text-muted)">
                {switchChecked ? "Включено" : "Выключено"}
              </span>
            </div>
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>disabled</p>
            <div class="flex items-center gap-(--space-1)">
              <Switch bind:checked={switchDisabled} disabled />
              <Switch bind:checked={switchCheckedDisabled} disabled />
            </div>
          </div>
        </div>
      </section>

      <section id="slider" class="scroll-mt-(--space-3)">
        <h2 class="text-(length:--font-size-lg) font-semibold">Slider</h2>
        <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">
          Ползунки и кастомные диапазоны.
        </p>

        <div class="mt-(--space-2) grid gap-(--space-2) sm:grid-cols-2">
          <div class={demoCardClass}>
            <p class={demoLabelClass}>default</p>
            <Slider bind:value={sliderBasic} />
            <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">
              Значение: {sliderBasic}
            </p>
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>min=0 max=10 step=1</p>
            <Slider bind:value={sliderCustom} min={0} max={10} step={1} />
            <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">
              Значение: {sliderCustom}
            </p>
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>disabled</p>
            <Slider bind:value={sliderDisabled} disabled />
            <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">
              Значение: {sliderDisabled}
            </p>
          </div>
        </div>
      </section>

      <section id="avatar" class="scroll-mt-(--space-3)">
        <h2 class="text-(length:--font-size-lg) font-semibold">Avatar</h2>
        <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">
          Изображение, fallback и размер.
        </p>

        <div class="mt-(--space-2) grid gap-(--space-2) sm:grid-cols-3">
          <div class={demoCardClass}>
            <p class={demoLabelClass}>image</p>
            <Avatar src={demoAvatar} alt="Демо-аватар" fallback="ZQ" />
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>fallback</p>
            <Avatar src={null} alt="Без изображения" fallback="ZN" />
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>size override</p>
            <Avatar
              src={null}
              alt="Большой аватар"
              fallback="UI"
              style="height: var(--space-7); width: var(--space-7)"
            />
          </div>
        </div>
      </section>

      <section id="loader" class="scroll-mt-(--space-3)">
        <h2 class="text-(length:--font-size-lg) font-semibold">Loader</h2>
        <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">Индикаторы загрузки.</p>

        <div class="mt-(--space-2) grid gap-(--space-2) sm:grid-cols-3">
          <div class={demoCardClass}>
            <p class={demoLabelClass}>size="sm"</p>
            <Loader size="sm" />
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>size="md"</p>
            <Loader size="md" />
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>size="lg"</p>
            <Loader size="lg" />
          </div>
        </div>
      </section>

      <section id="tooltip" class="scroll-mt-(--space-3)">
        <h2 class="text-(length:--font-size-lg) font-semibold">Tooltip</h2>
        <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">
          Стороны и контентные подсказки.
        </p>

        <div class="mt-(--space-2) grid gap-(--space-2) sm:grid-cols-2 lg:grid-cols-3">
          {#snippet sideTrigger()}
            <Button variant="secondary">Наведи</Button>
          {/snippet}

          {#snippet topContent()}
            <span>Сверху</span>
          {/snippet}
          {#snippet rightContent()}
            <span>Справа</span>
          {/snippet}
          {#snippet bottomContent()}
            <span>Снизу</span>
          {/snippet}
          {#snippet leftContent()}
            <span>Слева</span>
          {/snippet}

          <div class={demoCardClass}>
            <p class={demoLabelClass}>side="top"</p>
            <Tooltip trigger={sideTrigger} content={topContent} contentProps={{ side: "top" }} />
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>side="right"</p>
            <Tooltip trigger={sideTrigger} content={rightContent} contentProps={{ side: "right" }} />
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>side="bottom"</p>
            <Tooltip trigger={sideTrigger} content={bottomContent} contentProps={{ side: "bottom" }} />
          </div>
          <div class={demoCardClass}>
            <p class={demoLabelClass}>side="left"</p>
            <Tooltip trigger={sideTrigger} content={leftContent} contentProps={{ side: "left" }} />
          </div>

          {#snippet longTrigger()}
            <Button variant="secondary">Длинный текст</Button>
          {/snippet}
          {#snippet longContent()}
            <span>
              Это длинная подсказка с несколькими строками текста. Она ограничена по ширине и переносится, чтобы
              оставаться читаемой даже при большом объёме контента.
            </span>
          {/snippet}

          <div class={demoCardClass}>
            <p class={demoLabelClass}>длинный текст</p>
            <Tooltip trigger={longTrigger} content={longContent} />
          </div>
        </div>
      </section>

      <section id="dialog" class="scroll-mt-(--space-3)">
        <h2 class="text-(length:--font-size-lg) font-semibold">Dialog</h2>
        <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">
          Обычный и вложенный диалог.
        </p>

        <div class="mt-(--space-2) grid gap-(--space-2) sm:grid-cols-2">
          <div class={demoCardClass}>
            <p class={demoLabelClass}>default</p>
            <Dialog.Root>
              <Dialog.Trigger>
                {#snippet child({ props })}
                  <Button {...props}>Открыть диалог</Button>
                {/snippet}
              </Dialog.Trigger>
              <Dialog.Portal>
                <Dialog.Overlay />
                <Dialog.Content>
                  <Dialog.Title>Заголовок диалога</Dialog.Title>
                  <p class="text-(length:--font-size-sm) text-(--color-text-muted)">
                    Это пример диалога. Закройте его клавишей Esc, кликом по фону или кнопкой справа сверху.
                  </p>
                  <Dialog.Close />
                </Dialog.Content>
              </Dialog.Portal>
            </Dialog.Root>
          </div>

          <div class={demoCardClass}>
            <p class={demoLabelClass}>nested</p>
            <Dialog.Root>
              <Dialog.Trigger>
                {#snippet child({ props })}
                  <Button {...props} variant="secondary">Открыть внешний</Button>
                {/snippet}
              </Dialog.Trigger>
              <Dialog.Portal>
                <Dialog.Overlay />
                <Dialog.Content>
                  <Dialog.Title>Внешний диалог</Dialog.Title>
                  <p class="text-(length:--font-size-sm) text-(--color-text-muted)">Из него можно открыть вложенный.</p>
                  <div class="mt-(--space-2)">
                    <Dialog.Root>
                      <Dialog.Trigger>
                        {#snippet child({ props })}
                          <Button {...props} variant="secondary">Открыть вложенный</Button>
                        {/snippet}
                      </Dialog.Trigger>
                      <Dialog.Portal>
                        <Dialog.Overlay />
                        <Dialog.Content>
                          <Dialog.Title>Вложенный диалог</Dialog.Title>
                          <p class="text-(length:--font-size-sm) text-(--color-text-muted)">
                            Вложенный диалог смещён и уменьшен через --bits-dialog-nested-count.
                          </p>
                          <Dialog.Close />
                        </Dialog.Content>
                      </Dialog.Portal>
                    </Dialog.Root>
                  </div>
                  <Dialog.Close />
                </Dialog.Content>
              </Dialog.Portal>
            </Dialog.Root>
          </div>
        </div>
      </section>

      <section id="dropdown-menu" class="scroll-mt-(--space-3)">
        <h2 class="text-(length:--font-size-lg) font-semibold">DropdownMenu</h2>
        <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">Меню с disabled-пунктом.</p>

        <div class="mt-(--space-2) grid gap-(--space-2) sm:grid-cols-2">
          <div class={demoCardClass}>
            <p class={demoLabelClass}>items + disabled</p>
            <DropdownMenu.Root>
              <DropdownMenu.Trigger>
                {#snippet child({ props })}
                  <Button {...props} variant="secondary">Открыть меню</Button>
                {/snippet}
              </DropdownMenu.Trigger>
              <DropdownMenu.Portal>
                <DropdownMenu.Content>
                  <DropdownMenu.Item>Профиль</DropdownMenu.Item>
                  <DropdownMenu.Item>Настройки</DropdownMenu.Item>
                  <DropdownMenu.Item disabled>Недоступно</DropdownMenu.Item>
                </DropdownMenu.Content>
              </DropdownMenu.Portal>
            </DropdownMenu.Root>
          </div>
        </div>
      </section>

      <section id="playground" class="scroll-mt-(--space-3)">
        <h2 class="text-(length:--font-size-lg) font-semibold">Playground</h2>
        <p class="mt-(--space-05) text-(length:--font-size-sm) text-(--color-text-muted)">
          Интерактивная форма и live-состояние.
        </p>

        <div class="mt-(--space-2) grid gap-(--space-2) lg:grid-cols-2">
          <form
            class="flex flex-col gap-(--space-2) rounded-(--radius-xl) border-(length:--border-width-default) border-(color:--color-border-base) bg-(--color-background-surface) p-(--space-2)"
            onsubmit={(event) => event.preventDefault()}
          >
            <div>
              <label for="pg-name" class={fieldLabelClass}>Имя</label>
              <Input id="pg-name" bind:value={name} placeholder="Как вас зовут?" />
            </div>

            <div>
              <label for="pg-bio" class={fieldLabelClass}>О себе</label>
              <Textarea id="pg-bio" bind:value={bio} placeholder="Пара слов о себе" />
            </div>

            <div>
              <span class={fieldLabelClass}>Роль</span>
              <Select items={roles} bind:value={role} />
            </div>

            <div class="flex items-center justify-between gap-(--space-2)">
              <span class="text-(length:--font-size-sm) text-(--color-text-base)">Уведомления</span>
              <Switch bind:checked={notifications} />
            </div>

            <div>
              <div class="flex items-center justify-between">
                <span class="text-(length:--font-size-sm) text-(--color-text-base)">Громкость</span>
                <span class="text-(length:--font-size-sm) text-(--color-text-muted)">{volume}</span>
              </div>
              <Slider bind:value={volume} />
            </div>

            <div class="flex flex-wrap gap-(--space-1)">
              <Button type="submit">Сохранить</Button>
              <Button type="button" variant="secondary" onclick={resetPlayground}>Сбросить</Button>
            </div>
          </form>

          <div
            class="rounded-(--radius-xl) border-(length:--border-width-default) border-(color:--color-border-base) bg-(--color-background-surface) p-(--space-2)"
          >
            <p class="mb-(--space-1) font-mono text-(length:--font-size-xs) text-(--color-text-muted)">state</p>
            <pre
              class="overflow-auto rounded-(--radius-md) bg-(--color-background-surface-sunken) p-(--space-2) font-mono text-(length:--font-size-xs) text-(--color-text-base)">{JSON.stringify(
                playgroundState,
                null,
                2,
              )}</pre>
          </div>
        </div>
      </section>
    </main>
  </div>
</div>
