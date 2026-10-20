// A virtualized list over the native recycling List. The caller supplies a
// fixed row height; only the rows near the viewport exist.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { View } from 'nsh';
import { List as NativeList } from 'nsh/list';
import { ListRow } from './list';
import { tokens } from './theme';

export interface VirtualListProps<T> {
  items: T[];
  itemHeight?: number;
  renderItem?: (item: T, index: number) => JSX.Element;
  keyExtractor?: (item: T, index: number) => string;
  header?: JSX.Element;
  footer?: JSX.Element;
  empty?: JSX.Element;
  /** Horizontal padding around rows. */
  contentPadding?: number;
  refreshing?: boolean;
  onRefresh?: () => void;
  onEndReached?: () => void;
  onEndReachedThreshold?: number;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function VirtualList<T>(props: VirtualListProps<T>) {
  const height = () => props.itemHeight ?? 52;
  return (
    <NativeList
      each={props.items}
      estimatedItemSize={height()}
      keyExtractor={props.keyExtractor}
      ListHeaderComponent={props.header}
      ListFooterComponent={props.footer}
      ListEmptyComponent={props.empty}
      contentContainerStyle={{ paddingHorizontal: props.contentPadding ?? 12 }}
      refreshing={props.refreshing}
      onRefresh={props.onRefresh}
      onEndReached={props.onEndReached}
      onEndReachedThreshold={props.onEndReachedThreshold}
      ItemSeparatorComponent={
        <View style={{ height: 1, backgroundColor: tokens.border, marginHorizontal: 2 }} />
      }
      style={props.style}
    >
      {(item, index) =>
        props.renderItem ? (
          props.renderItem(item(), index())
        ) : (
          <View style={{ height: height(), justifyContent: 'center' }}>
            <ListRow title={String(item())} />
          </View>
        )
      }
    </NativeList>
  );
}
