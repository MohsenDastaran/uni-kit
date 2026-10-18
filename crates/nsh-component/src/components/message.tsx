// A whole chat turn: identity, metadata, bubble and actions.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Avatar } from './avatar';
import { Bubble } from './bubble';
import { MessageRole, type MessageRoleName } from './message_role';
import { tokens } from './theme';

export interface MessageProps {
  role?: MessageRoleName;
  name?: string;
  time?: string;
  avatar?: string | { uri: string };
  content?: string;
  children?: JSX.Element;
  actions?: JSX.Element;
  reactions?: string[];
  align?: 'start' | 'end';
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Message(props: MessageProps) {
  const role = () => props.role ?? 'assistant';
  const end = () => (props.align ?? (role() === 'user' ? 'end' : 'start')) === 'end';
  return (
    <View
      testID={props.testID}
      style={[{ gap: 4, maxWidth: '100%', alignItems: end() ? 'flex-end' : 'flex-start' }, props.style]}
    >
      <View style={{ flexDirection: end() ? 'row-reverse' : 'row', alignItems: 'center', gap: 6 }}>
        <MessageRole role={role()} name={props.name} time={props.time} avatar={props.avatar} />
      </View>
      <Bubble align={end() ? 'end' : 'start'} variant={role() === 'user' ? 'primary' : 'default'} content={props.content} reactions={props.reactions}>
        {props.children}
      </Bubble>
      {props.actions ? <View style={{ flexDirection: 'row', gap: 4 }}>{props.actions}</View> : null}
    </View>
  );
}

export function MessageAvatar(props: { name?: string; src?: string | { uri: string } }) {
  return <Avatar name={props.name} src={props.src} size="sm" />;
}

export function MessageMeta(props: { children?: JSX.Element }) {
  return <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>{props.children}</Text>;
}
