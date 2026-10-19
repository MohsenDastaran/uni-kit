//! `carousel` — a snapping viewport with one or more visible slides.
import { Text, View } from "nsh";
import { Image } from "nsh/image";
import cover from "../../examples/assets/cover.png";
import { Carousel } from "../components/carousel";
import { Icon } from "../components/icon";
import { alpha, inkOn, tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

const gradient = (from: string, to: string) => ({
  backgroundColor: from,
  backgroundImage: `linear-gradient(135deg, ${alpha(from, 0.92)}, ${alpha(to, 0.72)})`,
});

const gradientSlide = (label: string, detail: string, from: string, to: string) => (
  <View
    style={{
      flex: 1,
      justifyContent: "flex-end",
      padding: 16,
      gap: 2,
      ...gradient(from, to),
    }}
  >
    <Text style={{ fontSize: 22, lineHeight: 28, fontWeight: "700", color: inkOn(from) }}>
      {label}
    </Text>
    <Text style={{ fontSize: 12, lineHeight: 17, color: alpha(inkOn(from), 0.8) }}>
      {detail}
    </Text>
  </View>
);

const coverSlide = (title: string, detail: string) => (
  <View style={{ flex: 1, justifyContent: "flex-end", padding: 18 }}>
    <Image src={cover} resizeMode="cover" style={{ position: "absolute", top: 0, left: 0, width: "100%", height: "100%" }} />
    <View
      style={{
        position: "absolute",
        left: 0,
        right: 0,
        bottom: 0,
        height: 140,
        backgroundImage: `linear-gradient(to top, ${alpha(tokens.background, 0.9)}, ${alpha(tokens.background, 0)})`,
      }}
    />
    <Text style={{ fontSize: 22, lineHeight: 28, fontWeight: "700", color: tokens.foreground }}>
      {title}
    </Text>
    <Text style={{ fontSize: 12, lineHeight: 17, color: tokens.mutedForeground }}>{detail}</Text>
  </View>
);

export function Demo() {
  // section: Usage
  const usage = (
    <Carousel
      items={[coverSlide("Alpine ridge", "A single hero slide with overlay text."), coverSlide("Still water", "Snap, loop and page indicators.")]}
      height={300}
      showDots={false}
      showCounter
      style={{ maxWidth: 620 }}
    />
  );

  // section: Basic carousel with single item
  const single = (
    <Carousel
      items={[coverSlide("Alpine ridge", "A single slide, measured to the viewport."), coverSlide("Still water", "The next page waits off-screen.")]}
      height={320}
      showDots={false}
      style={{ maxWidth: 620 }}
    />
  );

  // section: Vertical carousel orientation
  const vertical = (
    <Carousel
      orientation="vertical"
      items={[
        coverSlide("Fog on the lake", "Vertical paging keeps the same snap."),
        gradientSlide("Under the pier", "The second page.", tokens.info, tokens.success),
      ]}
      height={360}
      showDots={false}
      controlPlacement="outside"
      style={{ maxWidth: 360 }}
    />
  );

  // section: Carousel with multiple items visible
  const multiple = (
    <Carousel
      items={[
        gradientSlide("Field notes", "A quiet start.", tokens.info, tokens.success),
        gradientSlide("Signal", "The middle page.", tokens.success, tokens.warning),
        gradientSlide("Afterglow", "Late light.", tokens.warning, tokens.danger),
        gradientSlide("Nightfall", "The last page.", tokens.danger, tokens.info),
      ]}
      slidesPerView={3}
      gap={12}
      height={250}
      showDots={false}
      style={{ maxWidth: 720 }}
    />
  );

  // section: Carousel with responsive basis
  const responsive = (
    <Carousel
      items={[
        gradientSlide("Coast", "One of three.", tokens.info, tokens.primary),
        gradientSlide("Ridge", "Two visible.", tokens.primary, tokens.success),
        gradientSlide("Ferris", "The layout reflows from the measured width.", tokens.warning, tokens.info),
      ]}
      slidesPerView={2}
      gap={12}
      height={280}
      showDots={false}
      showCounter
      style={{ maxWidth: 640 }}
    />
  );

  // section: Carousel with autoplay
  const autoplay = (
    <Carousel
      items={[
        gradientSlide("01 · Morning", "Autoplay, looping.", tokens.info, tokens.success),
        gradientSlide("02 · Noon", "Pause on the controls.", tokens.success, tokens.warning),
        gradientSlide("03 · Dusk", "Back to the first page.", tokens.warning, tokens.danger),
      ]}
      autoplay={2400}
      loop
      height={260}
      showDots={false}
      showCounter
      style={{ maxWidth: 620 }}
    />
  );

  // section: Carousel with centered slide alignment
  const centered = (
    <Carousel
      items={[
        gradientSlide("Before", "Peek", tokens.info, tokens.success),
        gradientSlide("Alpine", "Centered", tokens.primary, tokens.info),
        gradientSlide("After", "Peek", tokens.success, tokens.warning),
      ]}
      slidesPerView={3}
      slideWidth={250}
      initialIndex={1}
      centered
      height={260}
      gap={12}
      showDots={false}
      style={{ maxWidth: 620 }}
    />
  );

  // section: Carousel with custom spacing
  const spacing = (
    <Carousel
      items={[
        gradientSlide("Boat", "24pt gaps.", tokens.info, tokens.success),
        gradientSlide("Shore", "No lost rhythm.", tokens.success, tokens.warning),
        gradientSlide("Hills", "The snap follows the gap.", tokens.warning, tokens.info),
      ]}
      slidesPerView={3}
      gap={24}
      height={260}
      showDots={false}
      style={{ maxWidth: 620 }}
    />
  );

  // section: Carousel with images and overlays
  const overlays = (
    <Carousel
      items={[
        coverSlide("Slide 1", "Beautiful landscape description for slide 1."),
        coverSlide("Slide 2", "A second landscape, with the same quiet overlay."),
        coverSlide("Slide 3", "A third page for the final frame."),
      ]}
      height={300}
      showDots={false}
      showCounter
      style={{ maxWidth: 620 }}
    />
  );

  // section: Looping
  const loop = (
    <Carousel
      items={[
        gradientSlide("A", "Looping carousel.", tokens.primary, tokens.muted),
        gradientSlide("B", "Previous from the first reaches the last.", tokens.secondary, tokens.info),
        gradientSlide("C", "Next from the last returns to the first.", tokens.info, tokens.success),
      ]}
      loop
      height={220}
      showDots={false}
      style={{ maxWidth: 620 }}
    />
  );
  // section: end

  return (
    <Page>
      <Caption>
        The carousel is a paging Scroll with measured slides, overlay controls and optional
        autoplay. The slides are application content: images, cards or any other view.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic carousel with single item">{single}</ExampleSection>
      <ExampleSection index={2} title="Vertical carousel orientation">{vertical}</ExampleSection>
      <ExampleSection index={3} title="Carousel with multiple items visible">{multiple}</ExampleSection>
      <ExampleSection index={4} title="Carousel with responsive basis">{responsive}</ExampleSection>
      <ExampleSection index={5} title="Carousel with autoplay">{autoplay}</ExampleSection>
      <ExampleSection index={6} title="Carousel with centered slide alignment">{centered}</ExampleSection>
      <ExampleSection index={7} title="Carousel with custom spacing">{spacing}</ExampleSection>
      <ExampleSection index={8} title="Carousel with images and overlays">{overlays}</ExampleSection>
      <ExampleSection index={9} title="Looping">{loop}</ExampleSection>
    </Page>
  );
}
