// A sortable, searchable, filterable, paginated table built on Table. The
// toolbar and filter chips are inline — no modal or dialog.
import type { StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Button } from './button';
import { Input } from './input';
import { Pagination } from './pagination';
import { Table, type TableColumn } from './table';
import { alpha, isDark, SPACING_SM, tokens } from './theme';

export interface DataTableFilter {
  key: string;
  label: string;
  options: string[];
}

export interface DataTableProps {
  columns: TableColumn[];
  rows: Record<string, string>[];
  pageSize?: number;
  pageSizeOptions?: number[];
  sortable?: boolean;
  initialSortKey?: string;
  initialSortAscending?: boolean;
  searchable?: boolean;
  searchPlaceholder?: string;
  filters?: DataTableFilter[];
  title?: string;
  description?: string;
  emptyText?: string;
  /** A card surface around the toolbar and table. */
  card?: boolean;
  showToolbar?: boolean;
  selectedKeys?: string[];
  rowKey?: string;
  onRowPress?: (row: Record<string, string>, index: number) => void;
  onFilterChange?: (filters: Record<string, string>) => void;
  onSearchChange?: (query: string) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const compare = (a: string, b: string) => {
  const na = Number(a.replace(/[^0-9.-]/g, ''));
  const nb = Number(b.replace(/[^0-9.-]/g, ''));
  if (!Number.isNaN(na) && !Number.isNaN(nb) && a.trim() !== '' && b.trim() !== '') return na - nb;
  return a.localeCompare(b, 'en');
};

export function DataTable(props: DataTableProps) {
  const page = signal(1);
  const sortKey = signal(props.initialSortKey ?? '');
  const ascending = signal(props.initialSortAscending ?? true);
  const query = signal('');
  const filterValues = signal<Record<string, string>>({});
  const size = signal(props.pageSize ?? 10);

  const filtered = () => {
    const needle = query().trim().toLowerCase();
    const active = filterValues();
    return props.rows.filter((row) => {
      if (needle) {
        const haystack = Object.values(row).join(' ').toLowerCase();
        if (!haystack.includes(needle)) return false;
      }
      return (props.filters ?? []).every((filter) => {
        const chosen = active[filter.key];
        return !chosen || row[filter.key] === chosen;
      });
    });
  };
  const sorted = () => {
    const rows = filtered();
    if (!props.sortable || !sortKey()) return rows;
    return [...rows].sort((left, right) => {
      const result = compare(left[sortKey()] ?? '', right[sortKey()] ?? '');
      return ascending() ? result : -result;
    });
  };
  const pages = () => Math.max(1, Math.ceil(sorted().length / size()));
  const visible = () => sorted().slice((page() - 1) * size(), page() * size());
  const resetPage = () => page.set(1);

  const setFilter = (key: string, value: string) => {
    const next = value ? { ...filterValues(), [key]: value } : { ...filterValues(), [key]: '' };
    filterValues.set(next);
    resetPage();
    props.onFilterChange?.(next);
  };

  const onSort = (key: string) => {
    if (!props.sortable) return;
    if (sortKey() === key) ascending.set(!ascending());
    else {
      sortKey.set(key);
      ascending.set(true);
    }
    resetPage();
  };

  const toolbars = () => (
    <View style={{ gap: 8 }}>
      {props.title || props.description ? (
        <View style={{ gap: 2 }}>
          {props.title ? (
            <Text style={{ fontSize: 13, lineHeight: 17, fontWeight: '600', color: tokens.foreground }}>{props.title}</Text>
          ) : null}
          {props.description ? (
            <Text style={{ fontSize: 10, lineHeight: 14, color: tokens.mutedForeground }}>{props.description}</Text>
          ) : null}
        </View>
      ) : null}
      {props.searchable !== false || props.filters ? (
        <View style={{ flexDirection: 'row', flexWrap: 'wrap', alignItems: 'center', gap: 6 }}>
          {props.searchable !== false ? (
            <View style={{ flexGrow: 1, minWidth: 180, maxWidth: 280 }}>
              <Input
                value={query()}
                size="small"
                prefixIcon="search"
                placeholder={props.searchPlaceholder ?? 'Search…'}
                onChangeText={(next) => {
                  query.set(next);
                  resetPage();
                  props.onSearchChange?.(next);
                }}
                cleanable
              />
            </View>
          ) : null}
          {(props.filters ?? []).map((filter) => (
            <View style={{ flexDirection: 'row', alignItems: 'center', gap: 3, flexWrap: 'wrap' }}>
              <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{filter.label}</Text>
              {[ '', ...filter.options ].map((option) => {
                const chosen = (filterValues()[filter.key] ?? '') === option;
                return (
                  <View
                    accessibilityRole="button"
                    accessibilityLabel={`${filter.label}: ${option || 'All'}`}
                    accessibilityState={{ selected: chosen }}
                    onPress={() => setFilter(filter.key, option)}
                    activeOpacity={0.85}
                    class="transition-colors duration-150 hover:bg-black/[0.05] dark:hover:bg-white/[0.08]"
                    style={{
                      paddingHorizontal: 7,
                      paddingVertical: 3,
                      borderRadius: 5,
                      borderWidth: 1,
                      borderColor: chosen ? alpha(tokens.foreground, 0.3) : tokens.border,
                      backgroundColor: chosen ? alpha(tokens.foreground, isDark() ? 0.12 : 0.07) : 'transparent',
                    }}
                  >
                    <Text style={{ fontSize: 10, color: chosen ? tokens.foreground : tokens.mutedForeground }}>
                      {option || 'All'}
                    </Text>
                  </View>
                );
              })}
            </View>
          ))}
        </View>
      ) : null}
    </View>
  );

  return (
    <View
      testID={props.testID}
      style={[
        {
          gap: 10,
          ...(props.card
            ? {
                padding: 10,
                borderRadius: 12,
                borderWidth: 1,
                borderColor: tokens.border,
                backgroundColor: tokens.popover,
              }
            : {}),
        },
        props.style,
      ]}
    >
      {props.showToolbar !== false ? toolbars() : null}
      <Table
        columns={props.columns.map((column) => ({ ...column, sortable: props.sortable && column.sortable !== false }))}
        rows={visible()}
        bordered
        card={false}
        sortKey={sortKey()}
        sortAscending={ascending()}
        onSort={props.sortable ? onSort : undefined}
        selectedKeys={props.selectedKeys}
        rowKey={props.rowKey}
        onRowPress={props.onRowPress}
        emptyText={props.emptyText}
      />
      <View style={{ flexDirection: 'row', flexWrap: 'wrap', alignItems: 'center', gap: 8 }}>
        <Text style={{ flex: 1, fontSize: 10, color: tokens.mutedForeground }}>
          {sorted().length} of {props.rows.length} row{props.rows.length === 1 ? '' : 's'}
          {query() ? ` · “${query()}”` : ''}
        </Text>
        {props.pageSizeOptions ? (
          <View style={{ flexDirection: 'row', alignItems: 'center', gap: 3 }}>
            {props.pageSizeOptions.map((option) => (
              <View
                accessibilityRole="button"
                accessibilityLabel={`${option} rows`}
                onPress={() => {
                  size.set(option);
                  resetPage();
                }}
                style={{
                  paddingHorizontal: 5,
                  paddingVertical: 2,
                  borderRadius: 4,
                  backgroundColor: size() === option ? tokens.muted : 'transparent',
                }}
              >
                <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{option}</Text>
              </View>
            ))}
          </View>
        ) : null}
        <Pagination page={page()} totalPages={pages()} onPageChange={(next) => page.set(next)} />
      </View>
    </View>
  );
}
