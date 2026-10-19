// Page navigation: labels, numbers, ellipsis, first/last, page info, a rows-per-
// page select, a page selector and a go-to-page field.
import type { StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Button } from './button';
import { Input } from './input';
import { Select } from './select';
import { type ControlSize, RADIUS_MD, tokens } from './theme';

export type PaginationShape = 'square' | 'circle';
export type PaginationInfoAt = 'start' | 'center' | 'end';

export interface PaginationProps {
  page?: number;
  totalPages: number;
  onPageChange?: (page: number) => void;
  siblingCount?: number;
  boundaryCount?: number;
  size?: ControlSize;
  /** Text Previous / Next instead of icon-only buttons. */
  labels?: boolean;
  showNumbers?: boolean;
  showPageInfo?: boolean;
  pageInfoAt?: PaginationInfoAt;
  /** Overall alignment; `between` spreads the row ends. */
  align?: 'start' | 'center' | 'end' | 'between';
  showFirstLast?: boolean;
  shape?: PaginationShape;
  /** The resting style of inactive page buttons. */
  buttonVariant?: 'ghost' | 'secondary';
  /** Draw page buttons with the outline treatment. */
  outline?: boolean;
  card?: boolean;
  disabled?: boolean;
  /** Rows-per-page selector; pair with `totalItems` for a range label. */
  rowsPerPage?: number;
  rowsPerPageOptions?: number[];
  onRowsPerPageChange?: (rows: number) => void;
  totalItems?: number;
  /** A compact page selector instead of the numbered row. */
  pageSelector?: boolean;
  /** A page input after the controls. */
  goToPage?: boolean;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Pagination(props: PaginationProps) {
  const page = () => Math.max(1, Math.min(Math.max(1, props.totalPages), props.page ?? 1));
  const go = signal(String(page()));
  const disabled = () => Boolean(props.disabled);
  const shape = () => props.shape ?? 'square';
  const rounded = () => (shape() === 'circle' ? 999 : 7);
  const change = (next: number) => {
    if (disabled()) return;
    const to = Math.max(1, Math.min(props.totalPages, next));
    if (to !== page()) props.onPageChange?.(to);
  };

  /** The pages plus `-1` ellipsis markers. */
  const tokensOfPages = () => {
    const total = Math.max(1, props.totalPages);
    const boundary = Math.max(1, props.boundaryCount ?? 1);
    const sibling = Math.max(0, props.siblingCount ?? 1);
    const wanted = new Set<number>();
    for (let index = 1; index <= Math.min(boundary, total); index += 1) wanted.add(index);
    for (let index = Math.max(1, total - boundary + 1); index <= total; index += 1) wanted.add(index);
    for (let index = Math.max(1, page() - sibling); index <= Math.min(total, page() + sibling); index += 1) wanted.add(index);
    const sorted = [...wanted].sort((a, b) => a - b);
    const out: number[] = [];
    let previous = 0;
    for (const entry of sorted) {
      if (previous && entry - previous > 1) out.push(-1);
      out.push(entry);
      previous = entry;
    }
    return out;
  };

  const button = (entry: number) => (
    <Button
      label={String(entry)}
      size={props.size ?? 'small'}
      variant={entry === page() ? 'secondary' : (props.buttonVariant ?? 'ghost')}
      outline={props.outline || entry === page()}
      rounded={rounded()}
      disabled={disabled()}
      onPress={() => change(entry)}
    />
  );

  const info = (
    <Text style={{ fontSize: 11, lineHeight: 15, color: tokens.mutedForeground }}>
      Page {page()} of {props.totalPages}
    </Text>
  );

  const range = () => {
    if (props.totalItems === undefined || !props.rowsPerPage) return null;
    const start = (page() - 1) * props.rowsPerPage + 1;
    const end = Math.min(props.totalItems, page() * props.rowsPerPage);
    return (
      <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>
        {start}–{end} of {props.totalItems}
      </Text>
    );
  };

  const rowsSelect = () =>
    props.rowsPerPage !== undefined ? (
      <View style={{ flexDirection: 'row', alignItems: 'center', gap: 5 }}>
        <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Rows per page</Text>
        <Select
          value={String(props.rowsPerPage)}
          size="small"
          options={(props.rowsPerPageOptions ?? [10, 25, 50, 100]).map((rows) => ({ value: String(rows), label: String(rows) }))}
          onChange={(next) => props.onRowsPerPageChange?.(Number(next))}
          style={{ width: 66 }}
        />
      </View>
    ) : null;

  const pageSelector = () =>
    props.pageSelector ? (
      <Select
        value={String(page())}
        size={props.size ?? 'small'}
        options={Array.from({ length: Math.max(1, props.totalPages) }, (_, index) => ({ value: String(index + 1), label: `Page ${index + 1}` }))}
        onChange={(next) => change(Number(next))}
        style={{ width: 112 }}
      />
    ) : null;

  const numbers = () =>
    props.showNumbers === false ? null : (
      <View style={{ flexDirection: 'row', alignItems: 'center', gap: 3 }}>
        {tokensOfPages().map((entry) =>
          entry === -1 ? (
            <Text style={{ width: 18, textAlign: 'center', fontSize: 11, color: tokens.mutedForeground }}>…</Text>
          ) : (
            button(entry)
          ),
        )}
      </View>
    );

  const navigation = (
    <View style={{ flexDirection: 'row', alignItems: 'center', gap: 3 }}>
      {props.showFirstLast ? (
        <Button icon="arrow-left" size={props.size ?? 'small'} variant={props.buttonVariant ?? 'ghost'} outline={props.outline} rounded={rounded()} disabled={disabled() || page() <= 1} accessibilityLabel="First page" onPress={() => change(1)} />
      ) : null}
      <Button
        {...(props.labels ? { label: 'Previous', icon: 'chevron-left' } : { icon: 'chevron-left' })}
        size={props.size ?? 'small'}
        variant={props.buttonVariant ?? 'ghost'}
        rounded={rounded()}
        disabled={disabled() || page() <= 1}
        accessibilityLabel="Previous page"
        onPress={() => change(page() - 1)}
      />
      {pageSelector() ?? numbers()}
      {props.pageInfoAt === 'center' && props.showPageInfo ? info : null}
      <Button
        {...(props.labels ? { label: 'Next', icon: 'chevron-right' } : { icon: 'chevron-right' })}
        size={props.size ?? 'small'}
        variant={props.buttonVariant ?? 'ghost'}
        rounded={rounded()}
        disabled={disabled() || page() >= props.totalPages}
        accessibilityLabel="Next page"
        onPress={() => change(page() + 1)}
      />
      {props.showFirstLast ? (
        <Button icon="arrow-right" size={props.size ?? 'small'} variant={props.buttonVariant ?? 'ghost'} outline={props.outline} rounded={rounded()} disabled={disabled() || page() >= props.totalPages} accessibilityLabel="Last page" onPress={() => change(props.totalPages)} />
      ) : null}
    </View>
  );

  return (
    <View
      testID={props.testID}
      accessibilityRole="tablist"
      style={[
        {
          width: '100%',
          flexDirection: 'row',
          alignItems: 'center',
          gap: 10,
          flexWrap: 'wrap',
          justifyContent:
            props.align === 'between'
              ? 'space-between'
              : props.align === 'start'
                ? 'flex-start'
                : props.align === 'end'
                  ? 'flex-end'
                  : props.align === 'center'
                    ? 'center'
                    : props.pageInfoAt === 'start'
                      ? 'flex-start'
                      : props.pageInfoAt === 'end'
                        ? 'flex-end'
                        : 'center',
          ...(props.card
            ? {
                padding: 8,
                borderRadius: RADIUS_MD,
                borderWidth: 1,
                borderColor: tokens.border,
                backgroundColor: tokens.muted,
              }
            : {}),
          opacity: disabled() ? 0.5 : 1,
        },
        props.style,
      ]}
    >
      {rowsSelect()}
      {range()}
      {props.pageInfoAt === 'start' && props.showPageInfo ? info : null}
      {navigation}
      {props.pageInfoAt !== 'start' && props.pageInfoAt !== 'center' && props.showPageInfo ? info : null}
      {props.goToPage ? (
        <View style={{ flexDirection: 'row', alignItems: 'center', gap: 5 }}>
          <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Go to page</Text>
          <Input
            value={go()}
            size="small"
            keyboardType="number-pad"
            onChangeText={(next) => go.set(next)}
            onSubmit={() => change(Number(go()) || page())}
            style={{ width: 56 }}
          />
        </View>
      ) : null}
    </View>
  );
}
