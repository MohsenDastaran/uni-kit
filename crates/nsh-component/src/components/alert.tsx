// An inline callout: a quiet tinted surface, a leading icon or avatar, a title,
// a description and optional top-right actions. The ReUI alert language: no
// heavy banner, no floating glyph; the tint and border carry the severity.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Button } from './button';
import { Icon } from './icon';
import { type IconName } from './icons';
import {
  alpha,
  FONT_SIZE_SM,
  RADIUS_MD,
  SPACING_MD,
  SPACING_SM,
  inkOn,
  isDark,
  mix,
  tokens,
} from './theme';

export type AlertVariant = 'default' | 'info' | 'success' | 'warning' | 'danger';
export type AlertSize = 'small' | 'medium' | 'large';

export interface AlertProps {
  title?: string;
  description?: string;
  variant?: AlertVariant;
  icon?: IconName;
  /** A caller-drawn leading element, such as an avatar. Wins over `icon`. */
  leading?: JSX.Element;
  closable?: boolean;
  onClose?: () => void;
  banner?: boolean;
  /** A filled surface: secondary for default, the tone for coloured variants. */
  inverted?: boolean;
  size?: AlertSize;
  /** Actions shown at the top right. */
  action?: JSX.Element;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const iconOf = (variant: AlertVariant): IconName =>
  variant === 'success'
    ? 'circle-check'
    : variant === 'warning'
      ? 'triangle-alert'
      : variant === 'danger'
        ? 'circle-alert'
        : 'info';

const toneOf = (variant: AlertVariant): string =>
  variant === 'success'
    ? tokens.success
    : variant === 'warning'
      ? tokens.warning
      : variant === 'danger'
        ? tokens.danger
        : tokens.info;

const PADDING_X: Record<AlertSize, number> = { small: 10, medium: 14, large: 16 };
const PADDING_Y: Record<AlertSize, number> = { small: 8, medium: 12, large: 14 };
const TITLE_FONT: Record<AlertSize, number> = { small: 12, medium: 13, large: 14 };
const ICON_SIZE: Record<AlertSize, number> = { small: 15, medium: 17, large: 19 };

export function Alert(props: AlertProps) {
  const variant = () => props.variant ?? 'default';
  const size = () => props.size ?? 'medium';
  const inverted = () => Boolean(props.inverted);
  const tone = () => (variant() === 'default' ? tokens.foreground : toneOf(variant()));
  const ink = () => (inverted() ? (variant() === 'default' ? tokens.foreground : inkOn(tone())) : variant() === 'default' ? tokens.foreground : tone());
  const titleInk = () => {
    if (inverted()) return ink();
    if (variant() === 'default') return tokens.foreground;
    // In the light palette the semantic tones are too bright for a title on a
    // 10% tint; pull them toward the foreground. In dark mode they are already
    // luminous and keep their hue.
    return isDark() ? tone() : mix(tone(), tokens.foreground, 0.55);
  };
  const mutedInk = () => {
    if (inverted()) return alpha(ink(), 0.78);
    if (variant() === 'default') return tokens.mutedForeground;
    return isDark() ? alpha(tone(), 0.82) : mix(tone(), tokens.foreground, 0.35);
  };
  const background = () => {
    if (inverted()) return variant() === 'default' ? tokens.secondary : tone();
    return variant() === 'default' ? tokens.muted : alpha(tone(), 0.1);
  };
  const border = () => {
    if (inverted()) return variant() === 'default' ? tokens.border : alpha(tone(), 0.55);
    return variant() === 'default' ? tokens.border : alpha(tone(), 0.26);
  };
  const showIcon = () =>
    variant() !== 'default' || Boolean(props.icon) || Boolean(props.leading);

  return (
    <View
      testID={props.testID}
      accessibilityRole="alert"
      class="transition-colors duration-150"
      style={[
        {
          flexDirection: 'row',
          alignItems: 'flex-start',
          gap: SPACING_SM,
          paddingHorizontal: PADDING_X[size()],
          paddingVertical: PADDING_Y[size()],
          borderRadius: props.banner ? 0 : RADIUS_MD,
          borderWidth: 1,
          borderColor: border(),
          backgroundColor: background(),
        },
        props.style,
      ]}
    >
      {props.leading ? (
        <View style={{ width: 30, height: 30, alignItems: 'center', justifyContent: 'center' }}>
          {props.leading}
        </View>
      ) : showIcon() ? (
        <Icon
          name={props.icon ?? iconOf(variant())}
          size={ICON_SIZE[size()]}
          color={ink()}
          style={{ marginTop: 1 }}
        />
      ) : null}
      <View style={{ flex: 1, gap: 3 }}>
        {props.title ? (
          <Text
            style={{
              fontSize: TITLE_FONT[size()],
              lineHeight: TITLE_FONT[size()] + 4,
              fontWeight: '600',
              color: titleInk(),
            }}
          >
            {props.title}
          </Text>
        ) : null}
        {props.description ? (
          <Text style={{ fontSize: 12, lineHeight: 18, color: mutedInk() }}>
            {props.description}
          </Text>
        ) : null}
        {props.children}
      </View>
      {props.action ? (
        <View style={{ flexDirection: 'row', alignItems: 'center', gap: 6, flexShrink: 0 }}>
          {props.action}
        </View>
      ) : null}
      {props.closable ? (
        <Button
          variant="ghost"
          size="xsmall"
          accessibilityLabel="Dismiss"
          onPress={props.onClose}
          style={{ width: 20, height: 20, minWidth: 20, paddingHorizontal: 0, alignSelf: "flex-start" }}
        >
          <Text style={{ fontSize: 15, lineHeight: 18, color: mutedInk() }}>×</Text>
        </Button>
      ) : null}
    </View>
  );
}
