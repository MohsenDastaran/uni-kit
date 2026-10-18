// Page navigation with previous, next and numbered pages.
import type { StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Button } from './button';
import { tokens } from './theme';

export interface PaginationProps {
  page?: number;
  totalPages: number;
  onPageChange?: (page: number) => void;
  siblingCount?: number;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Pagination(props: PaginationProps) {
  const page = () => Math.max(1, Math.min(props.totalPages, props.page ?? 1));
  const pages = () => {
    const total = Math.max(1, props.totalPages);
    const window = 2 + (props.siblingCount ?? 1);
    const from = Math.max(1, page() - window);
    const to = Math.min(total, page() + window);
    return Array.from({ length: to - from + 1 }, (_, index) => from + index);
  };
  return (
    <View testID={props.testID} accessibilityRole="tablist" style={[{ flexDirection: 'row', alignItems: 'center', gap: 4 }, props.style]}>
      <Button icon="chevron-left" size="small" variant="ghost" accessibilityLabel="Previous page" disabled={page() <= 1} onPress={() => props.onPageChange?.(page() - 1)} />
      {pages().map((entry) => (
        <Button
          label={String(entry)}
          size="small"
          variant={entry === page() ? 'primary' : 'ghost'}
          onPress={() => props.onPageChange?.(entry)}
        />
      ))}
      <Button icon="chevron-right" size="small" variant="ghost" accessibilityLabel="Next page" disabled={page() >= props.totalPages} onPress={() => props.onPageChange?.(page() + 1)} />
      <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>Page {page()} of {props.totalPages}</Text>
    </View>
  );
}
