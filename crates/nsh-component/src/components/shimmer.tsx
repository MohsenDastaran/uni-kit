// A theme-aware loading shimmer. NSH has no CSS mask, so this pulses the
// element's opacity — the same motion with the platform's own keyframes.
import type { DimensionValue, JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { FONT_SIZE_SM, RADIUS_MD, tokens } from './theme';

export interface ShimmerProps {
  duration?: number;
  direction?: 'left' | 'right';
  playOnce?: boolean;
  highlightColor?: string;
  spread?: number;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Shimmer(props: ShimmerProps) {
  const animation = {
    animationName: 'pulse' as const,
    animationDuration: props.duration ?? 1400,
    animationIterationCount: props.playOnce ? 1 : ('infinite' as const),
    animationTimingFunction: 'ease-in-out' as const,
  };
  return (
    <View testID={props.testID} style={[{ opacity: 1 }, animation, props.style]}>
      {props.children}
    </View>
  );
}

export function ShimmerText(props: {
  children?: JSX.Element;
  duration?: number;
  style?: StyleProp<ViewStyle>;
}) {
  return (
    <Shimmer duration={props.duration}>
      <Text style={[{ fontSize: FONT_SIZE_SM, color: tokens.mutedForeground }, props.style]}>
        {props.children}
      </Text>
    </Shimmer>
  );
}

export function ShimmerBlock(props: { width?: DimensionValue; height?: number; style?: StyleProp<ViewStyle> }) {
  return (
    <Shimmer>
      <View
        style={[
          {
            width: props.width ?? '100%',
            height: props.height ?? 12,
            borderRadius: RADIUS_MD,
            backgroundColor: tokens.muted,
          },
          props.style,
        ]}
      />
    </Shimmer>
  );
}
