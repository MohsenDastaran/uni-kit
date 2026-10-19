// Charts drawn with NSH's Canvas: line, area, bar, pie/donut, radar,
// candlestick and a simplified Sankey. A chart can be framed as a ReUI card
// with a title, description, delta badge and footer legend.
//
// Every drawing animates from the baseline when it first appears; the animation
// is a signal read inside `draw`, so Canvas replays the frame natively. Tapping
// a chart opens a small data popover at the top of the plot.
import type { JSX, LayoutEvent, StyleProp, ViewStyle } from 'nsh';
import { cleanup, effect, signal, Text, View } from 'nsh';
import { createNativeValue } from 'nsh/animated';
import { Canvas, type CanvasRenderingContext2D } from 'nsh/canvas';
import { reduceMotion } from 'nsh/accessibility';
import { alpha, FONT_SIZE_SM, isDark, RADIUS_LG, RADIUS_MD, SPACING_SM, SPACING_XS, tokens } from './theme';

export type ChartType = 'line' | 'area' | 'bar' | 'pie' | 'radar' | 'candlestick' | 'sankey';
export type ChartBarStyle = 'solid' | 'gradient' | 'diagonal' | 'dots' | 'slim' | 'horizontal';
export type ChartAreaStyle = 'gradient' | 'solid' | 'stacked' | 'step' | 'dots';
export type ChartDeltaTone = 'success' | 'danger' | 'info' | 'warning';

/** One legend entry: a label, its colour, and whether its series is shown. */
export interface ChartLegendItem {
  label: string;
  color?: string;
}

