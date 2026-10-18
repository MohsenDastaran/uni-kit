// A custom window title bar. On desktops it hides the system bar and lays the
// content around the platform's window controls; on web and mobile it draws the
// same bar as a preview and the window controls are absent.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { isAvailable, Menu, titleBar, Window } from 'nsh/desktop';
import { FONT_SIZE_SM, SPACING_SM, tokens } from './theme';

export interface TitleBarProps {
  title?: string;
  leading?: JSX.Element;
  trailing?: JSX.Element;
  menu?: boolean;
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
          paddingLeft: isAvailable ? insets().left + 8 : 12,
          paddingRight: isAvailable ? insets().right + 8 : 12,
          borderBottomWidth: props.styled === false ? 0 : 1,
          borderBottomColor: tokens.border,
          backgroundColor: tokens.background,
        },
        props.style,
      ]}
    >
      {props.showControls ? (
        <View style={{ flexDirection: 'row', gap: 6 }}>
          <View style={{ width: 12, height: 12, borderRadius: 6, backgroundColor: tokens.danger }} />
          <View style={{ width: 12, height: 12, borderRadius: 6, backgroundColor: tokens.warning }} />
          <View style={{ width: 12, height: 12, borderRadius: 6, backgroundColor: tokens.success }} />
        </View>
      ) : null}
      {props.leading}
      {props.menu ? (
        <View style={{ flexDirection: 'row', gap: 10 }}>
          <Text style={{ fontSize: FONT_SIZE_SM, color: tokens.mutedForeground }}>File</Text>
          <Text style={{ fontSize: FONT_SIZE_SM, color: tokens.mutedForeground }}>Edit</Text>
          <Text style={{ fontSize: FONT_SIZE_SM, color: tokens.mutedForeground }}>View</Text>
        </View>
      ) : null}
      <Text style={{ flex: 1, fontSize: FONT_SIZE_SM, fontWeight: '500', color: tokens.foreground }}>
        {props.title ?? ''}
      </Text>
      {props.children}
      {props.trailing}
    </View>
  );
}

export function TitleBarMenu(props: { items?: { label: string; onPress?: () => void }[] }) {
  return (
    <View style={{ flexDirection: 'row', gap: 10 }}>
      {(props.items ?? []).map((item) => (
        <Text
          onPress={item.onPress}
          style={{ fontSize: FONT_SIZE_SM, color: tokens.mutedForeground }}
        >
          {item.label}
        </Text>
      ))}
    </View>
  );
}

/** Show a native context menu on desktop; a no-op elsewhere. */
export function showMenu(items: { label: string; onPress?: () => void }[]) {
  if (!isAvailable) return;
  Menu.popup(items.map((item) => ({ label: item.label }))).catch(() => {});
}
