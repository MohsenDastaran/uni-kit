// A colour picker: an optional saturation area with hue and opacity sliders, a
// format field with a copy action, featured swatches, and the inline trigger
// field that opens it. Everything is drawn inline — no modal or dialog.
//
// The colours in `DEFAULT_FEATURED` are swatch data, the palette the picker
// shows; the rest of the chrome comes from the shared tokens.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { LayoutEvent, signal, Text, View } from 'nsh';
import { Input } from './input';
import { Icon } from './icon';
import { type IconName } from './icons';
import {
  alpha,
  type ControlSize,
  isDark,
  RADIUS_MD,
  tokens,
} from './theme';

export type ColorFormat = 'hex' | 'rgb' | 'hsl';

export interface ColorPickerProps {
  value?: string;
  defaultValue?: string;
  featured?: string[];
  size?: ControlSize;
  disabled?: boolean;
  /** A standalone field that opens the picker inline when pressed. */
  inline?: boolean;
  /** Shows the hue and opacity sliders with the saturation area. */
  showSpectrum?: boolean;
  /** Alpha is `0..1`; only meaningful with `showSpectrum`. */
  alpha?: number;
  onAlphaChange?: (alpha: number) => void;
  /** The field's format. */
  format?: ColorFormat;
  onFormatChange?: (format: ColorFormat) => void;
  /** Renders the colour disc with an icon or swatch instead of a square. */
  triggerIcon?: IconName;
  onChange?: (value: string) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const DEFAULT_FEATURED = ['#ef4444', '#f97316', '#eab308', '#22c55e', '#06b6d4', '#3b82f6', '#8b5cf6', '#ec4899', '#171717', '#737373', '#ffffff'];

const normalize = (value: string) => (value.startsWith('#') ? value : `#${value}`).toUpperCase();

const hexToRgb = (hex: string): [number, number, number] => {
  const clean = hex.replace('#', '');
  const full = clean.length === 3 ? clean.split('').map((c) => c + c).join('') : clean;
  const num = Number.parseInt(full.slice(0, 6), 16);
  if (Number.isNaN(num)) return [0, 0, 0];
  return [(num >> 16) & 255, (num >> 8) & 255, num & 255];
};

const rgbToHsv = ([r, g, b]: [number, number, number]): [number, number, number] => {
  const rn = r / 255;
  const gn = g / 255;
  const bn = b / 255;
  const max = Math.max(rn, gn, bn);
  const min = Math.min(rn, gn, bn);
  const d = max - min;
  let h = 0;
  if (d !== 0) {
    if (max === rn) h = ((gn - bn) / d) % 6;
    else if (max === gn) h = (bn - rn) / d + 2;
    else h = (rn - gn) / d + 4;
  }
  return [(h * 60 + 360) % 360, max === 0 ? 0 : d / max, max];
};

const hsvToHex = (h: number, s: number, v: number): string => {
  const c = v * s;
  const x = c * (1 - Math.abs(((h / 60) % 2) - 1));
  const m = v - c;
  const rgb: [number, number, number] =
    h < 60 ? [c, x, 0] : h < 120 ? [x, c, 0] : h < 180 ? [0, c, x] : h < 240 ? [0, x, c] : h < 300 ? [x, 0, c] : [c, 0, x];
  return `#${rgb
    .map((channel) => Math.round((channel + m) * 255).toString(16).padStart(2, '0'))
    .join('')
    .toUpperCase()}`;
};

const formatValue = (hex: string, format: ColorFormat, opacity: number): string => {
  if (format === 'hex') return normalize(hex);
  const [r, g, b] = hexToRgb(hex);
  if (format === 'rgb') return opacity < 1 ? `rgb(${r} ${g} ${b} / ${opacity.toFixed(2)})` : `rgb(${r} ${g} ${b})`;
  const [h, s, v] = rgbToHsv([r, g, b]);
  const l = v * (1 - s / 2);
  const sl = l === 0 || l === 1 ? 0 : (v - l) / Math.min(l, 1 - l);
  return `hsl(${Math.round(h)} ${Math.round(sl * 100)}% ${Math.round(l * 100)}%)`;
};

const checker = `repeating-conic-gradient(${alpha(tokens.mutedForeground, 0.22)} 0% 25%, transparent 0% 50%) 50% / 8px 8px`;

/** A circular swatch, used by the palette builders and feature rows. */
export function ColorSwatch(props: { color: string; selected?: boolean; onPress?: () => void; size?: number }) {
  const size = () => props.size ?? 24;
  return (
    <View
      accessibilityRole={props.onPress ? 'button' : undefined}
      accessibilityLabel={props.color}
      accessibilityState={props.onPress ? { selected: Boolean(props.selected) } : undefined}
      onPress={props.onPress}
      activeOpacity={0.85}
      class="transition-transform duration-150 hover:scale-105"
      style={{
        width: size(),
        height: size(),
        borderRadius: size() / 2,
        backgroundColor: props.color,
        borderWidth: props.selected ? 2 : 1,
        borderColor: props.selected ? tokens.foreground : alpha(tokens.foreground, 0.14),
        alignItems: 'center',
        justifyContent: 'center',
      }}
    >
      {props.selected ? <Icon name="check" size={size() * 0.55} color={isDark() ? tokens.background : tokens.background} /> : null}
    </View>
  );
}

export function ColorPicker(props: ColorPickerProps) {
  const own = signal(props.defaultValue ?? '#171717');
  const open = signal(false);
  const spectrumWidth = signal(0);
  const spectrumHeight = signal(0);
  const hueWidth = signal(1);
  const alphaWidth = signal(1);
  const value = () => props.value ?? own();
  const set = (next: string) => {
    if (props.disabled) return;
    if (props.value === undefined) own.set(next);
    props.onChange?.(next);
  };
  const opacity = () => props.alpha ?? 1;
  const format = () => props.format ?? 'hex';
  const [h, s, v] = rgbToHsv(hexToRgb(value()));
  const hueColor = (hsvToHex(h, 1, 1));

  const setFromSpectrum = (x: number, y: number) => {
    const w = Math.max(1, spectrumWidth());
    const height = Math.max(1, spectrumHeight());
    const saturation = Math.max(0, Math.min(1, x / w));
    const brightness = 1 - Math.max(0, Math.min(1, y / height));
    set(hsvToHex(h, saturation, brightness));
  };

  const spectrum = () => (
    <View style={{ gap: 10 }}>
      <View
        onLayout={(event: LayoutEvent) => {
          spectrumWidth.set(event.width);
          spectrumHeight.set(event.height);
        }}
        onTouchStart={(event) => setFromSpectrum(event.locationX, event.locationY)}
        onTouchMove={(event) => setFromSpectrum(event.locationX, event.locationY)}
        style={{
          width: '100%',
          height: 150,
          borderRadius: 8,
          overflow: 'hidden',
          backgroundColor: hueColor,
          backgroundImage: isDark()
            ? 'linear-gradient(to top, #000, transparent), linear-gradient(to right, #fff, transparent)'
            : 'linear-gradient(to top, #000, transparent), linear-gradient(to right, #fff, transparent)',
        }}
      >
        <View
          style={{
            position: 'absolute',
            left: `${s * 100}%`,
            top: `${(1 - v) * 100}%`,
            width: 12,
            height: 12,
            marginLeft: -6,
            marginTop: -6,
            borderRadius: 6,
            borderWidth: 2,
            borderColor: tokens.background,
            backgroundColor: value(),
          }}
        />
      </View>
      <View style={{ flexDirection: 'row', alignItems: 'center', gap: 8 }}>
        <Icon name="pipette" size={14} color={tokens.mutedForeground} />
        <View
          onLayout={(event: LayoutEvent) => hueWidth.set(event.width)}
          onTouchStart={(event) => set(hsvToHex(Math.max(0, Math.min(360, (event.locationX / Math.max(1, hueWidth())) * 360)), s, v))}
          onTouchMove={(event) => set(hsvToHex(Math.max(0, Math.min(360, (event.locationX / Math.max(1, hueWidth())) * 360)), s, v))}
          style={{
            flex: 1,
            height: 12,
            borderRadius: 6,
            backgroundImage:
              'linear-gradient(to right, #ff0000, #ffff00, #00ff00, #00ffff, #0000ff, #ff00ff, #ff0000)',
          }}
        >
          <View
            style={{
              position: 'absolute',
              left: `${(h / 360) * 100}%`,
              top: -2,
              width: 16,
              height: 16,
              marginLeft: -8,
              borderRadius: 8,
              borderWidth: 2,
              borderColor: tokens.background,
              backgroundColor: hueColor,
            }}
          />
        </View>
        <View
          accessibilityRole="button"
          accessibilityLabel="Copy colour"
          onPress={() => {}}
          class="transition-opacity duration-150 hover:opacity-70"
          style={{ width: 18, height: 18, alignItems: 'center', justifyContent: 'center' }}
        >
          <Icon name="copy" size={13} color={tokens.mutedForeground} />
        </View>
      </View>
      <View
        onLayout={(event: LayoutEvent) => alphaWidth.set(event.width)}
        onTouchStart={(event) => props.onAlphaChange?.(Math.max(0, Math.min(1, event.locationX / Math.max(1, alphaWidth()))))}
        onTouchMove={(event) => props.onAlphaChange?.(Math.max(0, Math.min(1, event.locationX / Math.max(1, alphaWidth()))))}
        style={{
          height: 12,
          borderRadius: 6,
          backgroundColor: tokens.muted,
          backgroundImage: `${checker}, linear-gradient(to right, transparent, ${value()})`,
        }}
      >
        <View
          style={{
            position: 'absolute',
            left: `${opacity() * 100}%`,
            top: -2,
            width: 16,
            height: 16,
            marginLeft: -8,
            borderRadius: 8,
            borderWidth: 2,
            borderColor: tokens.background,
            backgroundColor: value(),
          }}
        />
      </View>
    </View>
  );

  const field = () => (
    <View style={{ flexDirection: 'row', alignItems: 'center', gap: 8 }}>
      <View
        accessibilityRole="button"
        accessibilityLabel={`Format ${format().toUpperCase()}`}
        onPress={() => props.onFormatChange?.(format() === 'hex' ? 'rgb' : format() === 'rgb' ? 'hsl' : 'hex')}
        style={{
          flexDirection: 'row',
          alignItems: 'center',
          gap: 2,
          paddingHorizontal: 8,
          height: 28,
          borderRadius: 6,
          borderWidth: 1,
          borderColor: tokens.border,
        }}
      >
        <Text style={{ fontSize: 10, fontWeight: '600', color: tokens.foreground }}>{format().toUpperCase()}</Text>
        <Icon name="chevron-down" size={10} color={tokens.mutedForeground} />
      </View>
      <Input
        value={formatValue(value(), format(), opacity())}
        disabled={props.disabled}
        placeholder="#171717"
        onChangeText={set}
        size="small"
        style={{ flex: 1 }}
      />
      {props.onAlphaChange ? (
        <View
          style={{
            minWidth: 44,
            height: 28,
            paddingHorizontal: 6,
            borderRadius: 6,
            borderWidth: 1,
            borderColor: tokens.border,
            alignItems: 'center',
            justifyContent: 'center',
          }}
        >
          <Text style={{ fontSize: 10, color: tokens.foreground }}>{Math.round(opacity() * 100)}%</Text>
        </View>
      ) : null}
    </View>
  );

  const swatches = (size = 22) => (
    <View style={{ flexDirection: 'row', flexWrap: 'wrap', gap: 8 }}>
      {(props.featured ?? DEFAULT_FEATURED).map((color) => (
        <ColorSwatch
          color={color}
          size={props.size === 'large' ? size + 8 : size}
          selected={normalize(value()) === normalize(color)}
          onPress={() => set(color)}
        />
      ))}
    </View>
  );

  if (props.inline) {
    return (
      <View testID={props.testID} style={[{ gap: 6, opacity: props.disabled ? 0.5 : 1 }, props.style]}>
        <View
          accessibilityRole="button"
          accessibilityLabel={value()}
          accessibilityState={{ expanded: open(), disabled: Boolean(props.disabled) }}
          onPress={() => !props.disabled && open.set(!open())}
          activeOpacity={0.9}
          class="transition-colors duration-150 hover:bg-black/[0.02] dark:hover:bg-white/[0.04]"
          style={{
            height: 32,
            paddingHorizontal: 8,
            flexDirection: 'row',
            alignItems: 'center',
            gap: 8,
            borderRadius: 8,
            borderWidth: 1,
            borderColor: open() ? tokens.ring : tokens.input,
            backgroundColor: tokens.background,
          }}
        >
          <View
            style={{
              width: 16,
              height: 16,
              borderRadius: props.triggerIcon ? 8 : 4,
              backgroundColor: value(),
              borderWidth: 1,
              borderColor: alpha(tokens.foreground, 0.14),
              alignItems: 'center',
              justifyContent: 'center',
            }}
          >
            {props.triggerIcon ? <Icon name={props.triggerIcon} size={9} color={tokens.background} /> : null}
          </View>
          <Text style={{ flex: 1, fontSize: 12, color: tokens.foreground }}>{formatValue(value(), format(), opacity())}</Text>
          <Icon name="chevron-down" size={13} color={tokens.mutedForeground} />
        </View>
        {open() ? (
          <View
            style={{
              gap: 10,
              padding: 10,
              borderRadius: 10,
              borderWidth: 1,
              borderColor: tokens.border,
              backgroundColor: tokens.popover,
            }}
          >
            {props.showSpectrum !== false ? spectrum() : null}
            {field()}
            {swatches()}
          </View>
        ) : null}
      </View>
    );
  }

  return (
    <View
      testID={props.testID}
      style={[
        {
          gap: 10,
          padding: 12,
          borderRadius: 12,
          borderWidth: 1,
          borderColor: tokens.border,
          backgroundColor: tokens.popover,
          opacity: props.disabled ? 0.5 : 1,
        },
        props.style,
      ]}
    >
      {props.showSpectrum !== false ? spectrum() : null}
      {field()}
      {swatches()}
    </View>
  );
}

export type { JSX };
