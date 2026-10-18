// Lucide icons by name, drawn with Canvas and Path2D.
//
// NSH's Image element takes raster images, not SVG, so the catalog's icon set is
// converted to path data at build time (`icons.ts`) and replayed on a Canvas.
// The name, colour and size props match the Slint and GPUI icons: a component
// that draws `search` in the other frameworks draws the same glyph here.
import type { StyleProp, ViewStyle } from 'nsh';
import { Canvas, Path2D } from 'nsh/canvas';
import { tokens } from './theme';
import { ICONS, type IconName } from './icons';

export interface IconProps {
  /** A Lucide icon name from the catalog's set. */
  name: IconName | string;
  /** Points. 16 is the default control icon box. */
  size?: number;
  /** The stroke or fill colour. Defaults to the theme foreground. */
  color?: string;
  /** Rotates continuously; use with `loader-circle`. */
  spinning?: boolean;
  /** Extra styles for the Canvas box. */
  style?: StyleProp<ViewStyle>;
  testID?: string;
  accessibilityLabel?: string;
}

export function Icon(props: IconProps) {
  const size = () => props.size ?? 16;
  const color = () => props.color ?? tokens.foreground;

  return (
    <Canvas
      testID={props.testID}
      accessibilityRole="image"
      accessibilityLabel={props.accessibilityLabel ?? props.name}
      style={[
        {
          width: size(),
          height: size(),
          ...(props.spinning
            ? {
                animationName: 'spin',
                animationDuration: 1000,
                animationIterationCount: 'infinite',
                animationTimingFunction: 'linear',
              }
            : {}),
        },
        props.style,
      ]}
      draw={(ctx) => {
        const shapes = ICONS[props.name] ?? ICONS.circle ?? [];
        const scale = size() / 24;
        ctx.save();
        ctx.scale(scale, scale);
        ctx.lineWidth = 2;
        ctx.lineCap = 'round';
        ctx.lineJoin = 'round';
        for (const shape of shapes) {
          const path = new Path2D(shape.d);
          if (shape.fill) {
            ctx.fillStyle = color();
            ctx.fill(path);
          } else {
            ctx.strokeStyle = color();
            ctx.stroke(path);
          }
        }
        ctx.restore();
      }}
    />
  );
}
