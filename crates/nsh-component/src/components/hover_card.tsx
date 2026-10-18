// A richer card revealed by a trigger. Hover on desktops; press/long press on
// the web, where the hover events are not raised.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { RADIUS_MD, tokens } from './theme';

export interface HoverCardProps {
  trigger: JSX.Element;
  children?: JSX.Element;
  placement?: 'bottom' | 'top';
  delay?: number;
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?: (open: boolean) => void;
  style?: StyleProp<ViewStyle>;
}

export function HoverCard(props: HoverCardProps) {
  const open = signal(Boolean(props.defaultOpen));
  const isOpen = () => props.open ?? open();
  let timer: ReturnType<typeof setTimeout> | undefined;
  const show = () => {
    clearTimeout(timer);
    timer = setTimeout(() => {
      if (props.open === undefined) open.set(true);
      props.onOpenChange?.(true);
    }, props.delay ?? 250);
  };
  const hide = () => {
    clearTimeout(timer);
    if (props.open === undefined) open.set(false);
    props.onOpenChange?.(false);
  };
  return (
    <View style={{ position: 'relative', alignSelf: 'flex-start', zIndex: open() ? 30 : 0 }}>
      <View onHoverIn={show} onHoverOut={hide} onLongPress={() => { const next = !isOpen(); if (props.open === undefined) open.set(next); props.onOpenChange?.(next); }}>
        {props.trigger}
      </View>
      {isOpen() ? (
        <View
          class="shadow-lg"
          style={[
            {
              position: 'absolute',
              zIndex: 40,
              width: 260,
              padding: 12,
              gap: 8,
              borderRadius: RADIUS_MD,
              borderWidth: 1,
              borderColor: tokens.border,
              backgroundColor: tokens.popover,
              ...(props.placement === 'top' ? { bottom: 36, left: 0 } : { top: 36, left: 0 }),
            },
            props.style,
          ]}
        >
          {props.children}
        </View>
      ) : null}
    </View>
  );
}

export function HoverCardTitle(props: { children?: JSX.Element }) {
  return <Text style={{ fontSize: 14, fontWeight: '600', color: tokens.foreground }}>{props.children}</Text>;
}

export function HoverCardDescription(props: { children?: JSX.Element }) {
  return <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>{props.children}</Text>;
}
