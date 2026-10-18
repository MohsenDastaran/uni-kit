// An inline callout for attention: a status tint, icon, title and description.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { alpha, FONT_SIZE_SM, RADIUS_MD, SPACING_MD, SPACING_SM, tokens } from './theme';

export type AlertVariant = 'default' | 'info' | 'success' | 'warning' | 'danger';

export interface AlertProps {
  title?: string;
  description?: string;
  variant?: AlertVariant;
  icon?: IconName;
  closable?: boolean;
  onClose?: () => void;
  banner?: boolean;
  action?: JSX.Element;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const iconOf = (variant: AlertVariant): IconName =>
  variant === 'success' ? 'circle-check' : variant === 'warning' ? 'triangle-alert' : variant === 'danger' ? 'circle-alert' : 'info';

const toneOf = (variant: AlertVariant): string =>
  variant === 'success' ? tokens.success : variant === 'warning' ? tokens.warning : variant === 'danger' ? tokens.danger : tokens.info;

export function Alert(props: AlertProps) {
  const variant = () => props.variant ?? 'default';
  const tone = () => (variant() === 'default' ? tokens.foreground : toneOf(variant()));
  return (
    <View
      testID={props.testID}
      accessibilityRole="alert"
      style={[
        {
          flexDirection: 'row',
          alignItems: 'flex-start',
          gap: SPACING_SM,
          paddingHorizontal: SPACING_MD,
          paddingVertical: 10,
          borderRadius: props.banner ? 0 : RADIUS_MD,
          borderWidth: 1,
          borderColor: alpha(tone(), 0.4),
          backgroundColor: variant() === 'default' ? tokens.muted : alpha(tone(), 0.12),
        },
        props.style,
      ]}
    >
      <Icon name={props.icon ?? iconOf(variant())} size={16} color={tone()} />
      <View style={{ flex: 1, gap: 2 }}>
        {props.title ? (
          <Text style={{ fontSize: FONT_SIZE_SM, fontWeight: '600', color: tokens.foreground }}>{props.title}</Text>
        ) : null}
        {props.description ? (
          <Text style={{ fontSize: FONT_SIZE_SM, color: tokens.mutedForeground }}>{props.description}</Text>
        ) : null}
        {props.children}
      </View>
      {props.action}
      {props.closable ? (
        <View accessibilityRole="button" accessibilityLabel="Dismiss" onPress={props.onClose} style={{ width: 18, height: 18, alignItems: 'center', justifyContent: 'center' }}>
          <Icon name="x" size={12} color={tokens.mutedForeground} />
        </View>
      ) : null}
    </View>
  );
}
