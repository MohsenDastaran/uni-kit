// A theme-aware loading shimmer: a clipped highlight sweeps across the
// placeholder. The overlay runs as a native value, so the motion stays on the
// UI thread; NSH's CSS has no mask, so the parent clips the gradient instead.
import type { DimensionValue, JSX, StyleProp, TextStyle, ViewStyle } from 'nsh';
import { effect, cleanup, Text, View } from 'nsh';
import { Animated, createNativeValue, Easing } from 'nsh/animated';
import { reduceMotion } from 'nsh/accessibility';
import { alpha, FONT_SIZE_SM, RADIUS_MD, tokens } from './theme';

export interface ShimmerProps {
  /** Milliseconds for one sweep. */
  duration?: number;
  /** Which way the highlight travels. */
  direction?: 'left' | 'right';
  /** Stop after one sweep instead of repeating. */
  playOnce?: boolean;
  /** The highlight colour; defaults to a translucent foreground. */
  highlightColor?: string;
  /** The highlight band's width in points. */
  spread?: number;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Shimmer(props: ShimmerProps) {
  const travel = createNativeValue(0);
  effect(() => {
    if (reduceMotion()) {
      travel.set(0.5);
      return;
    }
    const duration = props.duration ?? 1400;
    const timing = Animated.timing(travel, {
      toValue: 1,
      duration,
      easing: Easing.linear,
    });
    if (props.playOnce) {
      timing.start();
      cleanup(() => timing.stop());
      return;
    }
    const loop = Animated.loop(timing, { iterations: -1 });
    loop.start();
    cleanup(() => loop.stop());
  });
  const width = Math.max(40, props.spread ?? 140);
  const x = travel.interpolate({
    inputRange: [0, 1],
    outputRange: props.direction === 'right' ? [640, -width] : [-width, 640],
  });
  return (
    <View
      testID={props.testID}
      style={[
        { position: 'relative', overflow: 'hidden' },
        props.style,
      ]}
    >
      {props.children}
      <View
        style={{
          position: 'absolute',
          top: 0,
          bottom: 0,
          width,
          backgroundImage: `linear-gradient(90deg, transparent, ${props.highlightColor ?? alpha(tokens.foreground, 0.12)}, transparent)`,
          transform: [{ translateX: x }],
        }}
      />
    </View>
  );
}

export function ShimmerText(props: {
  children?: JSX.Element;
  duration?: number;
  direction?: 'left' | 'right';
  highlightColor?: string;
  /** Half the highlight band's width, as a fraction of the text (0–1). */
  spread?: number;
  /** Stop after one sweep instead of repeating. */
  playOnce?: boolean;
  style?: StyleProp<TextStyle>;
}) {
  const phase = createNativeValue(0);
  effect(() => {
    if (reduceMotion()) {
      phase.set(0.5);
      return;
    }
    const timing = Animated.timing(phase, {
      toValue: 1,
      duration: props.duration ?? 1400,
      easing: Easing.linear,
    });
    if (props.playOnce) {
      timing.start();
      cleanup(() => timing.stop());
      return;
    }
    const loop = Animated.loop(timing, { iterations: -1 });
    loop.start();
    cleanup(() => loop.stop());
  });
  const text = typeof props.children === 'string' ? props.children : '';
  const chars = Array.from(text);
  const highlight = props.highlightColor ?? tokens.foreground;
  const base = tokens.mutedForeground;
  const half = Math.max(0.03, Math.min(0.45, props.spread ?? 0.18));
  const colors = chars.map((_, index) => {
    const raw = chars.length <= 1 ? 0.5 : index / (chars.length - 1);
    const at = props.direction === 'right' ? 1 - raw : raw;
    const points = [...new Set([0, at, Math.max(0, at - half), Math.min(1, at + half), 1])].sort((a, b) => a - b);
    return phase.interpolate({
      inputRange: points.length > 1 ? points : [0, 1],
      outputRange: (points.length > 1 ? points : [0, 1]).map((point) => (point === at ? highlight : base)),
      extrapolate: 'clamp',
    });
  });
  if (!text) {
    return (
      <Text style={[{ fontSize: FONT_SIZE_SM, color: base }, props.style]}>
        {props.children}
      </Text>
    );
  }
  return (
    <Text style={[{ fontSize: FONT_SIZE_SM, color: base }, props.style]}>
      {chars.map((char, index) => (
        <Text style={{ color: colors[index] }}>{char === ' ' ? '\u00a0' : char}</Text>
      ))}
    </Text>
  );
}

export function ShimmerBlock(props: {
  width?: DimensionValue;
  height?: number;
  duration?: number;
  direction?: 'left' | 'right';
  playOnce?: boolean;
  highlightColor?: string;
  spread?: number;
  style?: StyleProp<ViewStyle>;
}) {
  return (
    <Shimmer
      duration={props.duration}
      direction={props.direction}
      playOnce={props.playOnce}
      highlightColor={props.highlightColor}
      spread={props.spread}
      style={props.style}
    >
      <View
        style={{
          width: props.width ?? '100%',
          height: props.height ?? 12,
          borderRadius: RADIUS_MD,
          backgroundColor: tokens.muted,
        }}
      />
    </Shimmer>
  );
}
