// A form container: sections, fields, descriptions, validation and actions.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { FONT_SIZE_SM, SPACING_LG, SPACING_MD, tokens } from './theme';

export interface FormProps {
  children?: JSX.Element;
  layout?: 'vertical' | 'horizontal';
  columns?: number;
  style?: StyleProp<ViewStyle>;
  testID?: string;
  onSubmit?: () => void;
}

export function Form(props: FormProps) {
  return (
    <View
      testID={props.testID}
      style={[
        { gap: SPACING_MD, flexDirection: props.layout === 'horizontal' ? 'row' : 'column', flexWrap: 'wrap' },
        props.style,
      ]}
    >
      {props.children}
    </View>
  );
}

export function FormSection(props: { title?: string; description?: string; children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return (
    <View style={[{ gap: SPACING_MD, flex: 1, minWidth: 240 }, props.style]}>
      {props.title ? <Text style={{ fontSize: 16, fontWeight: '600', color: tokens.foreground }}>{props.title}</Text> : null}
      {props.description ? <Text style={{ fontSize: FONT_SIZE_SM, color: tokens.mutedForeground }}>{props.description}</Text> : null}
      {props.children}
    </View>
  );
}

export function FormField(props: {
  label: string;
  required?: boolean;
  description?: string;
  error?: string;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
}) {
  return (
    <View style={[{ gap: 6 }, props.style]}>
      <Text style={{ fontSize: 13, fontWeight: '500', color: tokens.foreground }}>
        {props.label}{props.required ? <Text style={{ color: tokens.danger }}> *</Text> : null}
      </Text>
      {props.children}
      {props.error ? (
        <Text style={{ fontSize: 12, color: tokens.danger }}>{props.error}</Text>
      ) : props.description ? (
        <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>{props.description}</Text>
      ) : null}
    </View>
  );
}

export function FormActions(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return (
    <View style={[{ flexDirection: 'row', justifyContent: 'flex-end', gap: 8, marginTop: SPACING_LG }, props.style]}>
      {props.children}
    </View>
  );
}
