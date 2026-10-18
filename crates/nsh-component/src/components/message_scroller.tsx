// A list of messages that stays at the newest one while content grows.
import type { JSX, ScrollProps, StyleProp, ViewStyle } from 'nsh';
import { Scroll, View } from 'nsh';
import { Message, type MessageProps } from './message';
import { tokens } from './theme';

export interface MessageScrollerProps {
  messages: MessageProps[];
  renderMessage?: (message: MessageProps, index: number) => JSX.Element;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
}

export function MessageScroller(props: MessageScrollerProps) {
  const scroll: ScrollProps = {
    style: [{ flex: 1, backgroundColor: tokens.background }, props.style],
  };
  return (
    <Scroll {...scroll}>
      <View style={{ flex: 1, justifyContent: 'flex-end', gap: 12, padding: 12 }}>
        {props.messages.map((message, index) =>
          props.renderMessage ? props.renderMessage(message, index) : <Message {...message} />,
        )}
        {props.children}
      </View>
    </Scroll>
  );
}
