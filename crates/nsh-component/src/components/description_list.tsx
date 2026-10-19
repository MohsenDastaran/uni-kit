// A key/value list. One item per row by default; `columns` packs items into a
// bordered metadata table with a muted label column, hairline separators and
// full-row spans.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import {
  type ControlSize,
  controlFontSize,
  FONT_SIZE_SM,
  RADIUS_MD,
  SPACING_SM,
  tokens,
} from './theme';

export interface DescriptionItem {
  label: string;
  value?: string;
  /** Rich value; takes precedence over `value`. */
  content?: JSX.Element;
  /** Number of value columns this item occupies when `columns > 1`. */
  span?: number;
}

export interface DescriptionListProps {
  items?: DescriptionItem[];
  columns?: number;
  bordered?: boolean;
  separators?: boolean;
  /** Draw the labels on the muted surface, like a metadata table. */
  labelBackground?: boolean;
  size?: ControlSize;
  labelWidth?: number;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

type Row = { items: DescriptionItem[]; span: number };

/** Pack items into rows, honouring full-row spans without leaving holes. */
function rowsOf(items: DescriptionItem[], columns: number): Row[] {
  const rows: Row[] = [];
  let current: DescriptionItem[] = [];
  let used = 0;
  const flush = () => {
    if (current.length) rows.push({ items: current, span: used });
    current = [];
    used = 0;
  };
  for (const item of items) {
    const span = Math.max(1, Math.min(item.span ?? 1, columns));
    if (span >= columns) {
      flush();
      rows.push({ items: [item], span: columns });
      continue;
    }
    if (used + span > columns) flush();
    current.push(item);
    used += span;
    if (used >= columns) flush();
  }
  flush();
  return rows;
}

export function DescriptionList(props: DescriptionListProps) {
  const items = () => props.items ?? [];
  const columns = () => Math.max(1, props.columns ?? 1);
  const size = () => props.size ?? 'medium';
  const labelWidth = () => props.labelWidth ?? (columns() > 1 ? 88 : 110);
  const pad = () => (size() === 'small' ? 6 : size() === 'large' ? 12 : 9);
  const labelBackground = () => props.labelBackground !== false;

  const label = (item: DescriptionItem) => (
    <View
      style={{
        width: labelWidth(),
        minWidth: labelWidth(),
        paddingHorizontal: 10,
        paddingVertical: pad(),
        backgroundColor: labelBackground() ? tokens.muted : 'transparent',
        borderRightWidth: 1,
        borderRightColor: tokens.border,
        justifyContent: 'flex-start',
      }}
    >
      <Text style={{ fontSize: controlFontSize(size()) - 2, lineHeight: 16, color: tokens.mutedForeground }}>
        {item.label}
      </Text>
    </View>
  );
  const value = (item: DescriptionItem) => (
    <View style={{ flex: 1, minWidth: 0, paddingHorizontal: 10, paddingVertical: pad(), justifyContent: 'center' }}>
      {item.content ?? (
        <Text style={{ fontSize: FONT_SIZE_SM, lineHeight: 18, color: tokens.foreground }}>{item.value ?? ''}</Text>
      )}
    </View>
  );

  return (
    <View
      testID={props.testID}
      style={[
        {
          borderWidth: props.bordered ? 1 : 0,
          borderColor: tokens.border,
          borderRadius: props.bordered ? RADIUS_MD : 0,
          overflow: 'hidden',
        },
        props.style,
      ]}
    >
      {columns() === 1
        ? items().map((item, index) => (
            <View
              style={{
                flexDirection: 'row',
                alignItems: 'stretch',
                borderTopWidth: props.separators !== false && index > 0 ? 1 : 0,
                borderTopColor: tokens.border,
              }}
            >
              {label(item)}
              {value(item)}
            </View>
          ))
        : rowsOf(items(), columns()).map((row, index) => (
            <View
              style={{
                flexDirection: 'row',
                alignItems: 'stretch',
                borderTopWidth: props.separators !== false && index > 0 ? 1 : 0,
                borderTopColor: tokens.border,
              }}
            >
              {row.items.map((item, itemIndex) => (
                <View
                  style={{
                    flex: item.span ?? 1,
                    minWidth: 0,
                    flexDirection: 'row',
                    borderLeftWidth: itemIndex > 0 ? 1 : 0,
                    borderLeftColor: tokens.border,
                  }}
                >
                  {label(item)}
                  {value(item)}
                </View>
              ))}
            </View>
          ))}
      {props.children}
    </View>
  );
}

export function DescriptionItemView(props: {
  label: string;
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
}) {
  return (
    <View style={[{ gap: 4 }, props.style]}>
      <Text style={{ fontSize: FONT_SIZE_SM, color: tokens.mutedForeground }}>{props.label}</Text>
      {props.children}
    </View>
  );
}
