// Shared pieces of the gallery demos: the page frame, a section heading with
// the code chip, and the small layout helpers.
//
// The `// section:` markers in each demo file are the contract with the website
// (see `remark-nsh-source.js`); `ExampleSection` draws the heading the reader
// sees and posts the section's index/title through `bridge.ts`.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Scroll, Text, View } from 'nsh';
import { Icon } from '../components/icon';
import {
  alpha,
  FONT_SIZE_SM,
  RADIUS_MD,
  SPACING_LG,
  SPACING_MD,
  SPACING_SM,
  SPACING_XL,
  tokens,
} from '../components/theme';
import { showSource } from './bridge';

export interface PageProps {
  children?: JSX.Element;
  gap?: number;
  style?: StyleProp<ViewStyle>;
}

/** The whole demo page: a scroll view with the catalog's 24 pt gutter. */
export function Page(props: PageProps) {
  return (
    <Scroll style={[{ flex: 1, backgroundColor: 'transparent' }, props.style]}>
      <View style={{ padding: SPACING_XL, paddingBottom: 48, gap: props.gap ?? 32 }}>
        {props.children}
      </View>
    </Scroll>
  );
}

export interface ExampleSectionProps {
  index: number;
  title: string;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
}

/**
 * One documented example. The title is the page heading the sample belongs
 * under; pressing the chip asks the page to scroll to that sample.
 */
export function ExampleSection(props: ExampleSectionProps) {
  const show = () => showSource(props.index, props.title);
  // The iced gallery's heading: a 28pt box with the 16pt code glyph, the
  // primary colour at 0.12 / 0.22 / 0.34 fill and 0.35 / 0.55 / 0.7 border
  // through rest, hover and press. Tailwind's state variants are native on
  // every backend, so the same classes drive desktop and web.
  const chip =
    'flex h-7 w-7 items-center justify-center rounded-lg border-[1px] border-black/[0.35] bg-black/[0.12] ' +
    'transition-all duration-150 ease-out hover:scale-105 hover:border-black/[0.55] hover:bg-black/[0.22] hover:shadow-sm ' +
    'active:scale-95 active:border-black/[0.7] active:bg-black/[0.34] ' +
    'dark:border-white/[0.35] dark:bg-white/[0.12] dark:hover:border-white/[0.55] dark:hover:bg-white/[0.22] ' +
    'dark:active:border-white/[0.7] dark:active:bg-white/[0.34]';
  return (
    <View
      style={[
        {
          gap: SPACING_MD,
          ...(props.index > 0
            ? { borderTopWidth: 1, borderTopColor: tokens.border, paddingTop: SPACING_LG }
            : {}),
        },
        props.style,
      ]}
    >
      <View style={{ flexDirection: 'row', alignItems: 'center', gap: SPACING_SM }}>
        <View
          accessibilityRole="button"
          accessibilityLabel={`Show the ${props.title} source`}
          onPress={show}
          activeOpacity={0.7}
          class="flex-1 transition-opacity duration-150 hover:opacity-70 active:opacity-50"
        >
          <Text
            style={{
              fontSize: FONT_SIZE_SM,
              fontWeight: '500',
              color: tokens.mutedForeground,
            }}
          >
            {props.title}
          </Text>
        </View>
        <View
          testID={`show-source-${props.index}`}
          accessibilityRole="button"
          accessibilityLabel={`Show the ${props.title} source`}
          onPress={show}
          class={chip}
        >
          <Icon name="code-xml" size={16} color={tokens.primary} />
        </View>
      </View>
      {props.children}
    </View>
  );
}

/** A row that wraps, for a set of controls. */
export function Row(props: { children?: JSX.Element; gap?: number; style?: StyleProp<ViewStyle> }) {
  return (
    <View
      style={[
        { flexDirection: 'row', flexWrap: 'wrap', alignItems: 'center', gap: props.gap ?? SPACING_SM },
        props.style,
      ]}
    >
      {props.children}
    </View>
  );
}

/** A column with the catalog's section gap. */
export function Column(props: { children?: JSX.Element; gap?: number; style?: StyleProp<ViewStyle> }) {
  return (
    <View style={[{ gap: props.gap ?? SPACING_MD }, props.style]}>
      {props.children}
    </View>
  );
}

/** Muted explanatory text under a control. */
export function Caption(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return (
    <Text
      style={[
        { fontSize: 13, lineHeight: 18, color: tokens.mutedForeground },
        props.style,
      ]}
    >
      {props.children}
    </Text>
  );
}

/** The line a demo writes when a control reports something. */
export function Status(props: { children?: JSX.Element }) {
  return (
    <View
      style={{
        paddingHorizontal: SPACING_MD,
        paddingVertical: SPACING_SM,
        borderRadius: RADIUS_MD,
        backgroundColor: tokens.muted,
      }}
    >
      <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>{props.children}</Text>
    </View>
  );
}

export { SPACING_LG, SPACING_SM };
