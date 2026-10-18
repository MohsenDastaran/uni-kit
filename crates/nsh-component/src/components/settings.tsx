// A settings screen: grouped rows, a heading and a page switch.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Group } from 'nsh/ui/mobile';
import { Button } from './button';
import { Icon } from './icon';
import { type IconName } from './icons';
import { tokens } from './theme';

export interface SettingsItem {
  id: string;
  label: string;
  description?: string;
  icon?: IconName;
  control?: JSX.Element;
  onPress?: () => void;
}

export interface SettingsSection {
  id: string;
  title?: string;
  items: SettingsItem[];
}

export interface SettingsProps {
  sections: SettingsSection[];
  active?: string;
  onChange?: (active: string) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Settings(props: SettingsProps) {
  const own = signal(props.sections[0]?.id ?? '');
  const active = () => props.active ?? own();
  const shown = () => props.sections.find((section) => section.id === active());
  return (
    <View testID={props.testID} style={[{ flexDirection: 'row', gap: 16 }, props.style]}>
      <View style={{ width: 180, gap: 2 }}>
        {props.sections.map((section) => (
          <View
            accessibilityRole="button"
            accessibilityLabel={section.title ?? section.id}
            accessibilityState={{ selected: section.id === active() }}
            onPress={() => {
              if (props.active === undefined) own.set(section.id);
              props.onChange?.(section.id);
            }}
            activeOpacity={0.8}
            style={{ paddingHorizontal: 8, paddingVertical: 7, borderRadius: 6, backgroundColor: section.id === active() ? tokens.accent : 'transparent' }}
          >
            <Text style={{ fontSize: 13, color: section.id === active() ? tokens.foreground : tokens.mutedForeground }}>{section.title ?? section.id}</Text>
          </View>
        ))}
      </View>
      <View style={{ flex: 1, gap: 12 }}>
        <Text style={{ fontSize: 18, fontWeight: '600', color: tokens.foreground }}>{shown()?.title ?? 'Settings'}</Text>
        <Group>
          {(shown()?.items ?? []).map((item) => (
            <View onPress={item.onPress} style={{ flexDirection: 'row', alignItems: 'center', gap: 10 }}>
              {item.icon ? <Icon name={item.icon} size={16} color={tokens.mutedForeground} /> : null}
              <View style={{ flex: 1, gap: 1 }}>
                <Text style={{ fontSize: 14, color: tokens.foreground }}>{item.label}</Text>
                {item.description ? <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>{item.description}</Text> : null}
              </View>
              {item.control ?? <Icon name="chevron-right" size={14} color={tokens.mutedForeground} />}
            </View>
          ))}
        </Group>
      </View>
    </View>
  );
}

export function SettingsRow(props: { label: string; description?: string; children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return (
    <View style={[{ flexDirection: 'row', alignItems: 'center', gap: 10 }, props.style]}>
      <View style={{ flex: 1, gap: 1 }}>
        <Text style={{ fontSize: 14, color: tokens.foreground }}>{props.label}</Text>
        {props.description ? <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>{props.description}</Text> : null}
      </View>
      {props.children}
    </View>
  );
}

export function SettingsPage(props: { title: string; children?: JSX.Element }) {
  return (
    <View style={{ gap: 12 }}>
      <Text style={{ fontSize: 18, fontWeight: '600', color: tokens.foreground }}>{props.title}</Text>
      {props.children}
    </View>
  );
}
