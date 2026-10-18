// A scroll container with the catalog's padding and a custom scrollbar is a
// native Scroll in NSH; this wrapper names the surface.
import type { JSX, ScrollProps } from 'nsh';
import { Scroll } from 'nsh';
import { tokens } from './theme';

export interface ScrollableProps extends ScrollProps {
  children?: JSX.Element;
}

export function Scrollable(props: ScrollableProps) {
  return <Scroll {...props} style={[{ flex: 1, backgroundColor: tokens.background }, props.style]} />;
}
