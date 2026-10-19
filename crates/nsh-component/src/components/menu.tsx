// Popup menus: the panel itself, a button trigger, a menu bar and a press/long-
// press context area. The panel is an absolute child of its trigger, never a
// modal.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Button, type ButtonVariant } from './button';
import { Icon } from './icon';
import { type IconName } from './icons';
import { alpha, controlHeight, RADIUS_MD, RADIUS_SM, tokens } from './theme';

export type MenuTone = 'default' | 'danger';

export interface MenuItem {
  label?: string;
  icon?: IconName;
  shortcut?: string;
  separator?: boolean;
  disabled?: boolean;
  /** A group label; not pressable. */
  heading?: string;
  /** A check mark before the label. */
  checked?: boolean;
  tone?: MenuTone;
  description?: string;
  submenu?: boolean;
  onSelect?: () => void;
}

export interface MenuProps {
  items: MenuItem[];
  onSelect?: (index: number) => void;
  width?: number;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Menu(props: MenuProps) {
  return (
    <View
      testID={props.testID}
      accessibilityRole="menu"
      class="shadow-2xl"
      style={[
        {
          width: props.width ?? 220,
          maxWidth: '100%',
          alignSelf: 'flex-start',
          paddingVertical: 4,
          paddingHorizontal: 4,
          borderRadius: RADIUS_MD,
          borderWidth: 1,
          borderColor: tokens.border,
          backgroundColor: tokens.popover,
        },
        props.style,
      ]}
    >
      {props.items.map((item, index) =>
        item.separator ? (
          <View style={{ height: 1, marginVertical: 4, backgroundColor: tokens.border }} />
        ) : item.heading ? (
          <Text style={{ paddingHorizontal: 7, paddingTop: 5, paddingBottom: 3, fontSize: 9, lineHeight: 13, fontWeight: '600', letterSpacing: 0.4, color: tokens.mutedForeground }}>
            {item.heading}
          </Text>
        ) : (
          <View
            accessibilityRole="menuitem"
            accessibilityLabel={item.label}
            accessibilityState={{ disabled: Boolean(item.disabled) }}
            onPress={() => {
              if (item.disabled) return;
              item.onSelect?.();
              props.onSelect?.(index);
            }}
            disabled={item.disabled}
            activeOpacity={0.8}
            class="bg-transparent transition-colors duration-150 hover:bg-black/[0.05] active:bg-black/[0.10] dark:hover:bg-white/[0.10] dark:active:bg-white/[0.15]"
            style={{
              flexDirection: 'row',
              alignItems: 'center',
              gap: 7,
              paddingHorizontal: 7,
              paddingVertical: 6,
              borderRadius: RADIUS_SM,
              opacity: item.disabled ? 0.42 : 1,
            }}
          >
            <View style={{ width: 14, alignItems: 'center' }}>
              {item.checked ? <Icon name="check" size={12} color={tokens.primary} /> : item.icon ? <Icon name={item.icon} size={13} color={item.tone === 'danger' ? tokens.danger : tokens.mutedForeground} /> : null}
            </View>
            <View style={{ flex: 1, minWidth: 0, gap: 1 }}>
              <Text style={{ fontSize: 12, lineHeight: 17, color: item.tone === 'danger' ? tokens.danger : tokens.foreground }}>{item.label ?? ''}</Text>
              {item.description ? <Text style={{ fontSize: 9, lineHeight: 13, color: tokens.mutedForeground }}>{item.description}</Text> : null}
            </View>
            {item.shortcut ? (
              <Text style={{ fontSize: 10, color: tokens.mutedForeground, fontFamily: 'monospace' }}>{item.shortcut}</Text>
            ) : null}
            {item.submenu ? <Icon name="chevron-right" size={11} color={tokens.mutedForeground} /> : null}
          </View>
        ),
      )}
    </View>
  );
}

/** A single button that opens a menu below it. */
export function MenuButton(props: {
  label: string;
  items: MenuItem[];
  variant?: ButtonVariant;
  size?: 'small' | 'medium' | 'large';
  align?: 'start' | 'end';
  width?: number;
  disabled?: boolean;
  onSelect?: (index: number) => void;
  onPress?: () => void;
  style?: StyleProp<ViewStyle>;
}) {
  const open = signal(false);
  const close = () => open.set(false);
  return (
    <View style={[{ position: 'relative', alignSelf: 'flex-start', zIndex: open() ? 1000 : 0 }, props.style]}>
      <Button
        label={props.label}
        variant={props.variant ?? 'secondary'}
        size={props.size ?? 'small'}
        disabled={props.disabled}
        suffixIcon="chevron-down"
        onPress={() => {
          open.set(!open());
          props.onPress?.();
        }}
      />
      {open() ? (
        <View style={{ position: 'absolute', top: controlHeight(props.size ?? 'small') + 4, ...(props.align === 'end' ? { right: 0 } : { left: 0 }), zIndex: 1001 }}>
          <Menu
            items={props.items}
            width={props.width}
            onSelect={(index) => {
              close();
              props.onSelect?.(index);
            }}
          />
        </View>
      ) : null}
    </View>
  );
}

/** A row of menu buttons; only one menu is open at a time. */
export function MenuBar(props: {
  menus: { label: string; items: MenuItem[] }[];
  width?: number;
  style?: StyleProp<ViewStyle>;
}) {
  const openIndex = signal(-1);
  return (
    <View
      style={[
        {
          flexDirection: 'row',
          alignItems: 'center',
          gap: 2,
          padding: 3,
          borderRadius: 7,
          borderWidth: 1,
          borderColor: tokens.border,
          backgroundColor: tokens.muted,
          alignSelf: 'flex-start',
          position: 'relative',
          zIndex: openIndex() >= 0 ? 1000 : 0,
        },
        props.style,
      ]}
    >
      {props.menus.map((menu, index) => (
        <View
          accessibilityRole="button"
          accessibilityLabel={menu.label}
          accessibilityState={{ expanded: openIndex() === index }}
          onPress={() => openIndex.set(openIndex() === index ? -1 : index)}
          activeOpacity={0.85}
          class="transition-colors duration-150 hover:bg-black/[0.05] dark:hover:bg-white/[0.08]"
          style={{
            paddingHorizontal: 9,
            paddingVertical: 5,
            borderRadius: 5,
            backgroundColor: openIndex() === index ? tokens.background : 'transparent',
          }}
        >
          <Text style={{ fontSize: 11, fontWeight: '500', color: tokens.foreground }}>{menu.label}</Text>
        </View>
      ))}
      {openIndex() >= 0 ? (
        <View style={{ position: 'absolute', top: 34, left: 4, zIndex: 1001 }}>
          <Menu
            items={props.menus[openIndex()]!.items}
            width={props.width}
            onSelect={() => openIndex.set(-1)}
          />
        </View>
      ) : null}
    </View>
  );
}

/** A card whose press/long press opens a context menu in place. */
export function ContextMenu(props: {
  items: MenuItem[];
  children?: JSX.Element;
  width?: number;
  style?: StyleProp<ViewStyle>;
}) {
  const open = signal(false);
  return (
    <View
      onLongPress={() => open.set(!open())}
      onPress={() => open.set(!open())}
      activeOpacity={0.95}
      style={[
        {
          position: 'relative',
          alignSelf: 'flex-start',
          padding: 14,
          borderRadius: 10,
          borderWidth: 1,
          borderColor: tokens.border,
          backgroundColor: tokens.muted,
          zIndex: open() ? 1000 : 0,
          cursor: 'context-menu',
        },
        props.style,
      ]}
    >
      {props.children}
      {open() ? (
        <View style={{ position: 'absolute', top: 38, left: 10, zIndex: 1001 }}>
          <Menu items={props.items} width={props.width} onSelect={() => open.set(false)} />
        </View>
      ) : null}
    </View>
  );
}

export function MenuLabel(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return (
    <Text style={[{ fontSize: 11, fontWeight: '600', color: tokens.foreground }, props.style]}>
      {props.children}
    </Text>
  );
}
