// A trigger and an anchored list of options, like the MenuButton panel.
import type { StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View, Scroll } from 'nsh';
import { Icon } from './icon';
import { Input } from './input';
import { type ControlSize, controlFontSize, inputHeight, RADIUS_LG, RADIUS_MD, tokens } from './theme';

export interface SelectOption {
  value: string;
  label: string;
  disabled?: boolean;
}

export interface SelectProps {
  options: SelectOption[];
  value?: string;
  defaultValue?: string;
  placeholder?: string;
  size?: ControlSize;
  disabled?: boolean;
  cleanable?: boolean;
  searchable?: boolean;
  /** Drop the frame when the Select sits inside an InputGroup. */
  appearance?: boolean;
  emptyText?: string;
  onChange?: (value: string) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

let closeActive: (() => void) | undefined;

export function Select(props: SelectProps) {
  const open = signal(false);
  const own = signal(props.defaultValue ?? '');
  const query = signal('');
  const value = () => props.value ?? own();
  const chosen = () => props.options.find((option) => option.value === value());
  const shown = () => {
    const needle = query().toLowerCase();
    return needle ? props.options.filter((option) => option.label.toLowerCase().includes(needle)) : props.options;
  };
  const close = () => {
    open.set(false);
    query.set('');
    if (closeActive === close) closeActive = undefined;
  };
  const toggle = () => {
    if (open()) {
      close();
      return;
    }
    closeActive?.();
    closeActive = close;
    open.set(true);
  };
  const choose = (option: SelectOption) => {
    if (option.disabled) return;
    if (props.value === undefined) own.set(option.value);
    props.onChange?.(option.value);
    close();
  };
  return (
    <View testID={props.testID} style={[{ position: 'relative', gap: 4 }, props.style]}>
      <View
        accessibilityRole="combobox"
        accessibilityLabel={chosen()?.label ?? props.placeholder ?? 'Select'}
        accessibilityState={{ expanded: open(), disabled: Boolean(props.disabled) }}
        onPress={() => !props.disabled && toggle()}
        disabled={props.disabled}
        activeOpacity={0.9}
        style={{
          height: inputHeight(props.size ?? 'medium'),
          paddingHorizontal: props.appearance === false ? 0 : 10,
          flexDirection: 'row',
          alignItems: 'center',
          gap: 8,
          borderRadius: RADIUS_MD,
          borderWidth: props.appearance === false ? 0 : 1,
          borderColor: open() ? tokens.ring : tokens.input,
          backgroundColor: props.appearance === false ? 'transparent' : tokens.background,
          opacity: props.disabled ? 0.5 : 1,
        }}
      >
        <Text style={{ flex: 1, fontSize: controlFontSize(props.size ?? 'medium'), color: chosen() ? tokens.foreground : tokens.mutedForeground }}>
          {chosen()?.label ?? props.placeholder ?? 'Select an option'}
        </Text>
        {props.cleanable && value() ? (
          <View onPress={() => { if (props.value === undefined) own.set(''); props.onChange?.(''); }} style={{ width: 18, height: 18, alignItems: 'center', justifyContent: 'center' }}>
            <Icon name="x" size={12} color={tokens.mutedForeground} />
          </View>
        ) : null}
        <Icon name="chevron-down" size={14} color={tokens.mutedForeground} />
      </View>
      {open() ? (
        <View
          accessibilityRole="menu"
          class="absolute z-50 shadow-2xl"
          style={{
            position: 'absolute',
            top: inputHeight(props.size ?? 'medium') + 4,
            left: 0,
            right: 0,
            maxHeight: 340,
            borderRadius: RADIUS_LG,
            borderWidth: 1,
            borderColor: tokens.border,
            backgroundColor: tokens.popover,
            overflow: 'hidden',
          }}
        >
          {props.searchable ? (
            <View style={{ padding: 8, borderBottomWidth: 1, borderBottomColor: tokens.border }}>
              <Input value={query()} placeholder="Search" onChangeText={(next) => query.set(next)} cleanable style={{ width: '100%' }} />
            </View>
          ) : null}
          <Scroll style={{ maxHeight: 300 }}>
            {shown().length === 0 ? (
              <Text style={{ padding: 14, fontSize: 13, color: tokens.mutedForeground }}>{props.emptyText ?? 'No options'}</Text>
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
                    flexDirection: 'row',
                    alignItems: 'center',
                    gap: 8,
                    paddingHorizontal: 12,
                    paddingVertical: 10,
                    backgroundColor: option.value === value() ? tokens.accent : tokens.popover,
                    opacity: option.disabled ? 0.5 : 1,
                  }}
                >
                  <Text style={{ flex: 1, fontSize: 14, color: tokens.foreground }}>{option.label}</Text>
                  {option.value === value() ? <Icon name="check" size={14} color={tokens.primary} /> : null}
                </View>
              ))
            )}
          </Scroll>
        </View>
      ) : null}
    </View>
  );
}

export function SelectItem(props: { option: SelectOption; selected?: boolean; onPress?: () => void }) {
  return (
    <View onPress={props.onPress} style={{ flexDirection: 'row', padding: 8, gap: 8 }}>
      <Text style={{ flex: 1, fontSize: 14, color: tokens.foreground }}>{props.option.label}</Text>
      {props.selected ? <Icon name="check" size={14} color={tokens.primary} /> : null}
    </View>
  );
}
