// A framed picture with the catalog's fit, radius and placeholder.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { Text, View } from 'nsh';
import { Image } from 'nsh/image';
import { Icon } from './icon';
import { RADIUS_MD, tokens } from './theme';

export type ImageFit = 'cover' | 'contain' | 'stretch' | 'center';

export interface ImageViewProps {
  src?: string | { uri: string };
  fit?: ImageFit;
  radius?: number;
  alt?: string;
  placeholder?: JSX.Element;
  onLoad?: () => void;
  onError?: () => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function ImageView(props: ImageViewProps) {
  return (
    <View
      testID={props.testID}
      accessibilityRole="image"
      accessibilityLabel={props.alt}
      style={[
        {
          minWidth: 0,
          minHeight: 80,
          borderRadius: props.radius ?? RADIUS_MD,
          backgroundColor: tokens.muted,
          overflow: 'hidden',
          borderWidth: 1,
          borderColor: tokens.border,
        },
        props.style,
      ]}
    >
      {props.src ? (
        <Image
          src={props.src}
          resizeMode={props.fit ?? 'cover'}
          onLoad={props.onLoad}
          onError={props.onError}
          style={{ width: '100%', height: '100%' }}
        />
      ) : (
        props.placeholder ?? (
          <View style={{ flex: 1, alignItems: 'center', justifyContent: 'center', gap: 6 }}>
            <Icon name="image" size={28} color={tokens.mutedForeground} />
            <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>No image</Text>
          </View>
        )
      )}
    </View>
  );
}
