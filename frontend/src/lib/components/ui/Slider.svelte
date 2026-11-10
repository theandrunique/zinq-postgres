<script lang="ts">
  import { Slider } from "bits-ui";
  import { cn } from "#lib/utils";

  type Props = {
    value?: number;
    min?: number;
    max?: number;
    step?: number;
    disabled?: boolean;
    class?: string;
  };

  let { value = $bindable(0), min = 0, max = 100, step = 1, disabled = false, class: className }: Props = $props();

  let rangeClasses = $derived(
    cn("absolute h-full rounded-(--slider-radius) bg-(--slider-range-background)", className),
  );

  let thumbClasses = $derived(
    cn(
      "block rounded-(--slider-radius)",
      "h-(--slider-thumb-size) w-(--slider-thumb-size)",
      "border-(length:--slider-border-width) border-(color:--slider-thumb-border)",
      "bg-(--slider-thumb-background) shadow-(--slider-thumb-shadow)",
      "focus-visible:focus-ring",
      "data-[disabled]:cursor-not-allowed data-[disabled]:opacity-(--slider-opacity-disabled)",
    ),
  );
</script>

<Slider.Root bind:value {min} {max} {step} {disabled} type="single" class="relative flex w-full items-center">
  <div class="relative h-(--slider-track-height) w-full rounded-(--slider-radius) bg-(--slider-track-background)">
    <Slider.Range class={rangeClasses} />
  </div>
  <Slider.Thumb index={0} class={thumbClasses} />
</Slider.Root>
