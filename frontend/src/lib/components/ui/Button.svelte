<script lang="ts">
  import { Button } from "bits-ui";
  import { cva, type VariantProps } from "class-variance-authority";
  import { cn } from "#lib/utils";

  const buttonVariants = cva(
    [
      "inline-flex items-center justify-center gap-(--button-gap)",
      "min-h-(--button-height) rounded-(--button-radius) px-(--button-padding-x) py-(--button-padding-y)",
      "text-(length:--button-font-size) font-(weight:--button-font-weight)",
      "transition-colors duration-(--duration-fast)",
      "focus-visible:focus-ring",
      "disabled:cursor-not-allowed disabled:opacity-(--button-opacity-disabled)",
    ],
    {
      variants: {
        variant: {
          primary: [
            "bg-(--button-background-primary) text-(--button-text-primary)",
            "enabled:hover:bg-(--button-background-primary-hover)",
            "enabled:active:bg-(--button-background-primary-active)",
          ],
          secondary: [
            "bg-(--button-background-secondary) text-(--button-text-secondary)",
            "enabled:hover:bg-(--button-background-secondary-hover)",
            "enabled:active:bg-(--button-background-secondary-active)",
          ],
          icon: [
            "bg-(--button-background-icon) p-(--button-padding-y) text-(--button-text-icon)",
            "enabled:hover:bg-(--button-background-icon-hover)",
            "enabled:active:bg-(--button-background-icon-active)",
          ],
          destructive: [
            "bg-(--button-background-error) text-(--button-text-error)",
            "enabled:hover:bg-(--button-background-error-hover)",
            "enabled:active:bg-(--button-background-error-active)",
          ],
        },
      },
      defaultVariants: {
        variant: "primary",
      },
    },
  );

  type Variant = VariantProps<typeof buttonVariants>["variant"];
  type Props = Button.RootProps & {
    variant?: Variant;
  };

  let { variant = "primary", class: className = "", children, ...rest }: Props = $props();

  let classes = $derived(cn(buttonVariants({ variant }), className));
</script>

<Button.Root class={classes} {...rest}>
  {@render children?.()}
</Button.Root>
