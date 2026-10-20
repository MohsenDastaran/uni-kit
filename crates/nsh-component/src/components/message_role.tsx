// The marker a conversation turn shows for the side it came from.
import type { StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Avatar } from './avatar';
import { tokens } from './theme';

export type MessageRoleName = 'user' | 'assistant' | 'system';

export interface MessageRoleProps {
  role?: MessageRoleName;
  name?: string;
  time?: string;
  avatar?: string | { uri: string };
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function MessageRole(props: MessageRoleProps) {
  const role = () => props.role ?? 'assistant';
  return (
    <View
      testID={props.testID}
      style={[{ flexDirection: 'row', alignItems: 'center', gap: 6 }, props.style]}
    >
      <Avatar name={props.name ?? (role() === 'user' ? 'You' : 'Assistant')} src={props.avatar} size="xs" />
      <Text style={{ fontSize: 12, fontWeight: '600', color: tokens.foreground }}>
        {props.name ?? (role() === 'user' ? 'You' : role() === 'system' ? 'System' : 'Assistant')}
      </Text>
      {props.time ? <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>{props.time}</Text> : null}
    </View>
  );
}
