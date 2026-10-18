// An input paired with a filtered list, presented in a transparent modal.
import type { StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View, Scroll } from 'nsh';
import { Modal } from 'nsh/ui/mobile';
import { Icon } from './icon';
import { Input } from './input';
import { RADIUS_LG, tokens } from './theme';

export interface ComboboxOption {
  value: string;
  label: string;
  description?: string;
  disabled?: boolean;
}

export interface ComboboxProps {
  options: ComboboxOption[];
  value?: string;
  defaultValue?: string;
  placeholder?: string;
  searchPlaceholder?: string;
  disabled?: boolean;
  emptyText?: string;
  onChange?: (value: string) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Combobox(props: ComboboxProps) {
  const open = signal(false);
  const query = signal('');
  const own = signal(props.defaultValue ?? '');
  const value = () => props.value ?? own();
  const chosen = () => props.options.find((option) => option.value === value());
  const shown = () => {
    const needle = query().toLowerCase();
    return needle
      ? props.options.filter((option) => option.label.toLowerCase().includes(needle))
      : props.options;
  };
  const choose = (option: ComboboxOption) => {
    if (option.disabled) return;
    if (props.value === undefined) own.set(option.value);
    props.onChange?.(option.value);
    open.set(false);
    query.set('');
  };
  return (
    <View testID={props.testID} style={[{ gap: 4 }, props.style]}>
      <Input
        value={open() ? query() : chosen()?.label ?? ''}
        placeholder={props.placeholder ?? 'Choose an option'}
        disabled={props.disabled}
        suffixIcon="chevrons-up-down"
        cleanable={open()}
        onChangeText={(text) => {
          query.set(text);
          open.set(true);
        }}
        onFocus={() => open.set(true)}
      />
      <Modal visible={open()} transparent animationType="fade" onRequestClose={() => open.set(false)}>
        <View style={{ flex: 1, backgroundColor: tokens.overlay, alignItems: 'center', justifyContent: 'center', padding: 24 }}>
          <View class="shadow-2xl" style={{ width: 380, maxWidth: '100%', maxHeight: 440, borderRadius: RADIUS_LG, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover, overflow: 'hidden' }}>
            <View style={{ padding: 8, borderBottomWidth: 1, borderBottomColor: tokens.border }}>
              <Input value={query()} placeholder={props.searchPlaceholder ?? 'Search'} onChangeText={(next) => query.set(next)} cleanable autoFocus />
            </View>
            <Scroll style={{ maxHeight: 360 }}>
              {shown().length === 0 ? (
                <Text style={{ padding: 14, fontSize: 13, color: tokens.mutedForeground }}>{props.emptyText ?? 'No matches'}</Text>
              ) : (
                shown().map((option) => (
                  <View
                    accessibilityRole="button"
                    accessibilityLabel={option.label}
                    accessibilityState={{ selected: option.value === value(), disabled: Boolean(option.disabled) }}
                    onPress={() => choose(option)}
                    disabled={option.disabled}
                    activeOpacity={0.8}
                    style={{
                      paddingHorizontal: 12,
                      paddingVertical: 9,
                      gap: 2,
                      backgroundColor: option.value === value() ? tokens.accent : tokens.popover,
                      opacity: option.disabled ? 0.5 : 1,
                    }}
                  >
                    <View style={{ flexDirection: 'row', alignItems: 'center', gap: 8 }}>
                      <Text style={{ flex: 1, fontSize: 14, color: tokens.foreground }}>{option.label}</Text>
                      {option.value === value() ? <Icon name="check" size={14} color={tokens.primary} /> : null}
                    </View>
                    {option.description ? (
                      <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>{option.description}</Text>
                    ) : null}
                  </View>
                ))
              )}
            </Scroll>
          </View>
        </View>
      </Modal>
    </View>
  );
}
