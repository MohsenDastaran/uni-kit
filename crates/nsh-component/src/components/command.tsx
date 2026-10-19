// A command palette: search, grouped rows with optional avatars, descriptions,
// badges and shortcut keys. Rendered inline in the page by default; the modal
// presentation is still available with `inline={false}` and `open`.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View, Scroll } from 'nsh';
import { Modal } from 'nsh/ui/mobile';
import { keymap } from 'nsh/keymap';
import { Icon } from './icon';
import { type IconName } from './icons';
import { Input } from './input';
import { Avatar } from './avatar';
import { alpha, isDark, RADIUS_LG, tokens } from './theme';

export type CommandBadgeTone = 'default' | 'info' | 'success' | 'warning' | 'danger';

export interface CommandItem {
  label: string;
  description?: string;
  group?: string;
  icon?: IconName;
  /** A caller-drawn leading element, such as an avatar. */
  leading?: JSX.Element;
  /** Draws the item's avatar from a name; simpler than passing a node. */
  avatarName?: string;
  shortcut?: string;
  badge?: string;
  badgeTone?: CommandBadgeTone;
  /** Extra searchable text, e.g. an email address. */
  keywords?: string;
  disabled?: boolean;
  onSelect?: () => void;
}

export interface CommandProps {
  open?: boolean;
  items: CommandItem[];
  placeholder?: string;
  /** Draw the panel inline instead of presenting it as a modal. */
  inline?: boolean;
  /** Start the inline panel visible (the default for inline). */
  defaultOpen?: boolean;
  /** Pin the open state; useful for a gallery sample. */
  alwaysOpen?: boolean;
  query?: string;
  onQueryChange?: (query: string) => void;
  selectedIndex?: number;
  defaultSelectedIndex?: number;
  onSelectedIndexChange?: (index: number) => void;
  showSearch?: boolean;
  emptyText?: string;
  footer?: JSX.Element;
  /** A global mod+k binding for a modal palette. */
  hotkey?: boolean;
  width?: number;
  maxHeight?: number;
  onOpenChange?: (open: boolean) => void;
  onSelect?: (index: number) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const toneOf = (tone?: CommandBadgeTone): string =>
  tone === 'info'
    ? tokens.info
    : tone === 'success'
      ? tokens.success
      : tone === 'warning'
        ? tokens.warning
        : tone === 'danger'
          ? tokens.danger
          : tokens.mutedForeground;

export function Command(props: CommandProps) {
  const ownOpen = signal(Boolean(props.defaultOpen ?? props.inline));
  const query = signal('');
  const selected = signal(props.defaultSelectedIndex ?? 0);
  const open = () => (props.alwaysOpen ? true : props.open ?? ownOpen());
  const activeQuery = () => props.query ?? query();
  const setQuery = (next: string) => {
    if (props.query === undefined) query.set(next);
    props.onQueryChange?.(next);
    if (props.selectedIndex === undefined) selected.set(0);
    props.onSelectedIndexChange?.(0);
  };
  const activeIndex = () => props.selectedIndex ?? selected();
  const setIndex = (next: number) => {
    if (props.selectedIndex === undefined) selected.set(next);
    props.onSelectedIndexChange?.(next);
  };

  if (props.hotkey && props.inline !== true) {
    keymap.bind('mod+k', () => {
      if (props.open === undefined) ownOpen.set(true);
      props.onOpenChange?.(true);
    });
  }

  const shown = () => {
    const needle = activeQuery().trim().toLowerCase();
    if (!needle) return props.items;
    return props.items.filter((item) => {
      const haystack = `${item.label} ${item.description ?? ''} ${item.keywords ?? ''}`.toLowerCase();
      return haystack.includes(needle);
    });
  };
  const groups = () => {
    const list = shown();
    return Array.from(new Set(list.map((item) => item.group ?? '')))
      .map((name) => ({ name, items: list.filter((item) => (item.group ?? '') === name) }))
      .filter((group) => group.items.length > 0);
  };

  const choose = (item: CommandItem) => {
    if (item.disabled) return;
    item.onSelect?.();
    props.onSelect?.(props.items.indexOf(item));
    if (props.inline !== true) {
      if (props.open === undefined) ownOpen.set(false);
      props.onOpenChange?.(false);
    }
  };

  const search = () => (
    <View
      style={{
        paddingHorizontal: 8,
        paddingVertical: 7,
        borderBottomWidth: 1,
        borderBottomColor: tokens.border,
      }}
    >
      <Input
        value={activeQuery()}
        placeholder={props.placeholder ?? 'Search commands…'}
        prefixIcon="search"
        onChangeText={setQuery}
        cleanable
        autoFocus={props.inline !== true}
        size="small"
      />
    </View>
  );

  const row = (item: CommandItem, index: number) => {
    const selectedRow = props.items.indexOf(item) === activeIndex();
    const tone = toneOf(item.badgeTone);
    return (
      <View
        accessibilityRole="menuitem"
        accessibilityLabel={item.label}
        accessibilityState={{ selected: selectedRow, disabled: Boolean(item.disabled) }}
        onPress={item.disabled ? undefined : () => choose(item)}
        onHoverIn={() => !item.disabled && setIndex(props.items.indexOf(item))}
        disabled={item.disabled}
        activeOpacity={0.9}
        class="transition-colors duration-150 hover:bg-black/[0.04] dark:hover:bg-white/[0.06]"
        style={{
          flexDirection: 'row',
          alignItems: 'center',
          gap: 9,
          paddingHorizontal: 9,
          paddingVertical: 7,
          borderRadius: 7,
          backgroundColor: selectedRow ? alpha(tokens.foreground, isDark() ? 0.08 : 0.055) : 'transparent',
          opacity: item.disabled ? 0.45 : 1,
        }}
      >
        {item.leading ??
          (item.avatarName ? (
            <Avatar name={item.avatarName} size="xs" />
          ) : item.icon ? (
            <View style={{ width: 20, alignItems: 'center', justifyContent: 'center' }}>
              <Icon name={item.icon} size={15} color={tokens.mutedForeground} />
            </View>
          ) : null)}
        <View style={{ flexShrink: 1, flexGrow: 1, gap: 1 }}>
          <Text
            style={{
              fontSize: 12,
              lineHeight: 16,
              fontWeight: item.description ? '600' : '500',
              color: tokens.foreground,
            }}
          >
            {item.label}
          </Text>
          {item.description ? (
            <Text style={{ fontSize: 10, lineHeight: 14, color: tokens.mutedForeground }} numberOfLines={1}>
              {item.description}
            </Text>
          ) : null}
        </View>
        {item.badge ? (
          <View
            style={{
              paddingHorizontal: 5,
              paddingVertical: 1,
              borderRadius: 4,
              borderWidth: 1,
              borderColor: alpha(tone, 0.32),
              backgroundColor: alpha(tone, 0.12),
            }}
          >
            <Text style={{ fontSize: 9, lineHeight: 13, fontWeight: '500', color: tone }}>{item.badge}</Text>
          </View>
        ) : null}
        {item.shortcut ? (
          <Text style={{ fontSize: 10, lineHeight: 14, color: tokens.mutedForeground, fontFamily: 'monospace' }}>
            {item.shortcut}
          </Text>
        ) : null}
      </View>
    );
  };

  const panel = () => (
    <View
      testID={props.testID}
      style={[
        {
          width: props.width ?? 380,
          maxWidth: '100%',
          borderRadius: RADIUS_LG,
          borderWidth: 1,
          borderColor: tokens.border,
          backgroundColor: tokens.popover,
          overflow: 'hidden',
        },
        props.style,
      ]}
    >
      {props.showSearch !== false ? search() : null}
      <Scroll style={{ maxHeight: props.maxHeight ?? 320 }}>
        {shown().length === 0 ? (
          <View style={{ paddingHorizontal: 12, paddingVertical: 18, gap: 2 }}>
            <Text style={{ fontSize: 11, fontWeight: '600', color: tokens.foreground }}>
              No results
            </Text>
            <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>
              {props.emptyText ?? 'Try another name or command.'}
            </Text>
          </View>
        ) : (
          groups().map((group) => (
            <View style={{ paddingVertical: 3 }}>
              {group.name ? (
                <Text
                  style={{
                    paddingHorizontal: 9,
                    paddingTop: 5,
                    paddingBottom: 3,
                    fontSize: 10,
                    lineHeight: 14,
                    fontWeight: '500',
                    color: tokens.mutedForeground,
                  }}
                >
                  {group.name}
                </Text>
              ) : null}
              {group.items.map((item) => row(item, props.items.indexOf(item)))}
            </View>
          ))
        )}
      </Scroll>
      {props.footer ? (
        <View style={{ paddingHorizontal: 9, paddingVertical: 7, borderTopWidth: 1, borderTopColor: tokens.border }}>
          {props.footer}
        </View>
      ) : null}
    </View>
  );

  if (props.inline === true) {
    return <View style={{ gap: 6 }}>{panel()}</View>;
  }

  return (
    <Modal
      visible={open()}
      transparent
      animationType="fade"
      onRequestClose={() => {
        if (props.open === undefined) ownOpen.set(false);
        props.onOpenChange?.(false);
      }}
    >
      <View
        style={{
          flex: 1,
          backgroundColor: tokens.overlay,
          alignItems: 'center',
          justifyContent: 'flex-start',
          paddingTop: 80,
          paddingHorizontal: 24,
        }}
      >
        {panel()}
      </View>
    </Modal>
  );
}

export function CommandTrigger(props: { onPress: () => void; children?: JSX.Element }) {
  return <View onPress={props.onPress}>{props.children}</View>;
}
