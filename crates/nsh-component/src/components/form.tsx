// A form container: cards, sections, stacked or inline fields, descriptions,
// validation and actions.
import type { DimensionValue, JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Icon } from './icon';
import { FONT_SIZE_SM, RADIUS_LG, SPACING_LG, SPACING_MD, tokens } from './theme';

export interface FormProps {
  title?: string;
  description?: string;
  children?: JSX.Element;
  layout?: 'vertical' | 'horizontal';
  columns?: number;
  /** Draw the form as a card with a header and optional footer. */
  card?: boolean;
  footer?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
  onSubmit?: () => void;
}

export function Form(props: FormProps) {
  const body = (
    <View
      style={{
        gap: SPACING_MD,
        flexDirection: props.layout === 'horizontal' ? 'row' : 'column',
        flexWrap: 'wrap',
        flex: props.card ? 1 : undefined,
        width: '100%',
      }}
    >
      {props.children}
    </View>
  );
  if (!props.card) {
    return (
      <View testID={props.testID} style={[{ width: '100%' }, props.style]}>
        {body}
      </View>
    );
  }
  return (
    <View
      testID={props.testID}
      style={[
        {
          borderRadius: RADIUS_LG,
          borderWidth: 1,
          borderColor: tokens.border,
          backgroundColor: tokens.popover,
          overflow: 'hidden',
        },
        props.style,
      ]}
    >
      {props.title || props.description ? (
        <View style={{ paddingHorizontal: 18, paddingVertical: 14, gap: 2, borderBottomWidth: 1, borderBottomColor: tokens.border }}>
          {props.title ? (
            <Text style={{ fontSize: 14, fontWeight: '600', color: tokens.foreground }}>{props.title}</Text>
          ) : null}
          {props.description ? (
            <Text style={{ fontSize: 10, lineHeight: 15, color: tokens.mutedForeground }}>{props.description}</Text>
          ) : null}
        </View>
      ) : null}
      <View style={{ padding: 18, gap: SPACING_MD, width: '100%' }}>{body}</View>
      {props.footer ? (
        <View style={{ paddingHorizontal: 18, paddingVertical: 12, borderTopWidth: 1, borderTopColor: tokens.border, flexDirection: 'row', alignItems: 'center', gap: 8 }}>
          {props.footer}
        </View>
      ) : null}
    </View>
  );
}

export function FormSection(props: {
  title?: string;
  description?: string;
  /** Draw a rule above the section. */
  divider?: boolean;
  children?: JSX.Element;
  footer?: JSX.Element;
  style?: StyleProp<ViewStyle>;
}) {
  return (
    <View
      style={[
        {
          gap: 10,
          width: '100%',
          ...(props.divider ? { borderTopWidth: 1, borderTopColor: tokens.border, paddingTop: 14 } : {}),
        },
        props.style,
      ]}
    >
      {props.title || props.description ? (
        <View style={{ gap: 2 }}>
          {props.title ? (
            <Text style={{ fontSize: 12, fontWeight: '600', color: tokens.foreground }}>{props.title}</Text>
          ) : null}
          {props.description ? (
            <Text style={{ fontSize: 10, lineHeight: 15, color: tokens.mutedForeground }}>{props.description}</Text>
          ) : null}
        </View>
      ) : null}
      {props.children}
      {props.footer}
    </View>
  );
}

export function FormRow(props: {
  children?: JSX.Element;
  columns?: number;
  gap?: number;
  style?: StyleProp<ViewStyle>;
}) {
  return (
    <View style={[{ width: '100%', flexDirection: 'row', alignItems: 'flex-start', flexWrap: 'wrap', gap: props.gap ?? 12 }, props.style]}>
      {props.children}
    </View>
  );
}

export function FormField(props: {
  label: string;
  required?: boolean;
  optional?: boolean;
  description?: string;
  error?: string;
  /** Stack the label above the control, or place it in a fixed left column. */
  layout?: 'stacked' | 'inline';
  labelWidth?: DimensionValue;
  /** A small help affordance beside the label. */
  help?: boolean;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
}) {
  const label = (
    <View style={{ flexDirection: 'row', alignItems: 'center', gap: 3 }}>
      <Text style={{ fontSize: props.layout === 'inline' ? 11 : 12, fontWeight: '500', color: tokens.foreground }}>
        {props.label}
        {props.required ? <Text style={{ color: tokens.danger }}> *</Text> : null}
      </Text>
      {props.optional ? <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>(optional)</Text> : null}
      {props.help ? <Icon name="info" size={10} color={tokens.mutedForeground} /> : null}
    </View>
  );
  const feedback = props.error ? (
    <Text style={{ fontSize: 10, lineHeight: 14, color: tokens.danger }}>{props.error}</Text>
  ) : props.description ? (
    <Text style={{ fontSize: 10, lineHeight: 14, color: tokens.mutedForeground }}>{props.description}</Text>
  ) : null;
  if (props.layout === 'inline') {
    return (
      <View style={[{ width: '100%', flexDirection: 'row', alignItems: 'flex-start', gap: 12 }, props.style]}>
        <View style={{ width: props.labelWidth ?? 150, flexShrink: 0, paddingTop: 8 }}>{label}</View>
        <View style={{ flex: 1, minWidth: 0, gap: 5 }}>
          {props.children}
          {feedback}
        </View>
      </View>
    );
  }
  return (
    <View style={[{ width: '100%', gap: 6, minWidth: 0 }, props.style]}>
      {label}
      {props.children}
      {feedback}
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

export function FormHint(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return (
    <Text style={[{ flex: 1, fontSize: 10, color: tokens.mutedForeground }, props.style]}>
      {props.children}
    </Text>
  );
}
