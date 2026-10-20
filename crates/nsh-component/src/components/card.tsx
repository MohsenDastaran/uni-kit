// A bordered content surface: optional cover, badges, title, description,
// body, actions and footer. The ReUI card language is a quiet elevated fill,
// a compact header, clipped media and a clear footer boundary.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Image } from 'nsh/image';
import {
  alpha,
  FONT_SIZE_MD,
  FONT_SIZE_SM,
  RADIUS_LG,
  RADIUS_MD,
  SPACING_SM,
  SPACING_XS,
  type ControlSize,
  tokens,
} from './theme';

export type CardCover = string | { uri: string };
export type CardVariant = 'default' | 'muted' | 'ghost';
export type CardFooterAlign = 'start' | 'center' | 'end' | 'between';

export interface CardProps {
  title?: string;
  description?: string;
  /** A compact text badge above the title. */
  badge?: string;
  /** A caller-drawn badge row; wins over the simple `badge` text. */
  badges?: JSX.Element;
  cover?: CardCover;
  /** The cover's height in points. Defaults to 160. */
  coverHeight?: number;
  /** A layer over the cover, typically a gradient with a title. */
  coverOverlay?: JSX.Element;
  /** Scale the cover image on hover. */
  imageHoverScale?: boolean;
  /** Top-right header actions, such as a dropdown trigger. */
  actions?: JSX.Element;
  size?: ControlSize;
  /** Padding, and the gap between the header, body and footer. */
  inset?: number;
  variant?: CardVariant;
  footer?: JSX.Element;
  footerAlign?: CardFooterAlign;
  footerBordered?: boolean;
  /** Makes the whole surface a pressable card. */
  onPress?: () => void;
  accessibilityLabel?: string;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  contentStyle?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Card(props: CardProps) {
  const inset = () => props.inset ?? (props.size === 'small' ? 12 : 16);
  const titleSize = () => (props.size === 'small' ? FONT_SIZE_SM : FONT_SIZE_MD);
  const variant = () => props.variant ?? 'default';
  const surface = () =>
    variant() === 'muted' ? tokens.muted : variant() === 'ghost' ? 'transparent' : tokens.popover;
  const justify = () =>
    props.footerAlign === 'start'
      ? 'flex-start'
      : props.footerAlign === 'center'
        ? 'center'
        : props.footerAlign === 'between'
          ? 'space-between'
          : 'flex-end';

  return (
    <View
      testID={props.testID}
      accessibilityRole={props.onPress ? 'button' : undefined}
      accessibilityLabel={props.onPress ? props.accessibilityLabel ?? props.title ?? 'Card' : undefined}
      onPress={props.onPress}
      activeOpacity={props.onPress ? 0.9 : 1}
      class="group transition-shadow duration-200 ease-out hover:shadow-md"
      style={[
        {
          backgroundColor: surface(),
          borderWidth: variant() === 'ghost' ? 0 : 1,
          borderColor: tokens.border,
          borderRadius: RADIUS_LG,
          overflow: 'hidden',
          alignSelf: 'stretch',
        },
        props.style,
      ]}
    >
      {props.cover ? (
        <View
          style={{
            position: 'relative',
            width: '100%',
            height: props.coverHeight ?? 160,
            overflow: 'hidden',
            backgroundColor: tokens.muted,
          }}
        >
          <View
            class={props.imageHoverScale ? 'transition-transform duration-300 ease-out hover:scale-[1.04]' : undefined}
            style={{ width: '100%', height: '100%' }}
          >
            <Image
              src={props.cover}
              resizeMode="cover"
              style={{ width: '100%', height: '100%' }}
            />
          </View>
          {props.coverOverlay ? (
            <View
              style={{
                position: 'absolute',
                left: 0,
                right: 0,
                bottom: 0,
                padding: 16,
                backgroundImage: `linear-gradient(to top, ${alpha(tokens.background, 0.9)}, ${alpha(tokens.background, 0)})`,
              }}
            >
              {props.coverOverlay}
            </View>
          ) : null}
        </View>
      ) : null}
      <View style={[{ padding: inset(), gap: inset() }, props.contentStyle]}>
        {props.badges || props.badge || props.title || props.description || props.actions ? (
          <View style={{ gap: SPACING_XS }}>
            <View style={{ flexDirection: 'row', alignItems: 'flex-start', gap: 8 }}>
              <View style={{ flex: 1, gap: SPACING_XS }}>
                {props.badges ??
                  (props.badge ? (
                    <View
                      style={{
                        alignSelf: 'flex-start',
                        paddingHorizontal: 6,
                        paddingVertical: 2,
                        borderRadius: RADIUS_MD,
                        backgroundColor: tokens.muted,
                      }}
                    >
                      <Text style={{ fontSize: FONT_SIZE_SM, color: tokens.foreground }}>{props.badge}</Text>
                    </View>
                  ) : null)}
                {props.title ? (
                  <Text style={{ fontSize: titleSize(), lineHeight: titleSize() + 6, fontWeight: '600', color: tokens.foreground }}>
                    {props.title}
                  </Text>
                ) : null}
                {props.description ? (
                  <Text style={{ fontSize: FONT_SIZE_SM, lineHeight: 20, color: tokens.mutedForeground }}>
                    {props.description}
                  </Text>
                ) : null}
              </View>
              {props.actions ? (
                <View style={{ flexDirection: 'row', alignItems: 'center', gap: 2 }}>{props.actions}</View>
              ) : null}
            </View>
          </View>
        ) : null}
        {props.children}
        {props.footer ? (
          <View
            style={{
              flexDirection: 'row',
              alignItems: 'center',
              justifyContent: justify(),
              gap: SPACING_SM,
              ...(props.footerBordered
                ? { paddingTop: SPACING_SM, borderTopWidth: 1, borderColor: tokens.border }
                : {}),
            }}
          >
            {props.footer}
          </View>
        ) : null}
      </View>
    </View>
  );
}

export function CardHeader(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return <View style={[{ gap: SPACING_XS }, props.style]}>{props.children}</View>;
}

export function CardTitle(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return (
    <Text style={[{ fontSize: FONT_SIZE_MD, fontWeight: '600', color: tokens.foreground }, props.style]}>
      {props.children}
    </Text>
  );
}

export function CardDescription(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return (
    <Text style={[{ fontSize: FONT_SIZE_SM, color: tokens.mutedForeground }, props.style]}>
      {props.children}
    </Text>
  );
}

export function CardContent(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return <View style={props.style}>{props.children}</View>;
}

export function CardFooter(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return (
    <View
      style={[
        { flexDirection: 'row', justifyContent: 'flex-end', alignItems: 'center', gap: SPACING_SM },
        props.style,
      ]}
    >
      {props.children}
    </View>
  );
}
