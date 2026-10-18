// A plain-text code editor: line numbers, a toolbar and a multiline Input.
//
// NSH's native Input cannot style spans, so this port has no syntax colours.
// It is a working editor with the catalog's frame and events; the limitation is
// recorded in docs/nsh-integration.md.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Input } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { FONT_SIZE_MONO, MONO, RADIUS_MD, tokens } from './theme';

export interface EditorProps {
  value?: string;
  defaultValue?: string;
  language?: string;
  readOnly?: boolean;
  lineNumbers?: boolean;
  toolbar?: JSX.Element;
  onChange?: (value: string) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Editor(props: EditorProps) {
  const own = signal(props.defaultValue ?? '');
  const value = () => props.value ?? own();
  const lines = () => Math.max(1, value().split('\n').length);
  return (
    <View
      testID={props.testID}
      style={[
        {
          borderWidth: 1,
          borderColor: tokens.border,
          borderRadius: RADIUS_MD,
          overflow: 'hidden',
          backgroundColor: tokens.background,
        },
        props.style,
      ]}
    >
      <View style={{ flexDirection: 'row', alignItems: 'center', gap: 6, paddingHorizontal: 8, paddingVertical: 5, borderBottomWidth: 1, borderBottomColor: tokens.border, backgroundColor: tokens.muted }}>
        <Icon name="code" size={13} color={tokens.mutedForeground} />
        <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>{props.language ?? 'text'}</Text>
        <View style={{ flex: 1 }} />
        {props.toolbar}
      </View>
      <View style={{ flexDirection: 'row', minHeight: 180 }}>
        {props.lineNumbers !== false ? (
          <View style={{ width: 34, paddingTop: 8, paddingRight: 6, backgroundColor: tokens.muted, alignItems: 'flex-end' }}>
            {Array.from({ length: lines() }, (_, index) => (
              <Text style={{ fontSize: FONT_SIZE_MONO, lineHeight: 20, color: tokens.mutedForeground }}>{index + 1}</Text>
            ))}
          </View>
        ) : null}
        <Input
          multiline
          value={value()}
          editable={!props.readOnly}
          onChangeText={(text) => {
            if (props.value === undefined) own.set(text);
            props.onChange?.(text);
          }}
          style={{
            flex: 1,
            padding: 8,
            fontSize: FONT_SIZE_MONO,
            lineHeight: 20,
            fontFamily: MONO,
            color: tokens.foreground,
            backgroundColor: 'transparent',
          }}
        />
      </View>
    </View>
  );
}

export function EditorToolbarButton(props: { icon: IconName; label: string; onPress: () => void }) {
  return (
    <View accessibilityRole="button" accessibilityLabel={props.label} onPress={props.onPress} style={{ width: 22, height: 22, alignItems: 'center', justifyContent: 'center' }}>
      <Icon name={props.icon} size={13} color={tokens.mutedForeground} />
    </View>
  );
}
