// A row of tab labels and the active panel. Underline is the default; segmented,
// pill and enclosed variants cover settings and dashboard toolbars.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Scroll, Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { type ControlSize, controlFontSize, RADIUS_MD, tokens } from './theme';

export type TabsVariant = 'underline' | 'segmented' | 'pills' | 'enclosed';

export interface TabItem {
  value: string;
  label: string;
  content?: JSX.Element;
  disabled?: boolean;
  icon?: IconName;
  badge?: string;
  /** A caller-drawn prefix before the icon. */
  prefix?: JSX.Element;
  /** A caller-drawn suffix after the label. */
  suffix?: JSX.Element;
  closeable?: boolean;
}

export interface TabsProps {
  items: TabItem[];
  value?: string;
  defaultValue?: string;
  onChange?: (value: string) => void;
  onClose?: (value: string) => void;
  variant?: TabsVariant;
  size?: ControlSize;
  /** Spread the tabs across the full width. */
  fullWidth?: boolean;
  /** Scroll the tab row sideways instead of shrinking it. */
  scrollable?: boolean;
  /** A maximum width for each tab. */
  maxTabWidth?: number;
  /** Padding/content style for the panel under the row. */
  contentStyle?: StyleProp<ViewStyle>;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const fontOf = (size: ControlSize): number =>
  size === 'xsmall' ? 10 : size === 'small' ? 11 : size === 'large' ? 14 : controlFontSize(size) - 1;

const padOf = (size: ControlSize): { x: number; y: number } =>
  size === 'xsmall' ? { x: 7, y: 4 } : size === 'small' ? { x: 9, y: 5 } : size === 'large' ? { x: 14, y: 9 } : { x: 11, y: 7 };

export function Tabs(props: TabsProps) {
  const own = signal(props.defaultValue ?? props.items[0]?.value ?? '');
  const value = () => props.value ?? own();
  const active = () => props.items.find((item) => item.value === value());
  const variant = () => props.variant ?? 'underline';
  const size = () => props.size ?? 'medium';
  const pad = () => padOf(size());
  const choose = (item: TabItem) => {
    if (item.disabled) return;
    if (props.value === undefined) own.set(item.value);
    props.onChange?.(item.value);
  };

  const tab = (item: TabItem) => {
    const selected = item.value === value();
    const enclosed = variant() === 'enclosed';
    const segmented = variant() === 'segmented';
    const pill = variant() === 'pills';
    const underline = variant() === 'underline';
    return (
      <View
        accessibilityRole="tab"
        accessibilityLabel={item.label}
        accessibilityState={{ selected, disabled: Boolean(item.disabled) }}
        onPress={() => choose(item)}
        disabled={item.disabled}
        activeOpacity={0.82}
        class="transition-colors duration-150"
        style={{
          flex: props.fullWidth ? 1 : undefined,
          maxWidth: props.maxTabWidth,
          overflow: 'hidden',
          flexDirection: 'row',
          alignItems: 'center',
          justifyContent: 'center',
          gap: 6,
          paddingHorizontal: pad().x,
          paddingVertical: pad().y,
          borderRadius: underline ? 0 : segmented ? 6 : pill ? 999 : 0,
          borderWidth: enclosed ? 1 : 0,
          borderColor: selected ? tokens.border : 'transparent',
          borderBottomWidth: underline ? 2 : enclosed ? 1 : 0,
          borderBottomColor: underline ? (selected ? tokens.primary : 'transparent') : tokens.border,
          backgroundColor: segmented
            ? selected
              ? tokens.background
              : 'transparent'
            : pill
              ? selected
                ? tokens.primary
                : 'transparent'
              : enclosed
                ? selected
                  ? tokens.popover
                  : tokens.muted
                : 'transparent',
          opacity: item.disabled ? 0.45 : 1,
          boxShadow: segmented && selected ? '0 1px 2px rgba(0,0,0,0.12)' : undefined,
        }}
      >
        {item.prefix}
        {item.icon ? <Icon name={item.icon} size={size() === 'large' ? 15 : 13} color={pill && selected ? tokens.primaryForeground : selected ? tokens.foreground : tokens.mutedForeground} /> : null}
        <Text
          numberOfLines={1}
          style={{
            flexShrink: 1,
            fontSize: fontOf(size()),
            fontWeight: selected ? '600' : '400',
            color: pill && selected ? tokens.primaryForeground : selected ? tokens.foreground : tokens.mutedForeground,
          }}
        >
          {item.label}
        </Text>
        {item.badge ? (
          <View
            style={{
              minWidth: 17,
              height: 16,
              paddingHorizontal: 4,
              borderRadius: 8,
              alignItems: 'center',
              justifyContent: 'center',
              backgroundColor: pill && selected ? tokens.primaryForeground : selected ? tokens.primary : tokens.muted,
              opacity: item.disabled ? 0.6 : 1,
            }}
          >
            <Text style={{ fontSize: 8, fontWeight: '600', color: pill && selected ? tokens.primary : selected ? tokens.primaryForeground : tokens.mutedForeground }}>{item.badge}</Text>
          </View>
        ) : null}
        {item.suffix}
        {item.closeable ? (
          <View
            accessibilityRole="button"
            accessibilityLabel={`Close ${item.label}`}
            onTouchStart={(event: any) => event.stopPropagation()}
            onPress={() => props.onClose?.(item.value)}
            style={{ width: 16, height: 16, borderRadius: 4, alignItems: 'center', justifyContent: 'center', marginStart: 1 }}
          >
            <Icon name="x" size={10} color={selected ? tokens.mutedForeground : tokens.mutedForeground} />
          </View>
        ) : null}
      </View>
    );
  };

  const row = (
    <View
      accessibilityRole="tablist"
      style={{
        flexDirection: 'row',
        alignItems: 'stretch',
        gap: variant() === 'segmented' ? 2 : variant() === 'enclosed' ? 0 : 3,
        width: props.scrollable ? undefined : '100%',
        padding: variant() === 'segmented' ? 3 : 0,
        borderRadius: variant() === 'segmented' ? 8 : variant() === 'enclosed' ? 8 : 0,
        backgroundColor: variant() === 'segmented' ? tokens.muted : 'transparent',
        borderWidth: 0,
        borderBottomWidth: variant() === 'underline' ? 1 : 0,
        borderColor: tokens.border,
        borderBottomColor: tokens.border,
        overflow: 'hidden',
      }}
    >
      {props.items.map(tab)}
    </View>
  );

  return (
    <View testID={props.testID} style={[{ width: '100%', gap: 10 }, props.style]}>
      {props.scrollable ? (
        <Scroll horizontal showsIndicator={false}>
          {row}
        </Scroll>
      ) : row}
      {active()?.content ? <View style={[{ paddingVertical: 4 }, props.contentStyle]}>{active()?.content}</View> : null}
    </View>
  );
}
