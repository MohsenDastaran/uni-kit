// The chat surface: alignment, maximum width and a content slot.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { alpha, FONT_SIZE_SM, RADIUS_MD, tokens } from './theme';

export interface BubbleProps {
  align?: 'start' | 'end';
  variant?: 'default' | 'primary' | 'muted';
  content?: string;
  actions?: JSX.Element;
  reactions?: string[];
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Bubble(props: BubbleProps) {
  const variant = () => props.variant ?? 'default';
  const background = () =>
    variant() === 'primary' ? tokens.primary : variant() === 'muted' ? tokens.muted : tokens.background;
  const foreground = () => (variant() === 'primary' ? tokens.primaryForeground : tokens.foreground);
  return (
    <View
      testID={props.testID}
      style={[
        {
          alignSelf: props.align === 'end' ? 'flex-end' : 'flex-start',
          maxWidth: '78%',
          gap: 4,
        },
        props.style,
      ]}
    >
      <View
        style={{
          paddingHorizontal: 12,
          paddingVertical: 8,
          borderRadius: RADIUS_MD,
          borderWidth: 1,
          borderColor: variant() === 'primary' ? tokens.primary : tokens.border,
          backgroundColor: background(),
        }}
      >
        {props.content ? <Text style={{ fontSize: FONT_SIZE_SM, lineHeight: 20, color: foreground() }}>{props.content}</Text> : null}
        {props.children}
      </View>
      {props.reactions && props.reactions.length > 0 ? (
        <View style={{ flexDirection: 'row', gap: 4, alignSelf: props.align === 'end' ? 'flex-end' : 'flex-start' }}>
          {props.reactions.map((reaction) => (
            <View style={{ paddingHorizontal: 5, paddingVertical: 1, borderRadius: 9, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.muted }}>
              <Text style={{ fontSize: 11, color: tokens.foreground }}>{reaction}</Text>
            </View>
          ))}
        </View>
      ) : null}
      {props.actions}
    </View>
  );
}

export function BubbleContent(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return <View style={props.style}>{props.children}</View>;
}
