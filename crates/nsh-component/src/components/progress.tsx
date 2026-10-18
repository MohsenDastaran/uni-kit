// A linear or circular completion indicator, with the catalog's sizes,
// statuses and optional label.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Canvas } from 'nsh/canvas';
import { Text, View } from 'nsh';
import { type ControlSize, FONT_SIZE_SM, RADIUS_MD, tokens } from './theme';

export interface ProgressProps {
  value?: number;
  max?: number;
  size?: ControlSize;
  status?: 'default' | 'info' | 'success' | 'warning' | 'danger';
  label?: string;
  showValue?: boolean;
  circle?: boolean;
  innerContent?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const heightOf = (size: ControlSize): number =>
  size === 'xsmall' ? 4 : size === 'small' ? 6 : size === 'large' ? 12 : 8;

const toneOf = (status: ProgressProps['status']): string => {
  switch (status) {
    case 'info':
      return tokens.info;
    case 'success':
      return tokens.success;
    case 'warning':
      return tokens.warning;
    case 'danger':
      return tokens.danger;
    default:
      return tokens.primary;
  }
};

export function Progress(props: ProgressProps) {
  const fraction = () => Math.max(0, Math.min(1, (props.value ?? 0) / (props.max ?? 100)));
  const color = () => toneOf(props.status);
  const size = () => props.size ?? 'medium';
  if (props.circle) {
    return (
      <View
        testID={props.testID}
        accessibilityRole="progressbar"
        accessibilityLabel={props.label ?? 'Progress'}
        accessibilityValue={{ min: 0, max: props.max ?? 100, now: props.value ?? 0 }}
        style={[{ width: 96, height: 96, alignItems: 'center', justifyContent: 'center' }, props.style]}
      >
        <Canvas
          style={{ position: 'absolute', top: 0, right: 0, bottom: 0, left: 0 }}
          draw={(ctx, { width, height }) => {
            const cx = width / 2;
            const cy = height / 2;
            const radius = Math.min(width, height) / 2 - 6;
            ctx.lineWidth = 8;
            ctx.lineCap = 'round';
            ctx.strokeStyle = tokens.muted;
            ctx.beginPath();
            ctx.arc(cx, cy, radius, 0, Math.PI * 2);
            ctx.stroke();
            ctx.strokeStyle = color();
            ctx.beginPath();
            ctx.arc(cx, cy, radius, -Math.PI / 2, -Math.PI / 2 + fraction() * Math.PI * 2);
            ctx.stroke();
          }}
        />
        {props.innerContent ?? (
          <Text style={{ fontSize: FONT_SIZE_SM, fontWeight: '600', color: tokens.foreground }}>
            {props.showValue === false ? '' : `${Math.round(fraction() * 100)}%`}
          </Text>
        )}
      </View>
    );
  }
  return (
    <View testID={props.testID} style={[{ gap: 6 }, props.style]}>
      {props.label || props.showValue !== false ? (
        <View style={{ flexDirection: 'row', justifyContent: 'space-between' }}>
          <Text style={{ fontSize: FONT_SIZE_SM, color: tokens.mutedForeground }}>{props.label ?? ''}</Text>
          {props.showValue !== false ? (
            <Text style={{ fontSize: FONT_SIZE_SM, color: tokens.foreground }}>
              {Math.round(fraction() * 100)}%
            </Text>
          ) : null}
        </View>
      ) : null}
      <View
        accessibilityRole="progressbar"
        accessibilityLabel={props.label ?? 'Progress'}
        accessibilityValue={{ min: 0, max: props.max ?? 100, now: props.value ?? 0 }}
        style={{
          height: heightOf(size()),
          borderRadius: RADIUS_MD,
          backgroundColor: tokens.muted,
          overflow: 'hidden',
        }}
      >
        <View
          style={{
            width: `${fraction() * 100}%`,
            height: '100%',
            borderRadius: RADIUS_MD,
            backgroundColor: color(),
          }}
        />
      </View>
    </View>
  );
}

export function ProgressCircle(props: Omit<ProgressProps, 'circle' | 'label' | 'showValue'>) {
  return <Progress {...props} circle showValue={false} />;
}
