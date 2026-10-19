// Two resizable panes with a draggable divider. The gesture is a PanResponder:
// it uses the movement delta from the grant, so the divider does not jump when
// the layout moves the handle under the pointer.
import type { JSX, LayoutEvent, StyleProp, ViewStyle } from 'nsh';
import { signal, View } from 'nsh';
import { PanResponder } from 'nsh/gesture';
import { tokens } from './theme';

export interface ResizableProps {
  left: JSX.Element;
  right: JSX.Element;
  initial?: number;
  min?: number;
  max?: number;
  vertical?: boolean;
  onResize?: (percent: number) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Resizable(props: ResizableProps) {
  const size = signal(props.initial ?? 50);
  const extent = signal(600);
  const dragging = signal(false);
  let start = props.initial ?? 50;

  const clamp = (value: number) => Math.max(props.min ?? 15, Math.min(props.max ?? 85, value));
  const pan = PanResponder.create({
    onStartShouldSetPanResponder: () => true,
    onMoveShouldSetPanResponder: () => true,
    onPanResponderGrant: () => {
      start = size();
      dragging.set(true);
    },
    onPanResponderMove: (_event, gesture) => {
      const delta = props.vertical ? gesture.dy : gesture.dx;
      const next = clamp(start + (delta / Math.max(1, extent())) * 100);
      size.set(next);
      props.onResize?.(next);
    },
    onPanResponderRelease: () => dragging.set(false),
    onPanResponderTerminate: () => dragging.set(false),
  });

  return (
    <View
      testID={props.testID}
      onLayout={(event: LayoutEvent) => extent.set(props.vertical ? event.height : event.width)}
      style={[
        {
          flex: 1,
          flexDirection: props.vertical ? 'column' : 'row',
          borderWidth: 1,
          borderColor: tokens.border,
          borderRadius: 8,
          overflow: 'hidden',
        },
        props.style,
      ]}
    >
      <View style={{ flexBasis: `${size()}%`, flexGrow: 0, flexShrink: 0, overflow: 'hidden' }}>{props.left}</View>
      <View
        {...pan.panHandlers}
        accessibilityRole="adjustable"
        accessibilityLabel={props.vertical ? 'Resize panes vertically' : 'Resize panes horizontally'}
        activeOpacity={0.9}
        style={{
          ...(props.vertical
            ? { height: 8, width: '100%', cursor: 'row-resize' }
            : { width: 8, height: '100%', cursor: 'col-resize' }),
          backgroundColor: dragging() ? tokens.ring : tokens.border,
          alignItems: 'center',
          justifyContent: 'center',
        }}
      >
        <View
          style={{
            ...(props.vertical ? { height: 2, width: 28 } : { width: 2, height: 28 }),
            borderRadius: 1,
            backgroundColor: tokens.mutedForeground,
          }}
        />
      </View>
      <View style={{ flex: 1, overflow: 'hidden' }}>{props.right}</View>
    </View>
  );
}
