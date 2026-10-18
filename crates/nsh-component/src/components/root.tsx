// The application root surface. NSH's `mount` already provides the top-level
// window view; this wrapper gives the catalog a named surface that paints the
// page background, applies the tokens and hosts the content.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { View } from 'nsh';
import { tokens } from './theme';

export interface RootProps {
  children?: JSX.Element;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Root(props: RootProps) {
  return (
    <View
      testID={props.testID}
      style={[
        { flex: 1, backgroundColor: tokens.background },
        props.style,
      ]}
    >
      {props.children}
    </View>
  );
}

export function RootProvider(props: { children?: JSX.Element }) {
  return <Root>{props.children}</Root>;
}
