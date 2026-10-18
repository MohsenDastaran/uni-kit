// Two resizable panes with a draggable divider.
import type { JSX, LayoutEvent, StyleProp, ViewStyle } from 'nsh';
import { signal, View } from 'nsh';
import { tokens } from './theme';

export interface ResizableProps {
  left: JSX.Element;
  right: JSX.Element;
  initial?: number;
  min?: number;
  max?: number;
  vertical?: boolean;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Resizable(props: ResizableProps) {
  const size = signal(props.initial ?? 50);
  const extent = signal(600);
  const dragging = signal(false);
  const setFrom = (position: number) => {
    const ratio = props.vertical ? position / Math.max(1, extent()) : position / Math.max(1, extent());
    size.set(Math.max(props.min ?? 15, Math.min(props.max ?? 85, ratio * 100)));
  };
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
        onTouchStart={(event) => {
          dragging.set(true);
          setFrom(props.vertical ? event.locationY : event.locationX);
        }}
        onTouchMove={(event) => {
          if (dragging()) setFrom(props.vertical ? event.locationY : event.locationX);
        }}
        onTouchEnd={() => dragging.set(false)}
        style={{
          ...(props.vertical ? { height: 8, width: '100%', cursor: 'row-resize' } : { width: 8, height: '100%', cursor: 'col-resize' }),
          backgroundColor: tokens.border,
        }}
      />
      <View style={{ flex: 1, overflow: 'hidden' }}>{props.right}</View>
    </View>
  );
}
