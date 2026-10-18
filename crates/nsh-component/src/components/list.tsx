// A styled row list. The mobile ListView owns recycling, dividers and padding.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { ListView } from 'nsh/ui/mobile';
import { tokens } from './theme';

export interface ListProps<T> {
  items: T[];
  renderItem?: (item: T, index: number) => JSX.Element;
  keyExtractor?: (item: T, index: number) => string;
  variant?: 'plain' | 'inset';
  dividers?: boolean;
  header?: JSX.Element;
  empty?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function List<T>(props: ListProps<T>) {
  return (
    <ListView
      testID={props.testID}
      each={props.items}
      keyExtractor={props.keyExtractor}
      variant={props.variant}
      dividers={props.dividers}
      ListHeaderComponent={props.header}
      ListEmptyComponent={props.empty}
      style={props.style}
    >
      {(item, index) => (props.renderItem ? props.renderItem(item(), index()) : <Text style={{ fontSize: 14, color: tokens.foreground }}>{String(item())}</Text>)}
    </ListView>
  );
}

export function ListRow(props: { title: string; description?: string; trailing?: JSX.Element; onPress?: () => void }) {
  return (
    <View onPress={props.onPress} style={{ flexDirection: 'row', alignItems: 'center', gap: 8 }}>
      <View style={{ flex: 1, gap: 1 }}>
        <Text style={{ fontSize: 14, color: tokens.foreground }}>{props.title}</Text>
        {props.description ? <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>{props.description}</Text> : null}
      </View>
      {props.trailing}
    </View>
  );
}