export interface ChartProps {
  type?: ChartType;
  data?: number[];
  /** Grouped/overlaid series for bar and area charts. */
  series?: number[][];
  labels?: string[];
  height?: number;
  color?: string;
  colors?: string[];
  showGrid?: boolean;
  /** ReUI-style chart surface. */
  card?: boolean;
  title?: string;
  description?: string;
  badge?: string;
  delta?: string;
  deltaTone?: ChartDeltaTone;
  barStyle?: ChartBarStyle;
  areaStyle?: ChartAreaStyle;
  donut?: boolean;
  centerLabel?: string;
  centerValue?: string;
  /** A legend row. Clicking an entry hides and shows its series. */
  legend?: ChartLegendItem[];
  /** Hide the built-in legend (the app can draw its own with the toggle handlers). */
  showLegend?: boolean;
  /** Reports the legend entries that are currently hidden. */
  onToggleSeries?: (label: string, hidden: boolean) => void;
  footer?: JSX.Element;
  /** Turn off the initial baseline animation. */
  animate?: boolean;
  /** Show the data popover on press/tap. */
  tooltip?: boolean;
  /** Pin the popover to a point initially, useful in demos. */
  defaultActiveIndex?: number;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const seriesColors = (): string[] => [tokens.chart1, tokens.chart2, tokens.chart3, tokens.chart4, tokens.chart5];

const format = (value: number): string =>
  Number.isInteger(value) ? String(value) : value.toFixed(1);

export function Chart(props: ChartProps) {
  const progress = signal(0);
  const active = signal(props.defaultActiveIndex ?? -1);
  // Series hidden through the legend, by label.
  const hidden = signal<string[]>([]);
  const legendHover = signal('');
  const width = signal(360);
  const height = signal(props.height ?? 220);
  const type = () => props.type ?? 'line';
  const labels = () => props.labels ?? [];
  const series = () => (props.series && props.series.length > 0 ? props.series : [props.data ?? [12, 28, 18, 36, 24, 42, 30]]);
  const colors = () => props.colors ?? seriesColors();
  const maxValue = () =>
    Math.max(
      1,
      ...series().map((values) => Math.max(...values)),
      ...(type() === 'pie' ? [series()[0].reduce((sum, value) => sum + value, 0)] : []),
    );
  const deltaTone = () => {
    switch (props.deltaTone) {
      case 'danger':
        return tokens.danger;
      case 'warning':
        return tokens.warning;
      case 'info':
        return tokens.info;
      default:
        return tokens.success;
    }
  };

  effect(() => {
    if (props.animate === false || reduceMotion()) {
      progress.set(1);
      return;
    }
    let start = 0;
    let raf = 0;
    const tick = (time: number) => {
      if (!start) start = time;
      const t = Math.min(1, (time - start) / 650);
      progress.set(1 - Math.pow(1 - t, 3));
      if (t < 1) raf = requestAnimationFrame(tick);
    };
    raf = requestAnimationFrame(tick);
    cleanup(() => cancelAnimationFrame(raf));
  });

  /** The series currently drawn: legend-hidden labels are left out. */
  const visibleSeries = () => {
    const all = series();
    const off = hidden();
    if (off.length === 0) return all;
    return all.filter((_, index) => !off.includes(props.legend?.[index]?.label ?? `Series ${index + 1}`));
  };
  const isHidden = (label: string) => hidden().includes(label);
  const toggleLegend = (label: string) => {
    const off = hidden();
    const next = off.includes(label) ? off.filter((item) => item !== label) : [...off, label];
    // Never hide the last visible series.
    const count = (props.legend ?? []).length || series().length;
    if (next.length >= count) return;
    hidden.set(next);
    props.onToggleSeries?.(label, next.includes(label));
  };

  const activePoint = () => {
    const index = active();
    if (index < 0) return null;
    const sets = visibleSeries();
    const label = labels()[index] ?? `Point ${index + 1}`;
    // Legend labels name the series in the popover when there is a legend.
    const names = (props.legend ?? []).map((item) => item.label);
    return {
      label,
      values: sets.map((set) => set[index] ?? 0),
      names: sets.map((_, at) => names[at] ?? `Series ${at + 1}`),
      index,
    };
  };

  /**
   * Hover slides the popover: on entry/move it points at the column or slice
   * under the pointer, and leaving clears it (unless a default is pinned).
   * NSH's `onHoverIn` carries no position, so the column is read from the
   * pointer when hovering starts and whenever the responder reports a move.
   */
  // The pointer position inside the plot. `onHoverIn` and native pan report
  // coordinates; plain hover does not, so the pan probe below keeps the last
  // known point and the popover follows it.
  const pointerX = createNativeValue(0);
  const pointerY = createNativeValue(0);
  let lastX = -1;
  let lastY = -1;

  const nearestIndex = (x: number, y: number) => {
    const n = Math.max(1, series()[0].length);
    const plotLeft = type() === 'bar' && props.barStyle === 'horizontal' ? 58 : 34;
    const plotWidth = Math.max(1, width() - plotLeft - 12);
    const plotHeight = Math.max(1, height() - 36);
    let index = 0;
    if (type() === 'bar' && props.barStyle === 'horizontal') {
      index = Math.floor(((y - 12) / plotHeight) * n);
    } else if (type() === 'pie') {
      const angle = Math.atan2(y - height() / 2, x - width() / 2) + Math.PI / 2;
      const total = series()[0].reduce((sum, value) => sum + value, 0) || 1;
      let cursor = 0;
      for (let at = 0; at < n; at += 1) {
        const slice = (series()[0][at] / total) * Math.PI * 2;
        if (angle >= cursor && angle < cursor + slice) {
          index = at;
          break;
        }
        cursor += slice;
      }
    } else if (type() === 'bar') {
      index = Math.floor((x - plotLeft) / (plotWidth / n));
    } else {
      index = Math.round((x - plotLeft) / (plotWidth / Math.max(1, n - 1)));
    }
    return Math.max(0, Math.min(n - 1, index));
  };

  const hoverMove = (x: number, y: number) => {
    if (props.tooltip === false) return;
    lastX = x;
    lastY = y;
    active.set(nearestIndex(x, y));
  };
  // A pan of zero distance still reports the pointer position on release, so
  // a plain hover leaves the popover at the column under the cursor on the
  // platforms that track hover but not hover coordinates.
  const hoverMoveX = (x: number) => {
    lastX = x;
    if (lastY >= 0) hoverMove(x, lastY);
  };
  const hoverMoveY = (y: number) => {
    lastY = y;
    if (lastX >= 0) hoverMove(lastX, y);
  };
  // A plain hover reports no position, so the popover centres on the nearest
  // column and the pan probe refines it once the pointer is known.
  const hoverEnter = () => {
    if (props.tooltip === false) return;
    // The pointer position is unknown on a plain hover, so the popover opens
    // on the column the pointer is most likely over and the column targets
    // below refine it. A pinned default wins.
    if (props.defaultActiveIndex !== undefined) {
      active.set(props.defaultActiveIndex);
      return;
    }
    active.set(nearestIndex(width() / 2, height() / 2));
  };


  const card = () => props.card ?? Boolean(props.title || props.description || props.badge || props.delta);

  const drawing = (ctx: CanvasRenderingContext2D, size: { width: number; height: number }) => {
    const width = size.width;
    const height = size.height;
    const p = progress();
    const left = type() === 'bar' && props.barStyle === 'horizontal' ? 58 : 34;
    const bottom = height - 24;
    const plotWidth = width - left - 12;
    const plotHeight = bottom - 12;
    const values = (visibleSeries()[0] ?? series()[0]);
    const max = maxValue();
    const barStyle = props.barStyle ?? 'solid';
    const areaStyle = props.areaStyle ?? 'gradient';
    const labelsList = labels();

    const barFill = (color: string, x: number, y: number, w: number, h: number, index: number) => {
      ctx.save();
      ctx.beginPath();
      ctx.rect(x, y, w, h);
      ctx.clip();
      if (barStyle === 'gradient') {
        const gradient = ctx.createLinearGradient(0, y, 0, y + h);
        gradient.addColorStop(0, alpha(color, 0.95));
        gradient.addColorStop(1, alpha(color, 0.3));
        ctx.fillStyle = gradient;
      } else {
        ctx.fillStyle = color;
      }
      ctx.fillRect(x, y, w, h);
      if (barStyle === 'diagonal' || barStyle === 'dots') {
        ctx.strokeStyle = alpha(tokens.background, isDark() ? 0.42 : 0.28);
        ctx.fillStyle = alpha(tokens.background, 0.35);
        ctx.lineWidth = 1;
        if (barStyle === 'diagonal') {
          for (let line = -h; line < w; line += 4) {
            ctx.beginPath();
            ctx.moveTo(x + line, y + h);
            ctx.lineTo(x + line + h, y);
            ctx.stroke();
          }
        } else {
          for (let dotX = 2; dotX < w; dotX += 5) {
            for (let dotY = 2; dotY < h; dotY += 5) {
              ctx.beginPath();
              ctx.arc(x + dotX, y + dotY, 0.8, 0, Math.PI * 2);
              ctx.fill();
            }
          }
        }
      }
      ctx.restore();
      // A quiet value marker in gradients/patterns for depth.
      if (index < 0) return;
    };

    if (props.showGrid !== false && barStyle !== 'horizontal') {
      ctx.strokeStyle = tokens.border;
      ctx.lineWidth = 1;
      for (let step = 0; step <= 4; step += 1) {
        const y = 12 + (plotHeight * step) / 4;
        ctx.beginPath();
        ctx.moveTo(left, y);
        ctx.lineTo(left + plotWidth, y);
        ctx.stroke();
      }
    }

    if (type() === 'bar') {
      if (barStyle === 'horizontal') {
        const slot = plotHeight / values.length;
        const barWidth = (plotWidth - 10) * p;
        values.forEach((value, index) => {
          const y = 12 + index * slot + slot * 0.2;
          const bar = (value / max) * (plotWidth - 10) * p;
          ctx.fillStyle = colors()[index % colors().length];
          ctx.fillRect(left, y, bar, slot * 0.55);
          if (labelsList[index]) {
            ctx.fillStyle = tokens.mutedForeground;
            ctx.font = '10px system-ui';
            ctx.textAlign = 'left';
            ctx.fillText(labelsList[index], 4, y + slot * 0.34);
          }
          void barWidth;
        });
      } else {
        const sets = visibleSeries();
        const slot = plotWidth / values.length;
        const group = slot * 0.72;
        const bar = group / Math.max(1, sets.length);
        values.forEach((value, index) => {
          sets.forEach((set, setIndex) => {
            const valueAt = set[index] ?? 0;
            const barHeight = (valueAt / max) * plotHeight * p;
            const x = left + index * slot + slot * 0.14 + setIndex * bar;
            barFill(
              colors()[setIndex % colors().length],
              x,
              bottom - barHeight,
              Math.max(1, bar * 0.84),
              barHeight,
              index,
            );
          });
        });
      }
    } else if (type() === 'pie') {
      const total = values.reduce((sum, value) => sum + value, 0) || 1;
      const cx = width / 2;
      const cy = height / 2;
      const radius = Math.min(width, height) / 2 - 18;
      const inner = props.donut ? radius * 0.58 : 0;
      let angle = -Math.PI / 2;
      values.forEach((value, index) => {
        const slice = (value / total) * Math.PI * 2 * p;
        ctx.fillStyle = colors()[index % colors().length];
        ctx.beginPath();
        if (inner > 0) ctx.arc(cx, cy, radius, angle, angle + slice);
        else ctx.moveTo(cx, cy), ctx.arc(cx, cy, radius, angle, angle + slice);
        if (inner > 0) ctx.arc(cx, cy, inner, angle + slice, angle, true);
        else ctx.lineTo(cx, cy);
        ctx.closePath();
        ctx.fill();
        angle += slice;
      });
      if (inner > 0) {
        ctx.fillStyle = tokens.foreground;
        ctx.font = '600 24px system-ui';
        ctx.textAlign = 'center';
        ctx.fillText(props.centerValue ?? String(Math.round(values.reduce((sum, value) => sum + value, 0))), cx, cy + 2);
        ctx.fillStyle = tokens.mutedForeground;
        ctx.font = '11px system-ui';
        ctx.fillText(props.centerLabel ?? 'Total', cx, cy + 18);
      }
    } else if (type() === 'radar') {
      const cx = width / 2;
      const cy = height / 2;
      const radius = Math.min(width, height) / 2 - 24;
      const count = values.length;
      ctx.strokeStyle = tokens.border;
      ctx.lineWidth = 1;
      for (let ring = 1; ring <= 4; ring += 1) {
        ctx.beginPath();
        for (let index = 0; index < count; index += 1) {
          const angle = -Math.PI / 2 + (index / count) * Math.PI * 2;
          const r = (radius * ring) / 4;
          if (index === 0) ctx.moveTo(cx + Math.cos(angle) * r, cy + Math.sin(angle) * r);
          else ctx.lineTo(cx + Math.cos(angle) * r, cy + Math.sin(angle) * r);
        }
        ctx.closePath();
        ctx.stroke();
      }
      ctx.fillStyle = alpha(colors()[0], 0.24);
      ctx.strokeStyle = colors()[0];
      ctx.lineWidth = 2;
      ctx.beginPath();
      values.forEach((value, index) => {
        const angle = -Math.PI / 2 + (index / count) * Math.PI * 2;
        const r = (value / max) * radius * p;
        if (index === 0) ctx.moveTo(cx + Math.cos(angle) * r, cy + Math.sin(angle) * r);
        else ctx.lineTo(cx + Math.cos(angle) * r, cy + Math.sin(angle) * r);
      });
      ctx.closePath();
      ctx.fill();
      ctx.stroke();
      if (labelsList.length === count) {
        ctx.fillStyle = tokens.mutedForeground;
        ctx.font = '10px system-ui';
        ctx.textAlign = 'center';
        labelsList.forEach((label, index) => {
          const angle = -Math.PI / 2 + (index / count) * Math.PI * 2;
          ctx.fillText(label, cx + Math.cos(angle) * (radius + 10), cy + Math.sin(angle) * (radius + 10) + 3);
        });
      }
    } else if (type() === 'candlestick') {
      const slot = plotWidth / values.length;
      values.forEach((value, index) => {
        const x = left + index * slot + slot / 2;
        const high = 12 + plotHeight - (Math.min(1, (value + 4) / max)) * plotHeight * p;
        const low = 12 + plotHeight - (Math.max(0, (value - 4) / max)) * plotHeight * p;
        ctx.strokeStyle = value >= (values[index - 1] ?? value) ? tokens.success : tokens.danger;
        ctx.beginPath();
        ctx.moveTo(x, low);
        ctx.lineTo(x, high);
        ctx.stroke();
        ctx.fillStyle = ctx.strokeStyle;
        ctx.fillRect(x - slot * 0.2, high + 2, slot * 0.4, Math.max(2, low - high - 4));
      });
    } else if (type() === 'sankey') {
      const slot = plotHeight / Math.max(1, values.length);
      values.forEach((value, index) => {
        const bar = (value / max) * plotWidth * 0.8 * p;
        ctx.fillStyle = colors()[index % colors().length];
        ctx.fillRect(left, 12 + index * slot + 3, bar, slot - 6);
      });
    } else {
      const sets = visibleSeries();
      const step = values.length > 1 ? plotWidth / (values.length - 1) : plotWidth;
      const point = (value: number, index: number, scale = p) => ({
        x: left + index * step,
        y: 12 + plotHeight - (value / max) * plotHeight * scale,
      });
      const strokeSet = (set: number[], index: number) => {
        ctx.strokeStyle = colors()[index % colors().length];
        ctx.lineWidth = index === 0 ? 2 : 1.5;
        ctx.lineJoin = 'round';
        ctx.beginPath();
        set.forEach((value, at) => {
          const pt = point(value, at);
          if (at === 0) ctx.moveTo(pt.x, pt.y);
          else if (areaStyle === 'step') {
            const prev = point(set[at - 1], at - 1);
            ctx.lineTo(pt.x, prev.y);
            ctx.lineTo(pt.x, pt.y);
          } else ctx.lineTo(pt.x, pt.y);
        });
        ctx.stroke();
      };
      if (type() === 'area') {
        const stacks = areaStyle === 'stacked';
        if (stacks) {
          const totals = values.map((_, index) => sets.reduce((sum, set) => sum + (set[index] ?? 0), 0));
          const stackMax = Math.max(1, ...totals);
          let lower = values.map(() => 0);
          sets.forEach((set, setIndex) => {
            const upper = set.map((value, index) => lower[index] + (value / stackMax) * plotHeight * p);
            const fill = ctx.createLinearGradient(0, 12, 0, bottom);
            fill.addColorStop(0, alpha(colors()[setIndex % colors().length], 0.42));
            fill.addColorStop(1, alpha(colors()[setIndex % colors().length], 0.08));
            ctx.fillStyle = fill;
            ctx.beginPath();
            ctx.moveTo(left, bottom);
            set.forEach((_, index) => {
              const x = left + index * step;
              const y = 12 + plotHeight - upper[index];
              ctx.lineTo(x, y);
            });
            for (let index = set.length - 1; index >= 0; index -= 1) {
              const x = left + index * step;
              const y = 12 + plotHeight - lower[index];
              ctx.lineTo(x, y);
            }
            ctx.closePath();
            ctx.fill();
            lower = upper;
          });
          sets.forEach(strokeSet);
        } else {
          const fill = ctx.createLinearGradient(0, 12, 0, bottom);
          fill.addColorStop(0, alpha(props.color ?? colors()[0], 0.34));
          fill.addColorStop(1, alpha(props.color ?? colors()[0], 0));
          ctx.fillStyle = areaStyle === 'solid' ? alpha(colors()[0], 0.18) : fill;
          ctx.beginPath();
          ctx.moveTo(left, bottom);
          values.forEach((value, index) => {
            const pt = point(value, index);
            if (areaStyle === 'step' && index > 0) {
              const prev = point(values[index - 1], index - 1);
              ctx.lineTo(pt.x, prev.y);
            }
            ctx.lineTo(pt.x, pt.y);
          });
          ctx.lineTo(left + plotWidth, bottom);
          ctx.closePath();
          ctx.fill();
          strokeSet(values, 0);
        }
        if (areaStyle === 'dots' || areaStyle === 'gradient') {
          ctx.fillStyle = colors()[0];
          values.forEach((value, index) => {
            const pt = point(value, index);
            ctx.beginPath();
            ctx.arc(pt.x, pt.y, 3, 0, Math.PI * 2);
            ctx.fill();
          });
        }
      } else {
        sets.forEach(strokeSet);
        if (type() === 'line') {
          ctx.fillStyle = colors()[0];
          values.forEach((value, index) => {
            const pt = point(value, index);
            ctx.beginPath();
            ctx.arc(pt.x, pt.y, 3, 0, Math.PI * 2);
            ctx.fill();
          });
        }
      }
    }

    if (labelsList.length > 0 && barStyle !== 'horizontal' && type() !== 'pie' && type() !== 'radar') {
      ctx.fillStyle = tokens.mutedForeground;
      ctx.font = '11px system-ui';
      ctx.textAlign = 'center';
      const slot = type() === 'bar' ? plotWidth / labelsList.length : plotWidth / Math.max(1, labelsList.length - 1);
      labelsList.forEach((label, index) => {
        ctx.fillText(label, left + index * slot + (type() === 'bar' ? slot / 2 : 0), height - 6);
      });
    }
  };

  const activeInfo = () => activePoint();

  return (
    <View
      testID={props.testID}
      style={[
        {
          gap: 8,
          ...(card()
            ? {
                padding: 12,
                borderRadius: RADIUS_LG,
                borderWidth: 1,
                borderColor: tokens.border,
                backgroundColor: tokens.popover,
              }
            : {}),
        },
        props.style,
      ]}
    >
      {card() && (props.title || props.description || props.badge || props.delta) ? (
        <View style={{ gap: 3 }}>
          <View style={{ flexDirection: 'row', alignItems: 'center', gap: 6 }}>
            {props.title ? (
              <Text style={{ flex: 1, fontSize: 14, lineHeight: 18, fontWeight: '600', color: tokens.foreground }}>
                {props.title}
              </Text>
            ) : null}
            {props.delta ? (
              <View
                style={{
                  paddingHorizontal: 5,
                  paddingVertical: 2,
                  borderRadius: 5,
                  backgroundColor: alpha(deltaTone(), 0.16),
                }}
              >
                <Text style={{ fontSize: 10, lineHeight: 13, fontWeight: '600', color: deltaTone() }}>
                  {props.delta}
                </Text>
              </View>
            ) : props.badge ? (
              <View style={{ paddingHorizontal: 6, paddingVertical: 2, borderRadius: 5, backgroundColor: tokens.muted }}>
                <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>{props.badge}</Text>
              </View>
            ) : null}
          </View>
          {props.description ? (
            <Text style={{ fontSize: 11, lineHeight: 15, color: tokens.mutedForeground }}>{props.description}</Text>
          ) : null}
        </View>
      ) : null}
      <View
        onLayout={(event: LayoutEvent) => {
          width.set(event.width);
          height.set(event.height);
        }}
        style={{ position: 'relative', width: '100%', height: props.height ?? 220 }}
      >
        <Canvas
          accessibilityRole="image"
          accessibilityLabel={`${type()} chart`}
          onHoverIn={hoverEnter}
          onHoverOut={() => active.set(props.defaultActiveIndex ?? -1)}
          onTouchStart={(event) => hoverMove(event.locationX, event.locationY)}
          style={{ width: '100%', height: '100%' }}
          draw={drawing}
        />
        {/* Invisible hover columns sit over the plot. NSH's hover carries no
            coordinates, so one target per column is what makes the popover
            follow the pointer; leaving every column clears it again. */}
        {type() !== 'pie' ? (
          <View
            style={{
              position: 'absolute',
              top: 0,
              left: 0,
              right: 0,
              bottom: 24,
              flexDirection: 'row',
              paddingStart: type() === 'bar' && props.barStyle === 'horizontal' ? 58 : 34,
              paddingEnd: 12,
            }}
          >
            {(visibleSeries()[0] ?? []).map((_, column) => (
              <View
                onHoverIn={() => {
                  if (props.tooltip !== false) active.set(column);
                }}
                onHoverOut={() => {
                  const last = (visibleSeries()[0] ?? []).length - 1;
                  if (column === last) active.set(props.defaultActiveIndex ?? -1);
                }}
                style={{ flex: 1 }}
              />
            ))}
          </View>
        ) : null}
        {activeInfo() ? (
          <View
            style={{
              position: 'absolute',
              top: 4,
              left: Math.max(
                4,
                Math.min(
                  width() - 140,
                  type() === 'bar'
                    ? 34 + ((activeInfo()!.index + 0.5) / Math.max(1, series()[0].length)) * (width() - 46) - 70
                    : 34 + (activeInfo()!.index / Math.max(1, series()[0].length - 1)) * (width() - 46) - 60,
                ),
              ),
              width: 140,
              paddingHorizontal: 9,
              paddingVertical: 7,
              gap: 3,
              borderRadius: RADIUS_MD,
              borderWidth: 1,
              borderColor: tokens.border,
              backgroundColor: tokens.background,
              zIndex: 3,
            }}
          >
            <Text style={{ fontSize: 10, lineHeight: 13, fontWeight: '600', color: tokens.foreground }}>
              {activeInfo()!.label}
            </Text>
            {activeInfo()!.values.map((value, index) => (
              <View style={{ flexDirection: 'row', alignItems: 'center', gap: 4 }}>
                <View
                  style={{
                    width: 6,
                    height: 6,
                    borderRadius: 3,
                    backgroundColor: props.legend?.[index]?.color ?? colors()[index % colors().length],
                  }}
                />
                <Text style={{ flex: 1, fontSize: 10, lineHeight: 13, color: tokens.mutedForeground }}>
                  {activeInfo()!.names[index]}
                </Text>
                <Text style={{ fontSize: 10, lineHeight: 13, fontWeight: '600', color: tokens.foreground }}>
                  {format(value)}
                </Text>
              </View>
            ))}
          </View>
        ) : null}
      </View>
      {props.legend && props.legend.length > 0 && props.showLegend !== false ? (
        <View style={{ flexDirection: 'row', flexWrap: 'wrap', alignItems: 'center', gap: 4 }}>
          {props.legend.map((item, index) => {
            const off = isHidden(item.label);
            return (
              <View
                accessibilityRole="button"
                accessibilityLabel={`${off ? 'Show' : 'Hide'} ${item.label}`}
                accessibilityState={{ selected: !off }}
                onPress={() => toggleLegend(item.label)}
                onHoverIn={() => legendHover.set(item.label)}
                onHoverOut={() => legendHover.set('')}
                activeOpacity={0.8}
                class="transition-colors duration-150 hover:bg-black/[0.04] dark:hover:bg-white/[0.06]"
                style={{
                  flexDirection: 'row',
                  alignItems: 'center',
                  gap: 5,
                  paddingHorizontal: 6,
                  paddingVertical: 3,
                  borderRadius: 5,
                  borderWidth: 1,
                  borderColor: tokens.border,
                  opacity: off ? 0.45 : 1,
                }}
              >
                <View
                  style={{
                    width: 7,
                    height: 7,
                    borderRadius: 2,
                    backgroundColor: item.color ?? colors()[index % colors().length],
                  }}
                />
                <Text
                  style={{
                    fontSize: 10,
                    lineHeight: 14,
                    fontWeight: legendHover() === item.label ? '600' : '500',
                    color: tokens.mutedForeground,
                    textDecorationLine: off ? 'line-through' : 'none',
                  }}
                >
                  {item.label}
                </Text>
              </View>
            );
          })}
        </View>
      ) : null}
      {props.footer ? (
        <View style={{ flexDirection: 'row', flexWrap: 'wrap', alignItems: 'center', gap: 10 }}>
          {props.footer}
        </View>
      ) : null}
    </View>
  );
}

export function ChartCard(props: { title: string; description?: string; children?: JSX.Element }) {
  return (
    <View
      style={{
        padding: 12,
        gap: 8,
        borderWidth: 1,
        borderColor: tokens.border,
        borderRadius: RADIUS_LG,
        backgroundColor: tokens.popover,
      }}
    >
      <View style={{ gap: 2 }}>
        <Text style={{ fontSize: FONT_SIZE_SM, fontWeight: '600', color: tokens.foreground }}>{props.title}</Text>
        {props.description ? (
          <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>{props.description}</Text>
        ) : null}
      </View>
      {props.children}
    </View>
  );
}
