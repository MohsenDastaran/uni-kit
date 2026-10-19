// A scroll container on the catalog's surface. NSH owns the scrolling; this
// wrapper names the card/inset treatment, the content inset and whether the
// native indicator is shown.
import type { JSX, ScrollProps } from 'nsh';
import { Scroll, View } from 'nsh';
import { RADIUS_MD, tokens } from './theme';

export type ScrollableVariant = 'plain' | 'card' | 'inset';

export interface ScrollableProps extends ScrollProps {
  children?: JSX.Element;
  /** The surface around the scroll area. */
  variant?: ScrollableVariant;
  /** Space around the content, inside the scroll area. */
  padding?: number;
  /** Show the native scroll indicator (default true). */
  indicator?: boolean;
}

export function Scrollable(props: ScrollableProps) {
  const variant = () => props.variant ?? 'plain';
  const surface = () => {
    switch (variant()) {
      case 'card':
        return {
          borderWidth: 1,
          borderColor: tokens.border,
          borderRadius: RADIUS_MD,
          backgroundColor: tokens.popover,
          overflow: 'hidden' as const,
        };
      case 'inset':
        return {
          borderRadius: RADIUS_MD,
          backgroundColor: tokens.muted,
          padding: 4,
          overflow: 'hidden' as const,
        };
      default:
        return { backgroundColor: tokens.background, overflow: 'hidden' as const };
    }
  };
  return (
    <Scroll
      {...props}
      showsIndicator={props.indicator ?? props.showsIndicator}
      contentContainerStyle={[
        props.padding ? { padding: props.padding } : undefined,
        props.contentContainerStyle,
      ]}
      style={[surface(), props.style]}
    >
      <View style={props.horizontal ? { flexDirection: 'row', alignSelf: 'flex-start', flexShrink: 0 } : undefined}>{props.children}</View>
    </Scroll>
  );
}
