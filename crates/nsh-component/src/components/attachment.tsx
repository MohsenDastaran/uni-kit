// One file or media attachment: a quiet bordered card (or a compact composer
// chip) with a file/image preview, filename, metadata, lifecycle state and the
// actions a composer or assistant reply needs — download, retry and remove.
//
// The component owns presentation only. Upload, scanning, retry, removal and
// preview behavior stay in the application; every handler is optional.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Image } from 'nsh/image';
import { Icon } from './icon';
import { type IconName } from './icons';
import { Button } from './button';
import { alpha, isDark, tokens } from './theme';

export type AttachmentStatus = 'idle' | 'uploading' | 'scanning' | 'complete' | 'error';

/** The card's density. `medium` is the conversation default. */
export type AttachmentSize = 'small' | 'medium' | 'large';

/** `card` is the bordered surface; `chip` is the compact composer/citation form. */
export type AttachmentVariant = 'card' | 'chip';

/** `row` keeps the preview beside the metadata; `stacked` puts it above. */
export type AttachmentLayout = 'row' | 'stacked';

const SIZES: AttachmentSize[] = ['small', 'medium', 'large'];

/**
 * Whether a `size` value names a density rather than a file size. The prop
 * historically carried the human-readable file size ("PDF · 1.9 MB"), so a
 * string that is not one of the names is still read as metadata.
 */
export function isAttachmentSize(value: unknown): value is AttachmentSize {
  return typeof value === 'string' && (SIZES as string[]).includes(value);
}

