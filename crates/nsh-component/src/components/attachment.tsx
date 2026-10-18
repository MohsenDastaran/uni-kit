// One file or media attachment with a preview, metadata, lifecycle state and
// remove/retry actions.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Image } from 'nsh/image';
import { Icon } from './icon';
import { type IconName } from './icons';
import { Button } from './button';
import { alpha, FONT_SIZE_SM, RADIUS_MD, tokens } from './theme';

export type AttachmentStatus = 'idle' | 'uploading' | 'complete' | 'error';

export interface AttachmentProps {
  name?: string;
  size?: string;
  icon?: IconName;
  image?: string | { uri: string };
  status?: AttachmentStatus;
  progress?: number;
  onRemove?: () => void;
  onRetry?: () => void;
  action?: JSX.Element;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const toneOf = (status: AttachmentStatus): string =>
  status === 'error' ? tokens.danger : status === 'complete' ? tokens.success : tokens.mutedForeground;

export function Attachment(props: AttachmentProps) {
  const status = () => props.status ?? 'idle';
  return (
    <View
      testID={props.testID}
      accessibilityLabel={props.name}
      style={[
        {
          flexDirection: 'row',
          alignItems: 'center',
          gap: 10,
          padding: 10,
          borderRadius: RADIUS_MD,
          borderWidth: 1,
          borderColor: status() === 'error' ? alpha(tokens.danger, 0.5) : tokens.border,
          backgroundColor: tokens.background,
        },
        props.style,
      ]}
    >
      {props.image ? (
        <Image src={props.image} resizeMode="cover" style={{ width: 44, height: 44, borderRadius: 6 }} />
      ) : (
        <View style={{ width: 44, height: 44, borderRadius: 6, backgroundColor: tokens.muted, alignItems: 'center', justifyContent: 'center' }}>
          <Icon name={props.icon ?? 'file'} size={20} color={tokens.mutedForeground} />
        </View>
      )}
      <View style={{ flex: 1, gap: 2 }}>
        <Text style={{ fontSize: FONT_SIZE_SM, color: tokens.foreground }}>{props.name ?? 'Attachment'}</Text>
        <Text style={{ fontSize: 11, color: toneOf(status()) }}>
          {status() === 'uploading' ? `Uploading ${Math.round(props.progress ?? 0)}%` : status() === 'error' ? 'Upload failed' : status() === 'complete' ? 'Uploaded' : props.size ?? ''}
        </Text>
        {status() === 'uploading' ? (
          <View style={{ height: 4, borderRadius: 2, backgroundColor: tokens.muted, overflow: 'hidden' }}>
            <View style={{ width: `${Math.max(0, Math.min(100, props.progress ?? 0))}%`, height: '100%', backgroundColor: tokens.primary }} />
          </View>
        ) : null}
      </View>
      {props.children ?? props.action}
      {status() === 'error' ? <Button label="Retry" size="small" variant="ghost" onPress={props.onRetry} /> : null}
      {props.onRemove ? <Button icon="x" size="small" variant="ghost" accessibilityLabel="Remove" onPress={props.onRemove} /> : null}
    </View>
  );
}

export function AttachmentGroup(props: { children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return <View style={[{ gap: 8 }, props.style]}>{props.children}</View>;
}
