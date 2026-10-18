//! `shimmer` — a loading sweep over text or a block.
import { Text } from "nsh";
import { Shimmer, ShimmerBlock, ShimmerText } from "../components/shimmer";
import { Marker } from "../components/marker";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

// section: Basic usage
const basic = <ShimmerText>Loading the catalog…</ShimmerText>;
// section: Duration
const duration = <ShimmerText duration={700}>A faster sweep</ShimmerText>;
// section: Direction and play-once
const direction = <Shimmer direction="right" playOnce><ShimmerBlock width={220} height={16} /></Shimmer>;
export function Demo() {
  // section: Compose with Marker
  const composed = <Marker loading><Text style={{ fontSize: 13, color: tokens.mutedForeground }}>Searching…</Text></Marker>;
  // section: end
  return (
    <Page>
      <Caption>Shimmer is a pulse over a placeholder while content loads; prefer Skeleton for a shape and Spinner for an action.</Caption>
      <ExampleSection index={0} title="Basic usage">{basic}</ExampleSection>
      <ExampleSection index={1} title="Duration">{duration}</ExampleSection>
      <ExampleSection index={2} title="Direction and play-once">{direction}</ExampleSection>
      <ExampleSection index={3} title="Compose with Marker">{composed}</ExampleSection>
    </Page>
  );
}
