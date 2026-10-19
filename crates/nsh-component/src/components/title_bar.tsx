// A custom window title bar. On desktops it hides the system bar and lays the
// content around the platform's window controls; on web and mobile it draws the
// same bar as a preview and the window controls are absent.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { isAvailable, Menu, titleBar, Window } from 'nsh/desktop';
import { FONT_SIZE_SM, SPACING_SM, tokens } from './theme';

export interface TitleBarMenuItem {
  label: string;
  onPress?: () => void;
  active?: boolean;
}

export interface TitleBarProps {
  title?: string;
  leading?: JSX.Element;
  trailing?: JSX.Element;
  /** Draw the application menu bar. */
  menu?: boolean;
  /** Menu entries for the bar; defaults to File / Edit / View. */
  menuItems?: TitleBarMenuItem[];
  height?: number;
  styled?: boolean;
  showControls?: boolean;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function TitleBar(props: TitleBarProps) {
  const height = () => props.height ?? 40;
  if (isAvailable) {
    Window.setTitleBar('hidden', height());
  }
  const insets = () => {
    if (!isAvailable) return { height: 0, left: 0, right: 0 };
    return titleBar();
  };
  const menuItems = () => {
    const base = props.menuItems ?? [
      { label: 'File' },
      { label: 'Edit' },
      { label: 'View' },
    ];
    if (!props.title) return base;
    // With a menu bar the title is the active application entry, as in macOS.
    return [...base, { label: props.title, active: true }];
  };
  return (
    <View
      testID={props.testID}
      desktop={{ windowDrag: true }}
      style={[
        {
          height: height(),
          flexDirection: 'row',
          alignItems: 'center',
          gap: SPACING_SM,
          paddingLeft: isAvailable ? insets().left + 10 : 12,
          paddingRight: isAvailable ? insets().right + 10 : 12,
          borderBottomWidth: props.styled === false ? 0 : 1,
          borderBottomColor: tokens.border,
          backgroundColor: tokens.background,
        },
        props.style,
      ]}
    >
      {props.showControls ? (
        <View style={{ flexDirection: 'row', alignItems: 'center', gap: 7, height: 28 }}>
          <View style={{ width: 11, height: 11, borderRadius: 6, backgroundColor: tokens.danger }} />
          <View style={{ width: 11, height: 11, borderRadius: 6, backgroundColor: tokens.warning }} />
          <View style={{ width: 11, height: 11, borderRadius: 6, backgroundColor: tokens.success }} />
        </View>
      ) : null}
      {props.leading}
      {props.menu ? <TitleBarMenu items={menuItems()} /> : null}
      {!props.menu ? (
        <Text style={{ flex: 1, fontSize: FONT_SIZE_SM, fontWeight: '500', color: tokens.foreground }} numberOfLines={1}>
          {props.title ?? ''}
        </Text>
      ) : (
        <View style={{ flex: 1 }} />
      )}
      {props.children}
      {props.trailing ? <View style={{ alignItems: 'center' }}>{props.trailing}</View> : null}
    </View>
  );
}

export function TitleBarMenu(props: { items?: TitleBarMenuItem[] }) {
  return (
    <View style={{ flexDirection: 'row', alignItems: 'center', gap: 2, height: 28 }}>
      {(props.items ?? []).map((item) => (
        <View
          accessibilityRole={item.onPress ? 'button' : undefined}
          accessibilityLabel={item.label}
          onPress={item.onPress}
          activeOpacity={0.8}
          class="transition-colors duration-150 hover:bg-black/[0.05] dark:hover:bg-white/[0.06]"
          style={{
            height: 26,
            paddingHorizontal: 8,
            alignItems: 'center',
            justifyContent: 'center',
            borderRadius: 5,
            backgroundColor: item.active ? tokens.accent : 'transparent',
          }}
        >
          <Text
            style={{
              fontSize: 11,
              lineHeight: 16,
              fontWeight: item.active ? '600' : '400',
              color: item.active ? tokens.foreground : tokens.mutedForeground,
            }}
          >
            {item.label}
          </Text>
        </View>
      ))}
    </View>
  );
}

/** Show a native context menu on desktop; a no-op elsewhere. */
export function showMenu(items: { label: string; onPress?: () => void }[]) {
  if (!isAvailable) return;
  Menu.popup(items.map((item) => ({ label: item.label }))).catch(() => {});
}
