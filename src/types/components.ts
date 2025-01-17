interface IButtonProps {
  onClick?: () => void;
  disabled?: boolean;
  isBusy?: boolean;
}

/**
 * Available standard sizes for tailwind classes.
 *
 * NOTE: To support this dynamic way of generating class names you
 * have to extend the safelist in the tailwind.config.js.
 */
const Tailwind_Size = {
  0: 0,
  1: 1,
  2: 2,
  3: 3,
  4: 4,
  5: 5,
  6: 6,
  7: 7,
  8: 8,
  9: 9,
  10: 10,
  11: 11,
  12: 12,
  14: 14,
  16: 16,
  20: 20,
  24: 24,
  28: 28,
  32: 32,
  36: 36,
  40: 40,
  44: 44,
  48: 48,
  52: 52,
  56: 56,
  60: 60,
  64: 64,
  72: 72,
  80: 80,
  96: 96,
} as const;
type TailwindSize = keyof typeof Tailwind_Size;

export type { IButtonProps, TailwindSize };
