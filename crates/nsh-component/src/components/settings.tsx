// A settings screen: a section rail beside grouped preference cards. Controls
// are ordinary catalog controls supplied by the application.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { RADIUS_MD, RADIUS_LG, tokens } from './theme';

export interface SettingsItem {
  id: string;
  label: string;
  description?: string;
  icon?: IconName;
  /** A compact status chip after the label. */
  badge?: string;
  /** Quiet metadata before the control. */
  meta?: string;
  control?: JSX.Element;
  onPress?: () => void;
  /** Draw the label in the danger token, for destructive rows. */
  danger?: boolean;
  disabled?: boolean;
}

export interface SettingsSection {
  id: string;
  title?: string;
  description?: string;
  icon?: IconName;
  items: SettingsItem[];
}

export interface SettingsProps {
  sections: SettingsSection[];
  active?: string;
  onChange?: (active: string) => void;
  /** The page heading, above the section rail and cards. */
  title?: string;
  description?: string;
  sidebarWidth?: number;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

function SectionCard(props: { section?: SettingsSection }) {
  const items = props.section?.items ?? [];
  return (
    <View style={{ borderWidth: 1, borderColor: tokens.border, borderRadius: RADIUS_LG, backgroundColor: tokens.popover, overflow: 'hidden' }}>
      {props.section?.title ? (
        <View style={{ paddingHorizontal: 14, paddingVertical: 10, borderBottomWidth: 1, borderBottomColor: tokens.border, backgroundColor: tokens.muted }}>
          <Text style={{ fontSize: 11, fontWeight: '600', letterSpacing: 0.2, color: tokens.mutedForeground }}>{props.section.title}</Text>
        </View>
      ) : null}
      {items.map((item, index) => (
        <View
          accessibilityRole={item.onPress ? 'button' : undefined}
          accessibilityLabel={item.label}
          accessibilityState={{ disabled: Boolean(item.disabled) }}
          onPress={item.disabled ? undefined : item.onPress}
          activeOpacity={0.85}
          style={{
            flexDirection: 'row',
            alignItems: 'center',
            gap: 10,
            paddingHorizontal: 14,
            paddingVertical: 11,
            borderTopWidth: index === 0 ? 0 : 1,
            borderTopColor: tokens.border,
            opacity: item.disabled ? 0.5 : 1,
          }}
        >
          {item.icon ? (
            <View style={{ width: 28, height: 28, borderRadius: 7, alignItems: 'center', justifyContent: 'center', backgroundColor: tokens.muted }}>
              <Icon name={item.icon} size={14} color={tokens.mutedForeground} />
            </View>
          ) : null}
          <View style={{ flex: 1, gap: 1 }}>
            <View style={{ flexDirection: 'row', alignItems: 'center', gap: 6 }}>
              <Text style={{ fontSize: 12, fontWeight: '500', color: item.danger ? tokens.danger : tokens.foreground }}>{item.label}</Text>
              {item.badge ? (
                <View style={{ paddingHorizontal: 5, paddingVertical: 1, borderRadius: 4, backgroundColor: tokens.accent }}>
                  <Text style={{ fontSize: 8, fontWeight: '600', color: tokens.primary }}>{item.badge}</Text>
                </View>
              ) : null}
            </View>
            {item.description ? (
              <Text style={{ fontSize: 10, lineHeight: 14, color: tokens.mutedForeground }}>{item.description}</Text>
            ) : null}
          </View>
          {item.meta ? <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>{item.meta}</Text> : null}
          {item.control ?? (
            item.onPress ? <Icon name="chevron-right" size={14} color={tokens.mutedForeground} /> : null
          )}
        </View>
      ))}
    </View>
  );
}

export function Settings(props: SettingsProps) {
  const own = signal(props.sections[0]?.id ?? '');
  const active = () => props.active ?? own();
  const shown = () => props.sections.find((section) => section.id === active());
  const showRail = () => (props.sidebarWidth ?? 190) > 0;
  return (
    <View testID={props.testID} style={[{ width: '100%', gap: 14 }, props.style]}>
      {props.title || props.description ? (
        <View style={{ gap: 3 }}>
          {props.title ? <Text style={{ fontSize: 19, lineHeight: 24, fontWeight: '600', color: tokens.foreground }}>{props.title}</Text> : null}
          {props.description ? <Text style={{ fontSize: 12, lineHeight: 17, color: tokens.mutedForeground }}>{props.description}</Text> : null}
        </View>
      ) : null}
      <View style={{ flexDirection: 'row', alignItems: 'flex-start', gap: showRail() ? 18 : 0 }}>
        {showRail() ? <View style={{ width: props.sidebarWidth ?? 190, flexShrink: 0, gap: 2 }}>
          {props.sections.map((section) => {
            const selected = section.id === active();
            return (
              <View
                accessibilityRole="button"
                accessibilityLabel={section.title ?? section.id}
                accessibilityState={{ selected }}
                onPress={() => {
                  if (props.active === undefined) own.set(section.id);
                  props.onChange?.(section.id);
                }}
                activeOpacity={0.85}
                style={{
                  flexDirection: 'row',
                  alignItems: 'center',
                  gap: 8,
                  paddingHorizontal: 9,
                  paddingVertical: 8,
                  borderRadius: 7,
                  backgroundColor: selected ? tokens.accent : 'transparent',
                }}
              >
                {section.icon ? <Icon name={section.icon} size={13} color={selected ? tokens.foreground : tokens.mutedForeground} /> : null}
                <Text style={{ flex: 1, fontSize: 12, fontWeight: selected ? '600' : '400', color: selected ? tokens.foreground : tokens.mutedForeground }}>
                  {section.title ?? section.id}
                </Text>
                {selected ? <View style={{ width: 2, height: 14, borderRadius: 1, backgroundColor: tokens.primary }} /> : null}
              </View>
            );
          })}
        </View> : null}
        <View style={{ flex: 1, minWidth: 0, gap: 10 }}>
          <View style={{ gap: 2 }}>
            <Text style={{ fontSize: 15, fontWeight: '600', color: tokens.foreground }}>{shown()?.title ?? 'Settings'}</Text>
            <Text style={{ fontSize: 11, lineHeight: 15, color: tokens.mutedForeground }}>
              {shown()?.description ?? 'Choose what this section of the application remembers.'}
            </Text>
          </View>
          <SectionCard section={shown()} />
        </View>
      </View>
    </View>
  );
}

export function SettingsRow(props: { label: string; description?: string; children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return (
    <View style={[{ flexDirection: 'row', alignItems: 'center', gap: 10, paddingHorizontal: 14, paddingVertical: 11 }, props.style]}>
      <View style={{ flex: 1, gap: 1 }}>
        <Text style={{ fontSize: 12, fontWeight: '500', color: tokens.foreground }}>{props.label}</Text>
        {props.description ? <Text style={{ fontSize: 10, lineHeight: 14, color: tokens.mutedForeground }}>{props.description}</Text> : null}
      </View>
      {props.children}
    </View>
  );
}

export function SettingsPage(props: { title: string; description?: string; children?: JSX.Element }) {
  return (
    <View style={{ gap: 12 }}>
      <View style={{ gap: 3 }}>
        <Text style={{ fontSize: 19, lineHeight: 24, fontWeight: '600', color: tokens.foreground }}>{props.title}</Text>
        {props.description ? <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>{props.description}</Text> : null}
      </View>
      {props.children}
    </View>
  );
}
