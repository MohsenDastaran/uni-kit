// A virtualized list over the native recycling List.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { List as NativeList } from 'nsh/list';
import { tokens } from './theme';

export interface VirtualListProps<T> {
  items: T[];
  itemHeight?: number;
  renderItem?: (item: T, index: number) => JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function VirtualList<T>(props: VirtualListProps<T>) {
  return (
    <NativeList
      each={props.items}
      estimatedItemSize={props.itemHeight ?? 48}
      style={props.style}
    >
      {(item, index) =>
        props.renderItem ? props.renderItem(item(), index()) : (
          <View style={{ height: props.itemHeight ?? 48, justifyContent: 'center', paddingHorizontal: 12 }}>
            <Text style={{ fontSize: 14, color: tokens.foreground }}>{String(item())}</Text>
          </View>
        )
      }
    </NativeList>
  );
}
