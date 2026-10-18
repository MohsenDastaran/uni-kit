// A popup menu: items, icons, shortcuts, separators and disabled entries.
import type { StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { RADIUS_MD, SPACING_XS, tokens } from './theme';

export interface MenuItem {
  label?: string;
  icon?: IconName;
  shortcut?: string;
  separator?: boolean;
  disabled?: boolean;
  onSelect?: () => void;
}

export interface MenuProps {
  items: MenuItem[];
  onSelect?: (index: number) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Menu(props: MenuProps) {
  return (
    <View
      testID={props.testID}
      accessibilityRole="menu"
      class="shadow-lg"
      style={[
        {
          minWidth: 200,
          maxWidth: '100%',
          alignSelf: 'flex-start',
          paddingVertical: 4,
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
              gap: 8,
              paddingHorizontal: 8,
              paddingVertical: 7,
              borderRadius: SPACING_XS,
              opacity: item.disabled ? 0.5 : 1,
            }}
          >
            {item.icon ? <Icon name={item.icon} size={14} color={tokens.mutedForeground} /> : <View style={{ width: 14 }} />}
            <Text style={{ flex: 1, fontSize: 13, color: tokens.foreground }}>{item.label ?? ''}</Text>
            {item.shortcut ? (
              <Text style={{ fontSize: 11, color: tokens.mutedForeground, fontFamily: 'monospace' }}>{item.shortcut}</Text>
            ) : null}
          </View>
        ),
      )}
    </View>
  );
}
