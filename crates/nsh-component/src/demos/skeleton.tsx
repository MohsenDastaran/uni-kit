//! `skeleton` — loading placeholders in the catalog's shapes.
import { View } from "nsh";
import { Skeleton, SkeletonCircle, SkeletonRect, SkeletonText } from "../components/skeleton";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

// section: Basic Skeleton
const basic = <Skeleton width={240} />;
// section: Text Line Skeleton
const text = <SkeletonText lines={3} />;
// section: Circle Skeleton
const circle = <SkeletonCircle size={48} />;
// section: Rectangle Skeleton
const rect = <SkeletonRect width={240} height={96} />;
// section: Different Shapes
const shapes = <Row gap={16}><SkeletonCircle size={40} /><SkeletonRect width={120} height={40} /><Skeleton width={160} /></Row>;
// section: Secondary Variant
const secondary = <Skeleton secondary width={240} />;
// section: Loading Article List
const article = (
  <View style={{ gap: 12 }}>
    {[0, 1, 2].map((index) => (
      <Row gap={12} style={{ alignItems: "flex-start" }}>
        <SkeletonCircle size={40} />
        <View style={{ flex: 1, gap: 8 }}><Skeleton width="60%" /><Skeleton width="90%" /><Skeleton width="40%" /></View>
      </Row>
    ))}
  </View>
);
// section: end
export function Demo() {
  return (
    <Page>
      <Caption>Skeletons reserve the shape of the content that is coming, so the page does not jump when it arrives.</Caption>
      <ExampleSection index={0} title="Basic Skeleton">{basic}</ExampleSection>
      <ExampleSection index={1} title="Text Line Skeleton">{text}</ExampleSection>
      <ExampleSection index={2} title="Circle Skeleton">{circle}</ExampleSection>
      <ExampleSection index={3} title="Rectangle Skeleton">{rect}</ExampleSection>
      <ExampleSection index={4} title="Different Shapes">{shapes}</ExampleSection>
      <ExampleSection index={5} title="Secondary Variant">{secondary}</ExampleSection>
      <ExampleSection index={6} title="Loading Article List">{article}</ExampleSection>
    </Page>
  );
}
