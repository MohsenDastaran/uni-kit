// A table: columns, rows, optional stripes, border, rich cell renderers and a
// summary footer. The visual language is the ReUI table — a quiet surface, a
// compact muted header and hairline row separators.
import type { DimensionValue, JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Icon } from './icon';
import { type ControlSize, FONT_SIZE_SM, RADIUS_LG, tokens } from './theme';

export interface TableColumn {
  key: string;
  label: string;
  width?: DimensionValue;
  align?: 'start' | 'center' | 'end';
  /** Draw the cell content yourself; receives the raw string value. */
  render?: (value: string, row: Record<string, string>, index: number) => JSX.Element;
  /** Whether the header can be pressed to sort. */
  sortable?: boolean;
}

export interface TableProps {
  columns: TableColumn[];
  rows: Record<string, string>[];
  striped?: boolean;
  bordered?: boolean;
  /** A bordered card surface; table chrome without the page around it. */
  card?: boolean;
  size?: ControlSize;
  footer?: Record<string, string>;
  /** A footer note, e.g. "A list of your recent invoices." */
  caption?: string;
  emptyText?: string;
  selectedKeys?: string[];
  rowKey?: string;
  onRowPress?: (row: Record<string, string>, index: number) => void;
  sortKey?: string;
  sortAscending?: boolean;
  onSort?: (key: string) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Table(props: TableProps) {
  const cellStyle = (align?: string) => ({
    flex: 1,
    minWidth: 0,
    textAlign: (align === 'end' ? 'right' : align === 'center' ? 'center' : 'left') as 'left' | 'center' | 'right',
    fontSize: FONT_SIZE_SM,
    color: tokens.foreground,
  });
  return (
    <View
      testID={props.testID}
      style={[
        {
          borderWidth: props.bordered || props.card ? 1 : 0,
          borderColor: tokens.border,
          borderRadius: RADIUS_LG,
          overflow: 'hidden',
          backgroundColor: props.card ? tokens.popover : undefined,
        },
        props.style,
      ]}
    >
      <View
        style={{
          flexDirection: 'row',
          gap: 8,
          paddingHorizontal: 10,
          paddingVertical: 7,
          borderBottomWidth: 1,
          borderBottomColor: tokens.border,
        }}
      >
        {props.columns.map((column) => (
          <View
            accessibilityRole={column.sortable ? 'button' : undefined}
            accessibilityLabel={column.sortable ? `Sort by ${column.label}` : undefined}
            onPress={column.sortable ? () => props.onSort?.(column.key) : undefined}
            style={{ flex: 1, minWidth: 0, width: column.width, flexDirection: 'row', alignItems: 'center', gap: 2 }}
          >
            <Text
              style={{
                ...cellStyle(column.align),
                flex: undefined,
                fontWeight: '500',
                fontSize: 11,
                color: tokens.mutedForeground,
                width: column.width,
              }}
            >
              {column.label}
            </Text>
            {column.sortable && props.sortKey === column.key ? (
              <Icon name={props.sortAscending ? 'chevron-up' : 'chevron-down'} size={10} color={tokens.mutedForeground} />
            ) : null}
          </View>
        ))}
      </View>
      {props.rows.length === 0 ? (
        <Text style={{ paddingHorizontal: 10, paddingVertical: 18, fontSize: 12, color: tokens.mutedForeground }}>
          {props.emptyText ?? 'No rows'}
        </Text>
      ) : (
        props.rows.map((row, index) => {
          const key = props.rowKey ? row[props.rowKey] : String(index);
          const selected = props.selectedKeys?.includes(key) ?? false;
          return (
            <View
              accessibilityRole={props.onRowPress ? 'button' : undefined}
              accessibilityLabel={props.onRowPress ? row[props.columns[0]?.key ?? ''] : undefined}
              onPress={props.onRowPress ? () => props.onRowPress?.(row, index) : undefined}
              activeOpacity={0.9}
              class="transition-colors duration-150 hover:bg-black/[0.025] dark:hover:bg-white/[0.035]"
              style={{
                flexDirection: 'row',
                gap: 8,
                paddingHorizontal: 10,
                paddingVertical: props.size === 'small' ? 6 : 9,
                backgroundColor: selected
                  ? tokens.accent
                  : props.striped && index % 2 === 1
                    ? tokens.muted
                    : tokens.background,
                borderTopWidth: 1,
                borderTopColor: tokens.border,
              }}
            >
              {props.columns.map((column) => (
                <View style={{ flex: 1, minWidth: 0, width: column.width }}>
                  {column.render ? (
                    column.render(row[column.key] ?? '', row, index)
                  ) : (
                    <Text style={{ ...cellStyle(column.align), width: column.width }} numberOfLines={1}>
                      {row[column.key] ?? ''}
                    </Text>
                  )}
                </View>
              ))}
            </View>
          );
        })
      )}
      {props.footer ? (
        <View
          style={{
            flexDirection: 'row',
            gap: 8,
            paddingHorizontal: 10,
            paddingVertical: 8,
            borderTopWidth: 1,
            borderTopColor: tokens.border,
            backgroundColor: tokens.muted,
          }}
        >
          {props.columns.map((column) => (
            <Text style={{ ...cellStyle(column.align), width: column.width, fontWeight: '600' }}>
              {props.footer?.[column.key] ?? ''}
            </Text>
          ))}
        </View>
      ) : null}
      {props.caption ? (
        <Text style={{ paddingHorizontal: 10, paddingBottom: 8, paddingTop: props.footer ? 0 : 7, fontSize: 10, color: tokens.mutedForeground }}>
          {props.caption}
        </Text>
      ) : null}
    </View>
  );
}
