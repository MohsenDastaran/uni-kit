// The reactive face of the design tokens.
//
// `tokens.ts` holds the two palettes as plain data. This file makes one object
// whose getters read the appearance in force, so a component writes
// `tokens.background` in JSX and the expression follows a light/dark change
// without rebuilding the component. Do not spread this object (that would read
// every getter once) and do not destructure it outside JSX.
import { colorScheme } from 'nsh/device';
import { DARK, LIGHT, type Tokens } from './tokens';

/** Whether the app is showing its dark palette. Reading it subscribes. */
export const isDark = (): boolean => colorScheme() !== 'light';

/** `"light"` or `"dark"`, as `nsh.json` and the gallery player name them. */
export const appearance = (): 'light' | 'dark' => (isDark() ? 'dark' : 'light');

/**
 * An object with `dark`'s keys whose getters read `dark` or `light` by the
 * appearance shown. Every read in a reactive scope (JSX, a `draw`, an effect)
 * follows appearance changes, and values spread into props keep the getters.
 */
function themed<T extends object>(dark: T, light: T): T {
  const out = {} as T;
  for (const key of Object.keys(dark) as (keyof T)[]) {
    Object.defineProperty(out, key, {
      enumerable: true,
      get: () => (isDark() ? dark : light)[key],
    });
  }
  return out;
}

export const tokens: Tokens = themed<Tokens>(DARK, LIGHT);

export {
  DARK,
  LIGHT,
  FONT_SIZE_LG,
  FONT_SIZE_MD,
  FONT_SIZE_MONO,
  FONT_SIZE_SM,
  FONT_SIZE_XL,
  FONT_SIZE_XS,
  MONO,
  RADIUS_LG,
  RADIUS_MD,
  RADIUS_SM,
  RADIUS_XL,
  SPACING_LG,
  SPACING_MD,
  SPACING_SM,
  SPACING_XL,
  SPACING_XS,
  SPACING_XXL,
  SPACING_XXS,
  alpha,
  controlFontSize,
  controlHeight,
  controlPadding,
  hovered,
  iconSize,
  inputFontSize,
  inputHeight,
  inputPaddingX,
  inkOn,
  labelFontSize,
  mix,
  pad,
  pressed,
  tint,
  type ControlSize,
  type Status,
  type Tokens,
} from './tokens';
