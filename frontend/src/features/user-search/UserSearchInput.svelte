<script lang="ts">
  import { createQuery } from "@tanstack/svelte-query";
  import { Avatar, Input, Loader } from "$lib/components/ui";
  import { searchUsersQueryOptions } from "$lib/queries/users/queries.ts";
  import type { PublicUser } from "$lib/types";
  import { cn, debounced } from "$lib/utils";

  interface Props {
    id?: string;
    query?: string;
    excludeIds?: string[];
    placeholder?: string;
    disabled?: boolean;
    autofocus?: boolean;
    onSelect: (user: PublicUser) => void;
  }

  let {
    id,
    query = $bindable(""),
    excludeIds = [],
    placeholder = "Search by username…",
    disabled = false,
    autofocus = false,
    onSelect,
  }: Props = $props();

  let wrapper = $state<HTMLDivElement | null>(null);
  let open = $state(false);
  let activeIndex = $state(-1);

  const debouncedQuery = debounced(() => query);
  const resultsQuery = createQuery(() => searchUsersQueryOptions(debouncedQuery.current));

  let results = $derived.by(() => {
    const excluded = new Set(excludeIds);
    return (resultsQuery.data ?? []).filter((user) => !excluded.has(user.id));
  });

  let showDropdown = $derived(open && query.trim().length > 0);
  let isStale = $derived(debouncedQuery.current.trim() !== query.trim());

  $effect(() => {
    if (activeIndex < results.length) return;

    activeIndex = results.length === 0 ? -1 : results.length - 1;
  });

  $effect(() => {
    if (!showDropdown || activeIndex < 0) return;

    const option = wrapper?.querySelector<HTMLElement>(`[data-user-index="${activeIndex}"]`);
    option?.scrollIntoView({ block: "nearest" });
  });

  function initials(name: string): string {
    return (
      name
        .trim()
        .split(/\s+/)
        .filter(Boolean)
        .slice(0, 2)
        .map((part) => part[0]?.toUpperCase() ?? "")
        .join("") || "?"
    );
  }

  function focusInput() {
    wrapper?.querySelector<HTMLInputElement>("input")?.focus();
  }

  function select(user: PublicUser) {
    onSelect(user);
    query = "";
    open = false;
    activeIndex = -1;
    focusInput();
  }

  function handleInput() {
    open = true;
    activeIndex = -1;
  }

  function handleFocus() {
    if (query.trim().length > 0) open = true;
  }

  function handleFocusOut(event: FocusEvent) {
    const next = event.relatedTarget as Node | null;

    if (next && wrapper?.contains(next)) return;

    open = false;
  }

  function handleKeydown(event: KeyboardEvent) {
    if (!showDropdown) return;

    switch (event.key) {
      case "ArrowDown":
        event.preventDefault();
        if (results.length === 0) return;
        activeIndex = activeIndex < 0 ? 0 : (activeIndex + 1) % results.length;
        break;

      case "ArrowUp":
        event.preventDefault();
        if (results.length === 0) return;
        activeIndex = activeIndex <= 0 ? results.length - 1 : activeIndex - 1;
        break;

      case "Home":
        event.preventDefault();
        if (results.length === 0) return;
        activeIndex = 0;
        break;

      case "End":
        event.preventDefault();
        if (results.length === 0) return;
        activeIndex = results.length - 1;
        break;

      case "Enter": {
        const user = results[activeIndex] ?? results[0];
        if (!user) return;
        event.preventDefault();
        select(user);
        break;
      }

      case "Escape":
        event.preventDefault();
        event.stopPropagation();
        open = false;
        break;

      case "Tab":
        open = false;
        break;
    }
  }
</script>

<div class="relative" bind:this={wrapper}>
  <Input
    {id}
    bind:value={query}
    {placeholder}
    {disabled}
    {autofocus}
    autocomplete="off"
    maxlength={32}
    oninput={handleInput}
    onfocus={handleFocus}
    onfocusout={handleFocusOut}
    onkeydown={handleKeydown}
  />

  {#if showDropdown}
    <div
      class="absolute inset-x-0 top-full z-50 mt-1 max-h-60 overflow-y-auto rounded-(--menu-radius) bg-(--menu-background) p-(--menu-padding) shadow-(--menu-shadow)"
    >
      {#if resultsQuery.isError}
        <div class="flex flex-col gap-1 px-(--menu-item-padding-x) py-(--menu-item-padding-y)">
          <p class="text-(length:--font-size-sm) text-(--color-text-muted)">Couldn't load users.</p>
          <button
            type="button"
            class="self-start text-(length:--font-size-sm) font-medium text-(--color-text-link) hover:text-(--color-text-link-hover)"
            onclick={() => resultsQuery.refetch()}
          >
            Retry
          </button>
        </div>
      {:else if results.length === 0 && (resultsQuery.isPending || resultsQuery.isFetching || isStale)}
        <div class="flex items-center gap-2 px-(--menu-item-padding-x) py-(--menu-item-padding-y)">
          <Loader size="sm" />
          <span class="text-(length:--font-size-sm) text-(--color-text-muted)">Searching…</span>
        </div>
      {:else if results.length === 0}
        <p
          class="px-(--menu-item-padding-x) py-(--menu-item-padding-y) text-(length:--font-size-sm) text-(--color-text-muted)"
        >
          No users found.
        </p>
      {:else}
        {#each results as user, index (user.id)}
          <button
            type="button"
            data-user-index={index}
            class={cn(
              "flex w-full items-center gap-2 rounded-(--menu-item-radius) px-(--menu-item-padding-x) py-(--menu-item-padding-y) text-left",
              index === activeIndex && "bg-(--menu-item-background-hover)",
            )}
            onmousedown={(event) => event.preventDefault()}
            onmouseenter={() => (activeIndex = index)}
            onclick={() => select(user)}
          >
            <Avatar
              src={user.avatar}
              alt={user.display_name}
              fallback={initials(user.display_name || user.username)}
              class="h-8 w-8 shrink-0"
            />
            <span class="min-w-0">
              <span class="block truncate text-(length:--menu-item-font-size) font-medium text-(--menu-item-text)">
                {user.display_name}
              </span>
              <span class="block truncate text-(length:--font-size-xs) text-(--color-text-muted)">@{user.username}</span
              >
            </span>
          </button>
        {/each}
      {/if}
    </div>
  {/if}
</div>
