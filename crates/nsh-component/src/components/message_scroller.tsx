// A conversation scroller that stays at the newest message while content grows.
import type { JSX, StyleProp, ViewStyle, YNode } from 'nsh';
import { effect, Scroll, Text, View } from 'nsh';
import { Message, type MessageProps } from './message';
import { tokens } from './theme';

export interface MessageScrollerProps {
  messages: MessageProps[];
  renderMessage?: (message: MessageProps, index: number) => JSX.Element;
  header?: JSX.Element;
  footer?: JSX.Element;
  empty?: JSX.Element;
  /** Keep the viewport at the newest row when the array grows. */
  autoScroll?: boolean;
  padding?: number;
  gap?: number;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  contentStyle?: StyleProp<ViewStyle>;
}

export function MessageScroller(props: MessageScrollerProps) {
  let scroll: YNode | undefined;
  let previous = -1;
  effect(() => {
    const count = props.messages.length;
    if (count === 0 || props.autoScroll === false) return;
    // Do not scroll before the first layout: the content height is not known
    // yet, and a premature scrollToEnd leaves the viewport blank.
    if (previous < 0) {
      previous = count;
      return;
    }
    if (count === previous) return;
    previous = count;
    requestAnimationFrame(() => scroll?.scrollToEnd({ animated: false }));
  });
  return (
    <Scroll
      ref={(node) => (scroll = node)}
      style={[{ flex: 1, backgroundColor: tokens.background }, props.style]}
    >
      <View
        style={[
          {
            flex: 1,
            justifyContent: 'flex-start',
            gap: props.gap ?? 12,
            padding: props.padding ?? 12,
          },
          props.contentStyle,
        ]}
      >
        {props.header}
        {props.messages.length === 0
          ? props.empty ?? (
              <View style={{ padding: 24, alignItems: 'center' }}>
                <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>No messages yet.</Text>
              </View>
            )
          : props.messages.map((message, index) =>
              props.renderMessage ? props.renderMessage(message, index) : <Message {...message} />,
            )}
        {props.footer}
        {props.children}
      </View>
    </Scroll>
  );
}
