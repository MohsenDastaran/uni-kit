// A snapping horizontal carousel with previous/next controls.
import type { JSX, LayoutEvent, StyleProp, ViewStyle, YNode } from 'nsh';
import { signal, View, Scroll } from 'nsh';
import { Button } from './button';
import { tokens } from './theme';

export interface CarouselProps {
  items: JSX.Element[];
  initialIndex?: number;
  loop?: boolean;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Carousel(props: CarouselProps) {
  const width = signal(320);
  const index = signal(props.initialIndex ?? 0);
  let scroller: YNode | undefined;
  const go = (next: number) => {
    const count = props.items.length;
    const bounded = props.loop ? (next + count) % count : Math.max(0, Math.min(count - 1, next));
    index.set(bounded);
    scroller?.scrollTo({ x: bounded * width(), animated: true });
  };
  return (
    <View testID={props.testID} style={[{ gap: 8 }, props.style]}>
      <View style={{ flexDirection: 'row', alignItems: 'center', gap: 8 }}>
        <Button icon="chevron-left" size="small" variant="ghost" accessibilityLabel="Previous" onPress={() => go(index() - 1)} />
        <Scroll
          ref={(node) => (scroller = node)}
          horizontal
          pagingEnabled
          snapToInterval={width()}
          showsIndicator={false}
          onLayout={(event: LayoutEvent) => width.set(event.width)}
          style={{ flex: 1, height: 180, borderRadius: 8, overflow: 'hidden' }}
        >
          {props.items.map((item) => (
            <View style={{ width: width(), height: 180 }}>{item}</View>
          ))}
        </Scroll>
        <Button icon="chevron-right" size="small" variant="ghost" accessibilityLabel="Next" onPress={() => go(index() + 1)} />
      </View>
      <View style={{ flexDirection: 'row', justifyContent: 'center', gap: 6 }}>
        {props.items.map((_, at) => (
          <View
            accessibilityRole="button"
            accessibilityLabel={`Go to slide ${at + 1}`}
            onPress={() => go(at)}
            style={{ width: at === index() ? 18 : 7, height: 7, borderRadius: 4, backgroundColor: at === index() ? tokens.primary : tokens.border }}
          />
        ))}
      </View>
    </View>
  );
}
