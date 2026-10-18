// A command palette: a filter field over grouped commands, presented modally.
import type { JSX } from 'nsh';
import { signal, Text, View, Scroll } from 'nsh';
import { Modal } from 'nsh/ui/mobile';
import { Icon } from './icon';
import { type IconName } from './icons';
import { Input } from './input';
import { keymap } from 'nsh/keymap';
import { RADIUS_MD, RADIUS_LG, tokens } from './theme';

export interface CommandItem {
  label: string;
  group?: string;
  icon?: IconName;
  shortcut?: string;
  onSelect?: () => void;
}

export interface CommandProps {
  open?: boolean;
  items: CommandItem[];
  placeholder?: string;
  onOpenChange?: (open: boolean) => void;
  onSelect?: (index: number) => void;
}

export function Command(props: CommandProps) {
  const own = signal(false);
  const query = signal('');
  const open = () => props.open ?? own();
  const shown = () => {
    const needle = query().toLowerCase();
    return needle ? props.items.filter((item) => item.label.toLowerCase().includes(needle)) : props.items;
  };
  keymap.bind('mod+k', () => {
    if (props.open === undefined) own.set(true);
    else props.onOpenChange?.(true);
  });
  const groups = () => [...new Set(shown().map((item) => item.group ?? ''))];
  return (
    <Modal
      visible={open()}
      transparent
      animationType="fade"
      onRequestClose={() => {
        if (props.open === undefined) own.set(false);
        props.onOpenChange?.(false);
      }}
    >
      <View style={{ flex: 1, backgroundColor: tokens.overlay, alignItems: 'center', justifyContent: 'flex-start', paddingTop: 80, paddingHorizontal: 24 }}>
        <View
          style={{
            width: 520,
            maxWidth: '100%',
            maxHeight: 420,
            borderRadius: RADIUS_LG,
            borderWidth: 1,
            borderColor: tokens.border,
            backgroundColor: tokens.popover,
            overflow: 'hidden',
          }}
        >
          <View style={{ padding: 8, borderBottomWidth: 1, borderBottomColor: tokens.border }}>
            <Input value={query()} placeholder={props.placeholder ?? 'Type a command…'} onChangeText={(next) => query.set(next)} cleanable autoFocus />
          </View>
          <Scroll style={{ maxHeight: 340 }}>
            {shown().length === 0 ? (
              <Text style={{ padding: 16, fontSize: 13, color: tokens.mutedForeground }}>No commands found</Text>
            ) : (
              groups().map((group) => (
                <View style={{ paddingVertical: 4 }}>
                  {group ? (
                    <Text style={{ paddingHorizontal: 12, paddingVertical: 4, fontSize: 11, fontWeight: '600', color: tokens.mutedForeground }}>
                      {group.toUpperCase()}
                    </Text>
                  ) : null}
                  {shown()
                    .filter((item) => (item.group ?? '') === group)
                    .map((item) => (
                      <View
                       
                        accessibilityRole="menuitem"
                        accessibilityLabel={item.label}
                        onPress={() => {
                          item.onSelect?.();
                          props.onSelect?.(props.items.indexOf(item));
                          if (props.open === undefined) own.set(false);
                          props.onOpenChange?.(false);
                        }}
                        activeOpacity={0.8}
                        style={{ flexDirection: 'row', alignItems: 'center', gap: 8, paddingHorizontal: 12, paddingVertical: 8 }}
                      >
                        {item.icon ? <Icon name={item.icon} size={14} color={tokens.mutedForeground} /> : null}
                        <Text style={{ flex: 1, fontSize: 13, color: tokens.foreground }}>{item.label}</Text>
                        {item.shortcut ? (
                          <Text style={{ fontSize: 11, color: tokens.mutedForeground, fontFamily: 'monospace' }}>{item.shortcut}</Text>
                        ) : null}
                      </View>
                    ))}
                </View>
              ))
            )}
          </Scroll>
        </View>
      </View>
    </Modal>
  );
}

export function CommandTrigger(props: { onPress: () => void; children?: JSX.Element }) {
  return <View onPress={props.onPress}>{props.children}</View>;
}
