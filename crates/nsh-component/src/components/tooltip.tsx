// A short label over a trigger. On the web (no hover events) a press/long press
// reveals it; on desktops the pointer does.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { RADIUS_SM, tokens } from './theme';

export interface TooltipProps {
  content: string;
  children: JSX.Element;
  placement?: 'top' | 'bottom' | 'start' | 'end';
  delay?: number;
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?: (open: boolean) => void;
  style?: StyleProp<ViewStyle>;
}

export function Tooltip(props: TooltipProps) {
  const open = signal(Boolean(props.defaultOpen));
  const isOpen = () => props.open ?? open();
  let timer: ReturnType<typeof setTimeout> | undefined;
  const show = () => {
    clearTimeout(timer);
    timer = setTimeout(() => {
      if (props.open === undefined) open.set(true);
      props.onOpenChange?.(true);
    }, props.delay ?? 300);
  };
  const hide = () => {
    clearTimeout(timer);
    if (props.open === undefined) open.set(false);
    props.onOpenChange?.(false);
  };
  return (
    <View style={{ position: 'relative', alignSelf: 'flex-start', zIndex: open() ? 30 : 0 }}>
      <View onHoverIn={show} onHoverOut={hide} onLongPress={() => { const next = !isOpen(); if (props.open === undefined) open.set(next); props.onOpenChange?.(next); }}>
        {props.children}
      </View>
      {isOpen() ? (
        <View
          class="shadow-lg"
          style={[
            {
              position: 'absolute',
              zIndex: 40,
              paddingHorizontal: 6,
              paddingVertical: 3,
              borderRadius: RADIUS_SM,
              backgroundColor: tokens.foreground,
              ...(props.placement === 'bottom'
                ? { top: 30, left: 0 }
                : props.placement === 'start'
                  ? { top: 0, right: 30 }
                  : props.placement === 'end'
                    ? { top: 0, left: 30 }
                    : { bottom: 30, left: 0 }),
            },
            props.style,
          ]}
        >
          <Text style={{ fontSize: 11, color: tokens.background }}>{props.content}</Text>
        </View>
      ) : null}
    </View>
  );
}
