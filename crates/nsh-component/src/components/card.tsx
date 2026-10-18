// A bordered content surface: an optional cover, badge, title and description
// over a body, with an optional footer.
//
// Mirrors GPUI Component's `Card` and the Slint version beside it. Unlike the
// mobile `Card` in `nsh/ui/mobile`, this one carries a cover, a description and
// the catalog's size/inset scale, so it is built from Views and Text rather
// than wrapping the built-in. The built-in Card is still used where its
// header/footer surface is the right shape (settings, empty states).
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Image } from 'nsh/image';
import {
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

export interface CardProps {
  title?: string;
  description?: string;
  badge?: string;
  cover?: CardCover;
  size?: ControlSize;
  /** Padding, and the gap between the header, body and footer. */
  inset?: number;
  footer?: JSX.Element;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Card(props: CardProps) {
  const inset = () => props.inset ?? (props.size === 'small' ? 12 : 16);
  const titleSize = () => (props.size === 'small' ? FONT_SIZE_SM : FONT_SIZE_MD);

  return (
    <View
      testID={props.testID}
      class="transition-shadow duration-200 ease-out hover:shadow-md"
      style={[
        {
          backgroundColor: tokens.background,
          borderWidth: 1,
          borderColor: tokens.border,
          borderRadius: RADIUS_LG,
          overflow: 'hidden',
          alignSelf: 'stretch',
        },
        props.style,
      ]}
    >
      {props.cover ? (
        <Image
          src={props.cover}
          resizeMode="cover"
          style={{ width: '100%', height: 160 }}
        />
      ) : null}
      <View style={{ padding: inset(), gap: inset() }}>
        {props.badge || props.title || props.description ? (
          <View style={{ gap: SPACING_XS }}>
            {props.badge ? (
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
            ) : null}
            {props.title ? (
              <Text
                style={{
                  fontSize: titleSize(),
                  fontWeight: '600',
                  color: tokens.foreground,
                }}
              >
                {props.title}
              </Text>
            ) : null}
            {props.description ? (
              <Text style={{ fontSize: FONT_SIZE_SM, color: tokens.mutedForeground }}>
                {props.description}
              </Text>
            ) : null}
          </View>
        ) : null}
        {props.children}
        {props.footer ? (
          <View style={{ flexDirection: 'row', justifyContent: 'flex-end', gap: SPACING_SM }}>
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
