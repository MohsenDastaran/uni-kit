// An input paired with a filtered list, drawn inline under the trigger — no
// modal. The panel is a plain View, so it stacks in the page and a demo can
// show it open without stealing focus or covering the gallery.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View, Scroll } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { Input } from './input';
import { alpha, RADIUS_LG, tokens } from './theme';

export interface ComboboxOption {
  value: string;
  label: string;
  description?: string;
  disabled?: boolean;
  icon?: IconName;
  /** A token colour for the row's icon or trailing note. */
  tone?: string;
  /** A caller-drawn leading element, e.g. an avatar. */
  leading?: JSX.Element;
  /** A compact trailing badge text. */
  badge?: string;
  /** A right-aligned note, e.g. a date. */
  note?: string;
  /** Items without a group render first; grouped items keep their order. */
  group?: string;
}

export interface ComboboxProps {
  options: ComboboxOption[];
  /** Single value. Ignored when `multiple` is set. */
  value?: string;
  defaultValue?: string;
  /** Chips selection. */
  multiple?: boolean;
  values?: string[];
  defaultValues?: string[];
  onValuesChange?: (values: string[]) => void;
  /** Cap the selected chips. */
  maxSelected?: number;
  placeholder?: string;
  searchPlaceholder?: string;
  disabled?: boolean;
  emptyText?: string;
  /** A wider trigger and panel, e.g. for member pickers. */
  width?: number;
  /** Opens the list on the first render, for a gallery sample. */
  defaultOpen?: boolean;
  /** Shows the trigger's selected value in the header list; off for pickers. */
  clearable?: boolean;
  /** Gives the panel a footer action, like "Add new". */
  footer?: JSX.Element;
  onChange?: (value: string) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Combobox(props: ComboboxProps) {
  const open = signal(Boolean(props.defaultOpen));
  const query = signal('');
  const ownValue = signal(props.defaultValue ?? '');
  const ownValues = signal<string[]>(props.defaultValues ?? []);
  const multiple = () => Boolean(props.multiple);
  const value = () => props.value ?? ownValue();
  const values = () => props.values ?? ownValues();
  const chosen = () => props.options.find((option) => option.value === value());
  const isSelected = (option: ComboboxOption) =>
    multiple() ? values().includes(option.value) : option.value === value();
  const shown = () => {
    const needle = query().toLowerCase();
    return needle
      ? props.options.filter(
          (option) =>
            option.label.toLowerCase().includes(needle) ||
            (option.description ?? '').toLowerCase().includes(needle),
        )
      : props.options;
  };
  const groups = () => {
    const list = shown();
    const names = Array.from(new Set(list.map((option) => option.group).filter(Boolean) as string[]));
    return [
      { name: '', items: list.filter((option) => !option.group) },
      ...names.map((name) => ({ name, items: list.filter((option) => option.group === name) })),
    ].filter((group) => group.items.length > 0);
  };

  const commitValues = (next: string[]) => {
    ownValues.set(next);
    props.onValuesChange?.(next);
  };

  const choose = (option: ComboboxOption) => {
    if (option.disabled) return;
    if (multiple()) {
      const current = values();
      if (current.includes(option.value)) {
        commitValues(current.filter((item) => item !== option.value));
        return;
      }
      if (props.maxSelected !== undefined && current.length >= props.maxSelected) return;
      commitValues([...current, option.value]);
      return;
    }
    if (props.value === undefined) ownValue.set(option.value);
    props.onChange?.(option.value);
    open.set(false);
    query.set('');
  };

  const removeChip = (option: ComboboxOption) => {
    commitValues(values().filter((item) => item !== option.value));
  };

  const chips = () =>
    multiple() && values().length > 0 ? (
      <View style={{ flexDirection: 'row', flexWrap: 'wrap', gap: 4, flexShrink: 1 }}>
        {values().map((item) => {
          const option = props.options.find((entry) => entry.value === item);
          if (!option) return null;
          return (
            <View
              style={{
                flexDirection: 'row',
                alignItems: 'center',
                gap: 4,
                paddingStart: option.leading ? 2 : 6,
                paddingEnd: 3,
                height: 22,
                borderRadius: 6,
                borderWidth: 1,
                borderColor: tokens.border,
                backgroundColor: tokens.muted,
              }}
            >
              {option.leading ? option.leading : null}
              <Text style={{ fontSize: 11, color: tokens.foreground }}>{option.label}</Text>
              <View
                accessibilityRole="button"
                accessibilityLabel={`Remove ${option.label}`}
                onPress={() => removeChip(option)}
                class="transition-colors duration-150 hover:opacity-70 active:opacity-50"
                style={{ width: 14, height: 14, alignItems: 'center', justifyContent: 'center' }}
              >
                <Icon name="x" size={10} color={tokens.mutedForeground} />
              </View>
            </View>
          );
        })}
      </View>
    ) : null;

  const row = (option: ComboboxOption) => {
    const selected = isSelected(option);
    return (
      <View
        accessibilityRole="button"
        accessibilityLabel={option.label}
        accessibilityState={{ selected, disabled: Boolean(option.disabled) }}
        onPress={option.disabled ? undefined : () => choose(option)}
        disabled={option.disabled}
        activeOpacity={0.85}
        class="transition-colors duration-150 hover:bg-black/[0.04] dark:hover:bg-white/[0.06]"
        style={{
          flexDirection: 'row',
          alignItems: 'center',
          gap: 8,
          paddingHorizontal: 8,
          paddingVertical: option.description ? 5 : 6,
          borderRadius: 6,
          backgroundColor: selected ? tokens.accent : 'transparent',
          opacity: option.disabled ? 0.45 : 1,
        }}
      >
        {option.leading ??
          (option.icon ? (
            <Icon name={option.icon} size={13} color={option.tone ?? tokens.mutedForeground} />
          ) : null)}
        <View style={{ flexShrink: 1, flexGrow: 1, gap: 1 }}>
          <Text
            style={{
              fontSize: 12,
              lineHeight: 16,
              fontWeight: option.description ? '600' : '400',
              color: option.tone && !option.icon ? option.tone : tokens.foreground,
            }}
          >
            {option.label}
          </Text>
          {option.description ? (
            <Text style={{ fontSize: 10, lineHeight: 14, color: tokens.mutedForeground }}>
              {option.description}
            </Text>
          ) : null}
        </View>
        {option.badge ? (
          <View style={{ paddingHorizontal: 5, paddingVertical: 1, borderRadius: 4, backgroundColor: tokens.muted }}>
            <Text style={{ fontSize: 9, lineHeight: 13, color: tokens.mutedForeground }}>{option.badge}</Text>
          </View>
        ) : null}
        {option.note ? (
          <Text style={{ fontSize: 11, lineHeight: 15, color: tokens.mutedForeground }}>{option.note}</Text>
        ) : null}
        {selected ? <Icon name="check" size={13} color={tokens.foreground} /> : null}
      </View>
    );
  };

  const triggerLabel = () =>
    multiple()
      ? null
      : chosen()?.label ?? '';
  const triggerRow = () => (
    <View
      accessibilityRole="button"
      accessibilityLabel={props.placeholder ?? 'Choose an option'}
      accessibilityState={{ expanded: open(), disabled: Boolean(props.disabled) }}
      onPress={() => !props.disabled && open.set(!open())}
      activeOpacity={0.9}
      class="transition-colors duration-150 hover:bg-black/[0.02] dark:hover:bg-white/[0.04]"
      style={{
        minHeight: 32,
        paddingHorizontal: 8,
        paddingVertical: multiple() ? 4 : 0,
        flexDirection: 'row',
        alignItems: 'center',
        gap: 6,
        borderRadius: 8,
        borderWidth: 1,
        borderColor: open() ? tokens.ring : tokens.input,
        backgroundColor: tokens.background,
        opacity: props.disabled ? 0.5 : 1,
      }}
    >
      {multiple() ? (
        chips() ?? (
          <Text style={{ flex: 1, fontSize: 12, color: tokens.mutedForeground }}>
            {props.placeholder ?? 'Choose options'}
          </Text>
        )
      ) : (
        <>
          {chosen()?.leading ?? (chosen()?.icon ? <Icon name={chosen()!.icon!} size={13} color={tokens.mutedForeground} /> : null)}
          <Text
            style={{
              flex: 1,
              fontSize: 12,
              color: value() ? tokens.foreground : tokens.mutedForeground,
            }}
          >
            {triggerLabel() || props.placeholder || 'Choose an option'}
          </Text>
        </>
      )}
      {multiple() ? <View style={{ flex: 1 }} /> : null}
      <Icon name="chevron-down" size={13} color={tokens.mutedForeground} />
    </View>
  );

  const panel = () => (
    <View
      style={{
        gap: 2,
        marginTop: 4,
        padding: 5,
        borderRadius: 10,
        borderWidth: 1,
        borderColor: tokens.border,
        backgroundColor: tokens.popover,
        ...(props.width ? { width: props.width } : {}),
      }}
    >
      <View style={{ paddingVertical: 2 }}>
        <Input
          value={query()}
          placeholder={props.searchPlaceholder ?? 'Search…'}
          onChangeText={(next) => query.set(next)}
          cleanable
          size="small"
        />
      </View>
      <Scroll style={{ maxHeight: 220, gap: 2 }}>
        {shown().length === 0 ? (
          <Text style={{ paddingHorizontal: 8, paddingVertical: 10, fontSize: 11, color: tokens.mutedForeground }}>
            {props.emptyText ?? 'No matches'}
          </Text>
        ) : (
          groups().map((group) => (
            <View style={{ gap: 2 }}>
              {group.name ? (
                <Text
                  style={{
                    paddingHorizontal: 8,
                    paddingTop: 6,
                    paddingBottom: 2,
                    fontSize: 9,
                    lineHeight: 13,
                    letterSpacing: 0.5,
                    color: tokens.mutedForeground,
                  }}
                >
                  {group.name.toUpperCase()}
                </Text>
              ) : null}
              {group.items.map((option) => row(option))}
            </View>
          ))
        )}
      </Scroll>
      {props.footer ? (
        <View style={{ paddingTop: 4, borderTopWidth: 1, borderColor: tokens.border }}>{props.footer}</View>
      ) : null}
    </View>
  );

  return (
    <View
      testID={props.testID}
      style={[
        { gap: 0, ...(props.width ? { width: props.width } : { maxWidth: 320 }) },
        props.style,
      ]}
    >
      {triggerRow()}
      {open() ? panel() : null}
    </View>
  );
}
