// A sortable, paginated data table over the basic Table.
import type { StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View, Scroll } from 'nsh';
import { Button } from './button';
import { Pagination } from './pagination';
import { Table, type TableColumn } from './table';
import { tokens } from './theme';

export interface DataTableProps {
  columns: TableColumn[];
  rows: Record<string, string>[];
  pageSize?: number;
  sortable?: boolean;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function DataTable(props: DataTableProps) {
  const page = signal(1);
  const sortKey = signal('');
  const ascending = signal(true);
  const sorted = () => {
    if (!props.sortable || !sortKey()) return props.rows;
    return [...props.rows].sort((left, right) => {
      const a = left[sortKey()] ?? '';
      const b = right[sortKey()] ?? '';
      return ascending() ? a.localeCompare(b, 'en') : b.localeCompare(a, 'en');
    });
  };
  const pageSize = () => props.pageSize ?? 10;
  const pages = () => Math.max(1, Math.ceil(props.rows.length / pageSize()));
  const visible = () => sorted().slice((page() - 1) * pageSize(), page() * pageSize());
  return (
    <View testID={props.testID} style={[{ gap: 10 }, props.style]}>
      <View style={{ flexDirection: 'row', gap: 6 }}>
        {props.columns.map((column) => (
          <View
            accessibilityRole="button"
            accessibilityLabel={`Sort by ${column.label}`}
            onPress={() => {
              if (!props.sortable) return;
              if (sortKey() === column.key) ascending.set(!ascending());
              else { sortKey.set(column.key); ascending.set(true); }
            }}
            style={{ flex: 1, paddingVertical: 6, paddingHorizontal: 8, backgroundColor: tokens.muted, borderRadius: 4 }}
          >
            <Text style={{ fontSize: 12, fontWeight: '600', color: tokens.mutedForeground }}>
              {column.label}{sortKey() === column.key ? (ascending() ? ' ↑' : ' ↓') : ''}
            </Text>
          </View>
        ))}
      </View>
      <Scroll style={{ maxHeight: 420 }}>
        <Table columns={props.columns} rows={visible()} bordered striped />
      </Scroll>
      <Pagination page={page()} totalPages={pages()} onPageChange={(next) => page.set(next)} />
    </View>
  );
}