export interface AttachmentProps {
  name?: string;
  /**
   * Card density (`small`, `medium`, `large`). A legacy file-size string such
   * as `"1.9 MB"` keeps working: it is read as metadata instead.
   */
  size?: AttachmentSize | (string & {});
  /** The metadata line, e.g. `"PDF · 1.9 MB"` or `"PNG · 1280 × 720"`. */
  meta?: string;
  icon?: IconName;
  image?: string | { uri: string };
  status?: AttachmentStatus;
  /** 0–100, used by `uploading` and `scanning`. */
  progress?: number;
  variant?: AttachmentVariant;
  layout?: AttachmentLayout;
  /** Show the download action without supplying a handler. */
  downloadable?: boolean;
  onDownload?: () => void;
  /** Activate the card's content, e.g. to open a preview. Actions stay separate. */
  onPress?: () => void;
  onRemove?: () => void;
  onRetry?: () => void;
  /** An explicit trailing action, composed before the built-in actions. */
  action?: JSX.Element;
  /** Extra content under the metadata line, e.g. a reason or a badge. */
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const mediaIcon = (name: string | undefined): IconName => {
  const extension = (name ?? '').split('.').pop()?.toLowerCase() ?? '';
  if (['png', 'jpg', 'jpeg', 'gif', 'webp', 'svg', 'avif'].includes(extension)) return 'image';
  if (['zip', 'rar', '7z', 'tar', 'gz'].includes(extension)) return 'file';
  if (['csv', 'xls', 'xlsx'].includes(extension)) return 'table';
  if (['ts', 'tsx', 'js', 'jsx', 'json', 'md', 'txt'].includes(extension)) return 'file-text';
  return 'file-text';
};

interface Metrics {
  tile: number;
  padding: number;
  gap: number;
  title: number;
  detail: number;
  radius: number;
  icon: number;
  action: 'xsmall' | 'small';
}

const metricsOf = (size: AttachmentSize, variant: AttachmentVariant): Metrics => {
  if (variant === 'chip') {
    return {
      tile: 20,
      padding: 5,
      gap: 6,
      title: 12,
      detail: 10,
      radius: 8,
      icon: 13,
      action: 'xsmall',
    };
  }
  switch (size) {
    case 'small':
      return {
        tile: 28,
        padding: 8,
        gap: 8,
        title: 12,
        detail: 10,
        radius: 8,
        icon: 14,
        action: 'xsmall',
      };
    case 'large':
      return {
        tile: 46,
        padding: 14,
        gap: 12,
        title: 14,
        detail: 12,
        radius: 10,
        icon: 22,
        action: 'small',
      };
    default:
      return {
        tile: 38,
        padding: 10,
        gap: 10,
        title: 13,
        detail: 11,
        radius: 10,
        icon: 17,
        action: 'xsmall',
      };
  }
};

export function Attachment(props: AttachmentProps) {
  const hovering = signal(false);
  const status = () => props.status ?? 'idle';
  const variant = (): AttachmentVariant => props.variant ?? 'card';
  const layout = (): AttachmentLayout => props.layout ?? 'row';
  const size = (): AttachmentSize => (isAttachmentSize(props.size) ? props.size : 'medium');
  const metrics = () => metricsOf(size(), variant());
  // A legacy `size="1.9 MB"` is metadata, not density.
  const meta = () =>
    props.meta ?? (!isAttachmentSize(props.size) && props.size ? props.size : undefined);
  const name = () => props.name ?? 'Attachment';
  const percent = () => Math.max(0, Math.min(100, props.progress ?? 0));
  const inFlight = () => status() === 'uploading' || status() === 'scanning';
  const tone = () => {
    switch (status()) {
      case 'error':
        return tokens.danger;
      case 'scanning':
        return tokens.info;
      case 'uploading':
        return tokens.primary;
      case 'complete':
        return tokens.success;
      default:
        return tokens.mutedForeground;
    }
  };
  const detail = () => {
    switch (status()) {
      case 'uploading':
        return props.progress === undefined
          ? 'Uploading'
          : `Uploading · ${Math.round(percent())}%`;
      case 'scanning':
        return props.progress === undefined
          ? 'Scanning for threats'
          : `Scanning · ${Math.round(percent())}%`;
      case 'error':
        return 'Upload failed';
      case 'complete':
        return meta() ?? 'Uploaded';
      default:
        return meta() ?? '';
    }
  };
  const statusIcon = (): IconName | undefined => {
    switch (status()) {
      case 'uploading':
        return 'loader-circle';
      case 'scanning':
        return 'shield';
      case 'error':
        return 'circle-alert';
      case 'complete':
        return 'circle-check';
      default:
        return undefined;
    }
  };
  const showDownload = () =>
    Boolean(props.downloadable || props.onDownload) && !inFlight() && status() !== 'error';
  const tileFill = () => {
    switch (status()) {
      case 'error':
        return alpha(tokens.danger, 0.1);
      case 'scanning':
        return alpha(tokens.info, 0.1);
      case 'uploading':
        return alpha(tokens.primary, 0.08);
      default:
        return tokens.muted;
    }
  };
  const borderTone = () => {
    switch (status()) {
      case 'error':
        return alpha(tokens.danger, 0.55);
      case 'scanning':
        return alpha(tokens.info, 0.4);
      case 'uploading':
        return alpha(tokens.primary, 0.4);
      default:
        return tokens.border;
    }
  };
  const surface = () => {
    if (props.onPress && hovering()) return isDark() ? alpha(tokens.accent, 0.6) : tokens.accent;
    return tokens.background;
  };

  const tile = () => {
    const m = metrics();
    const radius = Math.max(4, m.radius - 2);
    if (props.image) {
      return (
        <Image
          src={props.image}
          resizeMode="cover"
          style={{ width: m.tile, height: m.tile, borderRadius: radius }}
        />
      );
    }
    return (
      <View
        style={{
          width: m.tile,
          height: m.tile,
          borderRadius: radius,
          backgroundColor: tileFill(),
          alignItems: 'center',
          justifyContent: 'center',
        }}
      >
        <Icon
          name={props.icon ?? mediaIcon(props.name)}
          size={m.icon}
          color={
            status() === 'idle' || status() === 'complete' ? tokens.mutedForeground : tone()
          }
        />
      </View>
    );
  };

  const progressBar = () =>
    inFlight() ? (
      <View
        style={{
          height: 3,
          marginTop: 3,
          borderRadius: 2,
          overflow: 'hidden',
          backgroundColor: alpha(tone(), 0.18),
        }}
      >
        <View
          class="transition-all duration-150 ease-out"
          style={{
            width: `${percent()}%`,
            height: '100%',
            borderRadius: 2,
            backgroundColor: tone(),
          }}
        />
      </View>
    ) : null;

  const download = () =>
    showDownload() ? (
      <Button
        icon="download"
        size={metrics().action}
        variant="ghost"
        accessibilityLabel={`Download ${name()}`}
        onPress={props.onDownload}
      />
    ) : null;

  const retry = () =>
    status() === 'error' && props.onRetry ? (
      <Button
        icon="refresh-cw"
        size={metrics().action}
        variant="ghost"
        accessibilityLabel={`Retry ${name()}`}
        onPress={props.onRetry}
      />
    ) : null;

  const remove = () =>
    props.onRemove ? (
      <Button
        icon="x"
        size={metrics().action}
        variant="ghost"
        accessibilityLabel={`Remove ${name()}`}
        onPress={props.onRemove}
      />
    ) : null;

  const actions = (
    <View style={{ flexDirection: 'row', alignItems: 'center', gap: 2 }}>
      {props.action}
      {download()}
      {retry()}
      {remove()}
    </View>
  );

  const statusLine = () => {
    const text = detail();
    const icon = statusIcon();
    if (!text) return null;
    return (
      <View style={{ flexDirection: 'row', alignItems: 'center', gap: 4, marginTop: 1 }}>
        {icon ? (
          <Icon
            name={icon}
            size={Math.max(11, metrics().detail)}
            color={tone()}
            spinning={status() === 'uploading'}
          />
        ) : null}
        <Text
          numberOfLines={1}
          style={{
            flex: 1,
            fontSize: metrics().detail,
            color: status() === 'idle' || status() === 'complete' ? tokens.mutedForeground : tone(),
          }}
        >
          {text}
        </Text>
      </View>
    );
  };

  const content = () => (
    <View style={{ flex: 1, minWidth: 0, gap: 2 }}>
      <Text
        numberOfLines={1}
        style={{ fontSize: metrics().title, fontWeight: '500', color: tokens.foreground }}
      >
        {name()}
      </Text>
      {statusLine()}
      {progressBar()}
      {props.children}
    </View>
  );

  if (variant() === 'chip') {
    return (
      <View
        testID={props.testID}
        accessibilityLabel={name()}
        class="transition-colors duration-150 ease-out"
        style={[
          {
            flexDirection: 'row',
            alignItems: 'center',
            alignSelf: 'flex-start',
            gap: metrics().gap,
            paddingVertical: metrics().padding,
            paddingLeft: metrics().padding,
            paddingRight: 3,
            borderRadius: metrics().radius,
            borderWidth: 1,
            borderColor: borderTone(),
            backgroundColor: tokens.muted,
          },
          props.style,
        ]}
      >
        {tile()}
        <Text
          numberOfLines={1}
          style={{ maxWidth: 190, fontSize: metrics().title, color: tokens.foreground }}
        >
          {name()}
        </Text>
        {statusIcon() ? (
          <Icon
            name={statusIcon() as IconName}
            size={12}
            color={tone()}
            spinning={status() === 'uploading'}
          />
        ) : null}
        {showDownload() ? (
          <Button
            icon="download"
            size="xsmall"
            variant="ghost"
            accessibilityLabel={`Download ${name()}`}
            onPress={props.onDownload}
          />
        ) : null}
        {remove()}
      </View>
    );
  }

  if (layout() === 'stacked') {
    const m = metrics();
    return (
      <View
        testID={props.testID}
        accessibilityLabel={name()}
        onHoverIn={() => {
          if (props.onPress) hovering.set(true);
        }}
        onHoverOut={() => hovering.set(false)}
        class="transition-colors duration-150 ease-out"
        style={[
          {
            borderRadius: m.radius,
            borderWidth: 1,
            borderColor: borderTone(),
            backgroundColor: surface(),
            overflow: 'hidden',
            alignSelf: 'flex-start',
          },
          props.style,
        ]}
      >
        <View
          accessibilityRole={props.onPress ? 'button' : undefined}
          accessibilityLabel={props.onPress ? `Open ${name()}` : undefined}
          onPress={props.onPress}
          activeOpacity={props.onPress ? 0.85 : 1}
          style={{ width: '100%', aspectRatio: 4 / 3, backgroundColor: tileFill() }}
        >
          {props.image ? (
            <Image
              src={props.image}
              resizeMode="cover"
              style={{ width: '100%', height: '100%' }}
            />
          ) : (
            <View style={{ flex: 1, alignItems: 'center', justifyContent: 'center' }}>
              <Icon
                name={props.icon ?? mediaIcon(props.name)}
                size={Math.max(24, m.icon + 8)}
                color={status() === 'idle' ? tokens.mutedForeground : tone()}
              />
            </View>
          )}
        </View>
        <View style={{ padding: m.padding, gap: m.gap }}>
          <View style={{ flexDirection: 'row', alignItems: 'flex-start', gap: 8 }}>
            {content()}
            {actions}
          </View>
        </View>
      </View>
    );
  }

  return (
    <View
      testID={props.testID}
      accessibilityLabel={name()}
      onHoverIn={() => {
        if (props.onPress) hovering.set(true);
      }}
      onHoverOut={() => hovering.set(false)}
      class="transition-colors duration-150 ease-out"
      style={[
        {
          flexDirection: 'row',
          alignItems: 'center',
          gap: metrics().gap,
          padding: metrics().padding,
          borderRadius: metrics().radius,
          borderWidth: 1,
          borderColor: borderTone(),
          backgroundColor: surface(),
        },
        props.style,
      ]}
    >
      <View
        accessibilityRole={props.onPress ? 'button' : undefined}
        accessibilityLabel={props.onPress ? `Open ${name()}` : undefined}
        onPress={props.onPress}
        activeOpacity={props.onPress ? 0.85 : 1}
        style={{
          flex: 1,
          minWidth: 0,
          flexDirection: 'row',
          alignItems: 'center',
          gap: metrics().gap,
        }}
      >
        {tile()}
        {content()}
      </View>
      {actions}
    </View>
  );
}

export interface AttachmentGroupProps {
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  /** `column` is the message/queue stack; `row` is the composer chip tray. */
  direction?: 'row' | 'column';
  gap?: number;
  wrap?: boolean;
}

/** A shared row or column of attachments; it owns spacing only. */
export function AttachmentGroup(props: AttachmentGroupProps) {
  return (
    <View
      style={[
        {
          flexDirection: props.direction ?? 'column',
          gap: props.gap ?? 8,
          ...(props.wrap ? { flexWrap: 'wrap' as const } : {}),
        },
        props.style,
      ]}
    >
      {props.children}
    </View>
  );
}
