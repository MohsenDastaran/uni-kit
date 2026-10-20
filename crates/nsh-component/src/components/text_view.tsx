// A small read-only rich-text renderer for Markdown-like content.
//
// NSH has no HTML engine in an app; this renders headings, paragraphs, lists,
// quotes and fenced code with Text/View. Inline emphasis is shown as written.
import type { StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { FONT_SIZE_MONO, MONO, RADIUS_MD, SPACING_SM, tokens } from './theme';

export interface TextViewProps {
  content?: string;
  children?: string;
  maxLines?: number;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function TextView(props: TextViewProps) {
  const source = () => props.children ?? props.content ?? '';
  const blocks = () => {
    const lines = source().split('\n');
    const result: { kind: 'h1' | 'h2' | 'p' | 'li' | 'quote' | 'code'; text: string }[] = [];
    let code: string[] | null = null;
    for (const line of lines) {
      if (line.startsWith('```')) {
        if (code === null) code = [];
        else { result.push({ kind: 'code', text: code.join('\n') }); code = null; }
        continue;
      }
      if (code !== null) { code.push(line); continue; }
      if (line.startsWith('### ')) result.push({ kind: 'h2', text: line.slice(4) });
      else if (line.startsWith('## ')) result.push({ kind: 'h2', text: line.slice(3) });
      else if (line.startsWith('# ')) result.push({ kind: 'h1', text: line.slice(2) });
      else if (line.startsWith('- ')) result.push({ kind: 'li', text: line.slice(2) });
      else if (line.startsWith('> ')) result.push({ kind: 'quote', text: line.slice(2) });
      else if (line.trim() !== '') result.push({ kind: 'p', text: line });
    }
    if (code !== null && code.length > 0) result.push({ kind: 'code', text: code.join('\n') });
    return result;
  };
  return (
    <View testID={props.testID} style={[{ gap: SPACING_SM }, props.style]}>
      {blocks().map((block) => {
        if (block.kind === 'h1') return <Text style={{ fontSize: 24, fontWeight: '700', color: tokens.foreground }}>{block.text}</Text>;
        if (block.kind === 'h2') return <Text style={{ fontSize: 18, fontWeight: '600', color: tokens.foreground }}>{block.text}</Text>;
        if (block.kind === 'li') return <Text style={{ fontSize: 14, lineHeight: 22, color: tokens.foreground }}>• {block.text}</Text>;
        if (block.kind === 'quote') return <View style={{ borderLeftWidth: 3, borderLeftColor: tokens.border, paddingLeft: 8 }}><Text style={{ fontSize: 14, color: tokens.mutedForeground }}>{block.text}</Text></View>;
        if (block.kind === 'code') return <View style={{ padding: 8, borderRadius: RADIUS_MD, backgroundColor: tokens.muted }}><Text style={{ fontSize: FONT_SIZE_MONO, lineHeight: 18, fontFamily: MONO, color: tokens.foreground }}>{block.text}</Text></View>;
        return <Text style={{ fontSize: 14, lineHeight: 22, color: tokens.foreground }}>{block.text}</Text>;
      })}
    </View>
  );
}

export function TextViewLink(props: { children?: string; onPress?: () => void; style?: StyleProp<ViewStyle> }) {
  return (
    <Text onPress={props.onPress} style={[{ fontSize: 14, color: tokens.link, textDecorationLine: 'underline' }, props.style]}>
      {props.children}
    </Text>
  );
}
