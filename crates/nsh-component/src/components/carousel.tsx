// A snapping carousel with horizontal or vertical pages, one or more visible
// slides, looping, autoplay, centered slides and overlay or outside controls.
//
// The viewport is a paging Scroll. Slide sizes are measured from the viewport,
// so the same component works with a single hero image, a row of three cards or
// a vertical stack without a breakpoint table.
import type { JSX, LayoutEvent, StyleProp, ViewStyle, YNode } from 'nsh';
import { cleanup, effect, signal, Scroll, Text, View } from 'nsh';
import { reduceMotion } from 'nsh/accessibility';
import { Button } from './button';
import { alpha, isDark, RADIUS_LG, tokens } from './theme';

export type CarouselOrientation = 'horizontal' | 'vertical';
export type CarouselControls = 'overlay' | 'outside';

export interface CarouselProps {
  items: JSX.Element[];
  initialIndex?: number;
  loop?: boolean;
  orientation?: CarouselOrientation;
  /** How many slides share the viewport. Defaults to one. */
  slidesPerView?: number;
  /** Space between slides in points. */
  gap?: number;
  /** Height of a horizontal viewport, or the vertical viewport. */
  height?: number;
  /** A fixed slide width; by default slides divide the viewport. */
  slideWidth?: number;
  /** Center the active slide and let neighbours peek at the edges. */
  centered?: boolean;
  autoplay?: number;
  showControls?: boolean;
  controlPlacement?: CarouselControls;
  showDots?: boolean;
  showCounter?: boolean;
  onIndexChange?: (index: number) => void;
  itemStyle?: StyleProp<ViewStyle>;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Carousel(props: CarouselProps) {
  const width = signal(360);
  const height = signal(props.height ?? 260);
  const index = signal(props.initialIndex ?? 0);
  let scroller: YNode | undefined;
  let positioned = false;

  const orientation = () => props.orientation ?? 'horizontal';
  const vertical = () => orientation() === 'vertical';
  const perView = () => Math.max(1, Math.min(props.items.length || 1, props.slidesPerView ?? 1));
  const gap = () => props.gap ?? 12;
  const slideWidth = () => {
    if (vertical()) return width();
    if (props.slideWidth) return Math.min(width(), props.slideWidth);
    return Math.max(0, (width() - gap() * (perView() - 1)) / perView());
  };
  const slideHeight = () => {
    if (!vertical()) return height();
    return Math.max(0, (height() - gap() * (perView() - 1)) / perView());
  };
  const leadingPad = () => (props.centered && !vertical() ? Math.max(0, (width() - slideWidth()) / 2) : 0);
  const step = () => (vertical() ? slideHeight() + gap() : slideWidth() + gap());

  const go = (next: number) => {
    const count = props.items.length;
    if (count === 0) return;
    const bounded = props.loop ? (next + count) % count : Math.max(0, Math.min(count - 1, next));
    index.set(bounded);
    scroller?.scrollTo(
      vertical()
        ? { y: bounded * step(), animated: true }
        : { x: bounded * step(), animated: true },
    );
    props.onIndexChange?.(bounded);
  };

  effect(() => {
    if (!props.autoplay || reduceMotion()) return;
    const id = setInterval(() => go(index() + 1), Math.max(1200, props.autoplay));
    cleanup(() => clearInterval(id));
  });

  const previousControl = (size: 'small' | 'xsmall' = 'small') => (
    <Button
      icon={vertical() ? 'chevron-up' : 'chevron-left'}
      size={size}
      variant="ghost"
      rounded={16}
      accessibilityLabel="Previous slide"
      onPress={() => go(index() - 1)}
    />
  );

  const nextControl = (size: 'small' | 'xsmall' = 'small') => (
    <Button
      icon={vertical() ? 'chevron-down' : 'chevron-right'}
      size={size}
      variant="ghost"
      rounded={16}
      accessibilityLabel="Next slide"
      onPress={() => go(index() + 1)}
    />
  );

  const overlayControls = () => {
    const style: ViewStyle = {
      position: 'absolute',
      zIndex: 2,
      width: 30,
      height: 30,
      borderRadius: 15,
      backgroundColor: isDark() ? alpha(tokens.background, 0.72) : alpha(tokens.popover, 0.88),
      borderWidth: 1,
      borderColor: tokens.border,
      alignItems: 'center',
      justifyContent: 'center',
    };
    return (
      <>
        <View
          style={[
            { ...style, left: vertical() ? '50%' : 8, top: vertical() ? 8 : '50%' },
            vertical() ? { marginLeft: -15 } : { marginTop: -15 },
          ]}
        >
          {previousControl('xsmall')}
        </View>
        <View
          style={[
            {
              ...style,
              right: vertical() ? undefined : 8,
              left: vertical() ? '50%' : undefined,
              bottom: vertical() ? 8 : undefined,
              top: vertical() ? undefined : '50%',
            },
            vertical() ? { marginLeft: -15 } : { marginTop: -15 },
          ]}
        >
          {nextControl('xsmall')}
        </View>
      </>
    );
  };

  const slides = () =>
    props.items.map((item, at) => (
      <View
        style={[
          {
            width: vertical() ? width() : slideWidth(),
            height: slideHeight(),
            flexShrink: 0,
            overflow: 'hidden',
            borderRadius: RADIUS_LG,
            ...(vertical() ? { marginBottom: gap() } : { marginEnd: gap() }),
          },
          props.itemStyle,
        ]}
      >
        {item}
      </View>
    ));

  const viewport = (
    <Scroll
      ref={(node) => (scroller = node)}
      horizontal={!vertical()}
      pagingEnabled={!props.centered}
      snapToInterval={step()}
      showsIndicator={false}
      onLayout={(event: LayoutEvent) => {
        width.set(event.width);
        height.set(event.height);
        if (!positioned && scroller) {
          positioned = true;
          const sw = vertical()
            ? event.width
            : props.slideWidth
              ? Math.min(event.width, props.slideWidth)
              : Math.max(0, (event.width - gap() * (perView() - 1)) / perView());
          const sh = vertical()
            ? Math.max(0, (event.height - gap() * (perView() - 1)) / perView())
            : event.height;
          const pad = props.centered && !vertical() ? Math.max(0, (event.width - sw) / 2) : 0;
          scroller.scrollTo(
            vertical()
              ? { y: (props.initialIndex ?? 0) * (sh + gap()), animated: false }
              : { x: (props.initialIndex ?? 0) * (sw + gap()), animated: false },
          );
        }
      }}
      style={{ width: '100%', height: props.height ?? 260, overflow: 'hidden' }}
    >
      {props.centered && !vertical() ? <View style={{ width: leadingPad() }} /> : null}
      {slides()}
      {props.centered && !vertical() ? <View style={{ width: leadingPad() }} /> : null}
    </Scroll>
  );

  const outside = () => props.controlPlacement === 'outside' && props.showControls !== false;

  return (
    <View testID={props.testID} style={[{ gap: 8 }, props.style]}>
      {outside() && vertical() ? (
        <View style={{ alignItems: 'center' }}>{previousControl()}</View>
      ) : null}
      <View
        style={{
          position: 'relative',
          width: '100%',
          ...(outside() && !vertical()
            ? { flexDirection: 'row', alignItems: 'center', gap: 2 }
            : {}),
        }}
      >
        {outside() && !vertical() ? <View>{previousControl()}</View> : null}
        {viewport}
        {outside() && !vertical() ? <View>{nextControl()}</View> : null}
        {!outside() && props.showControls !== false ? overlayControls() : null}
      </View>
      {outside() && vertical() ? (
        <View style={{ alignItems: 'center' }}>{nextControl()}</View>
      ) : null}
      <View style={{ flexDirection: 'row', alignItems: 'center', justifyContent: 'center', gap: 6 }}>
        {props.showDots === false
          ? null
          : props.items.map((_, at) => (
              <View
                accessibilityRole="button"
                accessibilityLabel={`Go to slide ${at + 1}`}
                onPress={() => go(at)}
                style={{
                  width: at === index() ? 18 : 7,
                  height: 7,
                  borderRadius: 4,
                  backgroundColor: at === index() ? tokens.primary : tokens.border,
                  ...(reduceMotion() ? {} : { transitionProperty: 'width, background-color', transitionDuration: 160 }),
                }}
              />
            ))}
        {props.showCounter ? (
          <Text style={{ marginStart: 6, fontSize: 11, color: tokens.mutedForeground }}>
            {index() + 1} / {props.items.length}
          </Text>
        ) : null}
      </View>
    </View>
  );
}
