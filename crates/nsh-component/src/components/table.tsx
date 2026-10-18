// A basic table: columns, rows, optional stripes and a compact size.
import type { DimensionValue, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { type ControlSize, FONT_SIZE_SM, RADIUS_MD, tokens } from './theme';

export interface TableColumn {
  key: string;
  label: string;
  width?: DimensionValue;
  align?: 'start' | 'center' | 'end';
}

export interface TableProps {
  columns: TableColumn[];
  rows: Record<string, string>[];
  striped?: boolean;
  bordered?: boolean;
  size?: ControlSize;
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
          borderWidth: props.bordered ? 1 : 0,
          borderColor: tokens.border,
          borderRadius: RADIUS_MD,
          overflow: 'hidden',
        },
        props.style,
      ]}
    >
      <View style={{ flexDirection: 'row', gap: 8, paddingHorizontal: 10, paddingVertical: 8, backgroundColor: tokens.muted }}>
        {props.columns.map((column) => (
          <Text style={{ ...cellStyle(column.align), fontWeight: '600', color: tokens.mutedForeground, width: column.width }}>
            {column.label}
          </Text>
        ))}
      </View>
      {props.rows.map((row, index) => (
        <View
          style={{
            flexDirection: 'row',
            gap: 8,
            paddingHorizontal: 10,
            paddingVertical: props.size === 'small' ? 6 : 9,
            backgroundColor: props.striped && index % 2 === 1 ? tokens.muted : tokens.background,
            borderTopWidth: 1,
            borderTopColor: tokens.border,
          }}
        >
          {props.columns.map((column) => (
            <Text style={{ ...cellStyle(column.align), width: column.width }}>{row[column.key] ?? ''}</Text>
          ))}
        </View>
      ))}
    </View>
  );
}
