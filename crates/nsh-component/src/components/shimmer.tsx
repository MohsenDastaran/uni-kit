// A theme-aware loading shimmer: a clipped highlight sweeps across the
// placeholder. The overlay runs as a native value, so the motion stays on the
// UI thread; NSH's CSS has no mask, so the parent clips the gradient instead.
import type { DimensionValue, JSX, StyleProp, TextStyle, ViewStyle } from 'nsh';
import { effect, cleanup, Text, View } from 'nsh';
import { Canvas } from 'nsh/canvas';
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
  const flat: any = Array.isArray(props.style) ? Object.assign({}, ...props.style.filter(Boolean)) : props.style;
  const fontSize = flat?.fontSize ?? FONT_SIZE_SM;
  const weight = flat?.fontWeight ?? '400';
  const family = flat?.fontFamily ?? 'Inter, sans-serif';
  const lineHeight = Math.ceil(fontSize * 1.6);
  const highlight = props.highlightColor ?? tokens.foreground;
  const base = tokens.mutedForeground;
  const half = Math.max(0.04, Math.min(0.45, props.spread ?? 0.18));

  if (!text) {
    return <Text style={[{ fontSize: FONT_SIZE_SM, color: base }, props.style]}>{props.children}</Text>;
  }

  return (
    <Canvas
      accessibilityLabel={text}
      style={{ width: '100%', height: lineHeight }}
      draw={(ctx, size) => {
        ctx.font = `${weight} ${fontSize}px ${family}`;
        ctx.textBaseline = 'middle';
        ctx.textAlign = 'left';
        const y = size.height / 2;
        const textWidth = ctx.measureText(text).width;
        ctx.fillStyle = base;
        ctx.fillText(text, 0, y);
        const band = Math.max(18, textWidth * half * 2);
        const bandX = phase.interpolate({
          inputRange: [0, 1],
          outputRange: props.direction === 'right' ? [textWidth, -band] : [-band, textWidth],
          extrapolate: 'clamp',
        });
        ctx.save();
        // Move the band and its local gradient together, then draw the text
        // back at its original position. The gradient fades at both edges so
        // the highlight never begins or ends on a hard vertical seam.
        ctx.translate(bandX, 0);
        const shine = ctx.createLinearGradient(0, 0, band, 0);
        shine.addColorStop(0, 'transparent');
        shine.addColorStop(0.32, alpha(highlight, 0.55));
        shine.addColorStop(0.5, highlight);
        shine.addColorStop(0.68, alpha(highlight, 0.55));
        shine.addColorStop(1, 'transparent');
        ctx.beginPath();
        ctx.rect(0, 0, band, size.height);
        ctx.clip();
        ctx.fillStyle = shine;
        ctx.fillText(text, bandX.mul(-1), y);
        ctx.restore();
      }}
    />
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
