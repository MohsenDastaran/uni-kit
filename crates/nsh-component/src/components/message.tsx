// A whole chat turn: identity, metadata, bubble or plain surface, attachments,
// actions, reactions and a delivery/edit footer.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Avatar } from './avatar';
import { Bubble, type BubbleReaction, type BubbleVariant } from './bubble';
import { type MessageRoleName } from './message_role';
import { tokens } from './theme';

export interface MessageProps {
  role?: MessageRoleName;
  name?: string;
  time?: string;
  avatar?: string | { uri: string };
  content?: string;
  children?: JSX.Element;
  /** Replaces the generated identity row. */
  header?: JSX.Element;
  actions?: JSX.Element;
  reactions?: BubbleReaction[];
  align?: 'start' | 'end';
  /** Hide the name/time/avatar row for grouped turns. */
  showHeader?: boolean;
  /** Hide the avatar but keep the sender name. */
  showAvatar?: boolean;
  /** Override the role's surface. */
  variant?: BubbleVariant;
  /** Render the body as plain text without a bubble surface. */
  plain?: boolean;
  /** A delivery status line such as "Read 09:41". */
  status?: string;
  /** An edit note such as "Edited". */
  edited?: string;
  footer?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Message(props: MessageProps) {
  const role = () => props.role ?? 'assistant';
  const end = () => (props.align ?? (role() === 'user' ? 'end' : 'start')) === 'end';
  const variant = () => {
    if (props.variant) return props.variant;
    if (props.plain) return 'ghost';
    return role() === 'user' ? 'filled' : 'outline';
  };
  const header = () => {
    if (props.header) return props.header;
    if (props.showHeader === false) return undefined;
    if (!props.name && !props.time && !props.avatar) return undefined;
    return undefined;
  };
  const footer = () => {
    if (props.footer) return props.footer;
    if (!props.status && !props.edited) return undefined;
    return (
      <Text style={{ fontSize: 9, lineHeight: 13, color: tokens.mutedForeground }}>
        {props.edited ? props.edited : ''}
        {props.edited && props.status ? ' · ' : ''}
        {props.status ? props.status : ''}
      </Text>
    );
  };
  return (
    <View
      testID={props.testID}
      style={[{ width: '100%', alignItems: end() ? 'flex-end' : 'flex-start' }, props.style]}
    >
      <Bubble
        align={end() ? 'end' : 'start'}
        variant={variant()}
        content={props.content}
        name={props.showHeader === false ? undefined : props.name}
        avatar={props.showHeader === false ? undefined : props.avatar}
        timestamp={props.showHeader === false ? undefined : props.time}
        showAvatar={props.showAvatar}
        header={header()}
        actions={props.actions}
        reactions={props.reactions}
        footer={footer()}
      >
        {props.children}
      </Bubble>
    </View>
  );
}

export function MessageGroup(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return <View style={[{ width: '100%', gap: 6 }, props.style]}>{props.children}</View>;
}

export function MessageAvatar(props: { name?: string; src?: string | { uri: string } }) {
  return <Avatar name={props.name} src={props.src} size="sm" />;
}

export function MessageMeta(props: { children?: JSX.Element }) {
  return <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>{props.children}</Text>;
}
