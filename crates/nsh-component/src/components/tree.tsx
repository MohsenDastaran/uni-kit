// A hierarchical tree with expandable nodes.
import type { StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Icon } from './icon';
import { type IconName } from './icons';
import { tokens } from './theme';

export interface TreeNode {
  id: string;
  label: string;
  icon?: IconName;
  children?: TreeNode[];
}

export interface TreeProps {
  nodes: TreeNode[];
  defaultExpanded?: string[];
  onSelect?: (id: string) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

function NodeView(props: { node: TreeNode; depth: number; expanded: () => string[]; toggle: (id: string) => void; select?: (id: string) => void }) {
  const isOpen = () => props.expanded().includes(props.node.id);
  return (
    <View>
      <View
        accessibilityLabel={props.node.label}
        accessibilityState={{ expanded: props.node.children ? isOpen() : undefined }}
        onPress={() => {
          if (props.node.children) props.toggle(props.node.id);
          props.select?.(props.node.id);
        }}
        activeOpacity={0.8}
        style={{ flexDirection: 'row', alignItems: 'center', gap: 6, paddingVertical: 5, paddingLeft: 6 + props.depth * 16 }}
      >
        {props.node.children ? (
          <Icon name={isOpen() ? 'chevron-down' : 'chevron-right'} size={12} color={tokens.mutedForeground} />
        ) : (
          <View style={{ width: 12 }} />
        )}
        {props.node.icon ? <Icon name={props.node.icon} size={14} color={tokens.mutedForeground} /> : null}
        <Text style={{ fontSize: 13, color: tokens.foreground }}>{props.node.label}</Text>
      </View>
      {props.node.children && isOpen()
        ? props.node.children.map((child) => (
            <NodeView node={child} depth={props.depth + 1} expanded={props.expanded} toggle={props.toggle} select={props.select} />
          ))
        : null}
    </View>
  );
}

export function Tree(props: TreeProps) {
  const expanded = signal<string[]>(props.defaultExpanded ?? []);
  const toggle = (id: string) => {
    expanded.update((list) => (list.includes(id) ? list.filter((entry) => entry !== id) : [...list, id]));
  };
  return (
    <View testID={props.testID} style={[{ gap: 1 }, props.style]}>
      {props.nodes.map((node) => (
        <NodeView node={node} depth={0} expanded={expanded} toggle={toggle} select={props.onSelect} />
      ))}
    </View>
  );
}
