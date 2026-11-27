export function debounced<T>(value: () => T, delay = 300) {
  let debouncedValue = $state($state.snapshot(value()));

  $effect(() => {
    const next = $state.snapshot(value());
    const timeout = setTimeout(() => (debouncedValue = next), delay);
    return () => clearTimeout(timeout);
  });

  return {
    get current() {
      return debouncedValue;
    },
  };
}
