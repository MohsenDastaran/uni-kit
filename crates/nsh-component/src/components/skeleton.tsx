// A loading placeholder: a text line, a circle or a rectangle, optionally
// pulsing. Composes into the article/card/table shapes the catalog shows.
import type { DimensionValue, JSX, StyleProp, ViewStyle } from 'nsh';
import { View } from 'nsh';
import { RADIUS_MD, type ControlSize, SPACING_SM, tokens } from './theme';

export type SkeletonShape = 'text' | 'circle' | 'rect';

export interface SkeletonProps {
  shape?: SkeletonShape;
  width?: DimensionValue;
  height?: number;
  lines?: number;
  size?: ControlSize;
  animated?: boolean;
  secondary?: boolean;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const heightOf = (size: ControlSize): number =>
  size === 'xsmall' ? 8 : size === 'small' ? 10 : size === 'large' ? 16 : 12;

export function Skeleton(props: SkeletonProps) {
  const shape = () => props.shape ?? 'text';
  const base = () => (props.secondary ? tokens.muted : tokens.accent);
  const lineHeight = () => props.height ?? heightOf(props.size ?? 'medium');
  const pulse = (): ViewStyle =>
    props.animated === false
      ? {}
      : {
          animationName: 'pulse',
          animationDuration: 1400,
          animationIterationCount: 'infinite',
          animationTimingFunction: 'ease-in-out',
        };
  if (shape() === 'text' && (props.lines ?? 1) > 1) {
    return (
      <View style={[{ gap: SPACING_SM }, props.style]} testID={props.testID}>
        {Array.from({ length: props.lines ?? 1 }, (_, index) => (
          <Skeleton
            shape="text"
            height={lineHeight()}
            width={(index === (props.lines ?? 1) - 1 ? '65%' : props.width ?? '100%') as DimensionValue}
            animated={props.animated}
            secondary={props.secondary}
          />
        ))}
      </View>
    );
  }
  return (
    <View
      testID={props.testID}
      style={[
        {
          width: shape() === 'circle' ? props.height ?? 40 : props.width ?? '100%',
          height: shape() === 'circle' ? props.height ?? 40 : lineHeight(),
          borderRadius: shape() === 'circle' ? (props.height ?? 40) / 2 : RADIUS_MD,
          backgroundColor: base(),
          ...pulse(),
        },
        props.style,
      ]}
    />
  );
}

export function SkeletonText(props: { lines?: number; width?: DimensionValue; style?: StyleProp<ViewStyle> }) {
  return <Skeleton shape="text" lines={props.lines ?? 3} width={props.width} style={props.style} />;
}

export function SkeletonCircle(props: { size?: number; style?: StyleProp<ViewStyle> }) {
  return <Skeleton shape="circle" height={props.size ?? 40} style={props.style} />;
}

export function SkeletonRect(props: { width?: DimensionValue; height?: number; style?: StyleProp<ViewStyle> }) {
  return <Skeleton shape="rect" width={props.width} height={props.height} style={props.style} />;
}
