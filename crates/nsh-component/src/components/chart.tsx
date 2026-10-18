// Charts drawn with NSH's Canvas: line, area, bar, pie, radar, candlestick.
//
// The catalog's chart page also documents a Sankey; this port draws its flows as
// stacked bars and records that simplification in docs/nsh-integration.md.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Canvas } from 'nsh/canvas';
import { alpha, tokens } from './theme';

export type ChartType = 'line' | 'area' | 'bar' | 'pie' | 'radar' | 'candlestick' | 'sankey';

export interface ChartProps {
  type?: ChartType;
  data?: number[];
  labels?: string[];
  height?: number;
  color?: string;
  colors?: string[];
  showGrid?: boolean;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const seriesColors = (): string[] => [tokens.chart1, tokens.chart2, tokens.chart3, tokens.chart4, tokens.chart5];

export function Chart(props: ChartProps) {
  const type = () => props.type ?? 'line';
  const data = () => props.data ?? [12, 28, 18, 36, 24, 42, 30];
  const colors = () => props.colors ?? seriesColors();
  return (
    <Canvas
      testID={props.testID}
      accessibilityRole="image"
      accessibilityLabel={`${type()} chart`}
      style={[{ width: '100%', height: props.height ?? 220 }, props.style]}
      draw={(ctx, { width, height }) => {
        const values = data();
        const max = Math.max(1, ...values);
        const left = 34;
        const bottom = height - 24;
        const plotWidth = width - left - 12;
        const plotHeight = bottom - 12;
        ctx.lineWidth = 1;
        if (props.showGrid !== false) {
          ctx.strokeStyle = tokens.border;
          for (let step = 0; step <= 4; step += 1) {
            const y = 12 + (plotHeight * step) / 4;
            ctx.beginPath();
            ctx.moveTo(left, y);
            ctx.lineTo(left + plotWidth, y);
            ctx.stroke();
          }
        }
        if (type() === 'bar') {
          const slot = plotWidth / values.length;
          values.forEach((value, index) => {
            const bar = (value / max) * plotHeight;
            ctx.fillStyle = colors()[index % colors().length];
            ctx.fillRect(left + index * slot + slot * 0.15, bottom - bar, slot * 0.7, bar);
          });
        } else if (type() === 'pie') {
          const total = values.reduce((sum, value) => sum + value, 0) || 1;
          const cx = width / 2;
          const cy = height / 2;
          const radius = Math.min(width, height) / 2 - 18;
          let angle = -Math.PI / 2;
          values.forEach((value, index) => {
            const slice = (value / total) * Math.PI * 2;
            ctx.fillStyle = colors()[index % colors().length];
            ctx.beginPath();
            ctx.moveTo(cx, cy);
            ctx.arc(cx, cy, radius, angle, angle + slice);
            ctx.closePath();
            ctx.fill();
            angle += slice;
          });
        } else if (type() === 'radar') {
          const cx = width / 2;
          const cy = height / 2;
          const radius = Math.min(width, height) / 2 - 22;
          const count = values.length;
          ctx.strokeStyle = tokens.border;
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
          ctx.strokeStyle = colors()[0];
          ctx.lineWidth = 2;
          ctx.beginPath();
          values.forEach((value, index) => {
            const angle = -Math.PI / 2 + (index / count) * Math.PI * 2;
            const r = (value / max) * radius;
            if (index === 0) ctx.moveTo(cx + Math.cos(angle) * r, cy + Math.sin(angle) * r);
            else ctx.lineTo(cx + Math.cos(angle) * r, cy + Math.sin(angle) * r);
          });
          ctx.closePath();
          ctx.stroke();
        } else if (type() === 'candlestick') {
          const slot = plotWidth / values.length;
          values.forEach((value, index) => {
            const x = left + index * slot + slot / 2;
            const high = 12 + plotHeight - (Math.min(1, (value + 4) / max)) * plotHeight;
            const low = 12 + plotHeight - (Math.max(0, (value - 4) / max)) * plotHeight;
            ctx.strokeStyle = value >= (values[index - 1] ?? value) ? tokens.success : tokens.danger;
            ctx.beginPath();
            ctx.moveTo(x, high);
            ctx.lineTo(x, low);
            ctx.stroke();
            ctx.fillStyle = ctx.strokeStyle;
            ctx.fillRect(x - slot * 0.2, high + 2, slot * 0.4, Math.max(2, low - high - 4));
          });
        } else if (type() === 'sankey') {
          const slot = plotHeight / Math.max(1, values.length);
          values.forEach((value, index) => {
            const bar = (value / max) * plotWidth * 0.8;
            ctx.fillStyle = colors()[index % colors().length];
            ctx.fillRect(left, 12 + index * slot + 3, bar, slot - 6);
          });
        } else {
          const step = values.length > 1 ? plotWidth / (values.length - 1) : plotWidth;
          const point = (value: number, index: number) => ({
            x: left + index * step,
            y: 12 + plotHeight - (value / max) * plotHeight,
          });
          if (type() === 'area') {
            const gradient = ctx.createLinearGradient(0, 12, 0, bottom);
            gradient.addColorStop(0, alpha(tokens.info, 0.35));
            gradient.addColorStop(1, alpha(tokens.info, 0));
            ctx.fillStyle = gradient;
            ctx.beginPath();
            ctx.moveTo(left, bottom);
            values.forEach((value, index) => {
              const p = point(value, index);
              ctx.lineTo(p.x, p.y);
            });
            ctx.lineTo(left + plotWidth, bottom);
            ctx.closePath();
            ctx.fill();
          }
          ctx.strokeStyle = props.color ?? colors()[0];
          ctx.lineWidth = 2;
          ctx.lineJoin = 'round';
          ctx.beginPath();
          values.forEach((value, index) => {
            const p = point(value, index);
            if (index === 0) ctx.moveTo(p.x, p.y);
            else ctx.lineTo(p.x, p.y);
          });
          if (type() !== 'area') ctx.stroke();
          else ctx.stroke();
          ctx.fillStyle = props.color ?? colors()[0];
          values.forEach((value, index) => {
            const p = point(value, index);
            ctx.beginPath();
            ctx.arc(p.x, p.y, 3, 0, Math.PI * 2);
            ctx.fill();
          });
        }
        if (props.labels && props.labels.length > 0) {
          ctx.fillStyle = tokens.mutedForeground;
          ctx.font = '11px system-ui';
          ctx.textAlign = 'center';
          const slot = type() === 'bar' ? plotWidth / props.labels.length : plotWidth / Math.max(1, props.labels.length - 1);
          props.labels.forEach((label, index) => {
            ctx.fillText(label, left + index * slot + (type() === 'bar' ? slot / 2 : 0), height - 6);
          });
        }
      }}
    />
  );
}

export function ChartCard(props: { title: string; description?: string; children?: JSX.Element }) {
  return <CanvasContainer>{props.children}</CanvasContainer>;
}

function CanvasContainer(props: { children?: JSX.Element }) {
  return <>{props.children}</>;
}
