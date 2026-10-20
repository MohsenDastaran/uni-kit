//! `shimmer` — a clipped highlight sweeping over loading placeholders, from a
//! single line to complete card, list and dashboard skeletons.
import { Text, View } from "nsh";
import { Icon } from "../components/icon";
import { Marker } from "../components/marker";
import { Shimmer, ShimmerBlock, ShimmerText } from "../components/shimmer";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

const bar = (width: any, height = 10) => (
  <View style={{ width, height, borderRadius: 6, backgroundColor: tokens.muted }} />
);
const circle = (size: number) => (
  <View style={{ width: size, height: size, borderRadius: size / 2, backgroundColor: tokens.muted }} />
);
const card = (children: any) => (
  <View style={{ width: 420, maxWidth: "100%", padding: 14, borderWidth: 1, borderColor: tokens.border, borderRadius: 10, backgroundColor: tokens.popover, gap: 10 }}>
    {children}
  </View>
);

export function Demo() {
  // section: Usage
  const usage = (
    <View style={{ width: 420, maxWidth: "100%", gap: 6 }}>
      <ShimmerText>Preparing your workspace…</ShimmerText>
      <ShimmerText duration={1100}>Loading recent activity…</ShimmerText>
      <ShimmerText duration={1700}>Almost there.</ShimmerText>
    </View>
  );
  // section: Basic usage
  const basic = <ShimmerText>Loading the catalog…</ShimmerText>;
  // section: Duration
  const duration = (
    <View style={{ width: 420, maxWidth: "100%", gap: 8 }}>
      {[
        [700, "A quick sweep"],
        [1400, "The default pace"],
        [2200, "A slow, quiet sweep"],
      ].map(([ms, label]) => (
        <View style={{ gap: 2 }}>
          <ShimmerText duration={ms as number}>{label as string}</ShimmerText>
          <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{ms}ms</Text>
        </View>
      ))}
    </View>
  );
  // section: Highlight color
  const highlight = (
    <View style={{ width: 420, maxWidth: "100%", gap: 7 }}>
      <ShimmerText highlightColor={tokens.primary}>Syncing with the primary accent…</ShimmerText>
      <ShimmerText highlightColor={tokens.success}>Search index is ready.</ShimmerText>
      <ShimmerText highlightColor={tokens.warning}>Reviewing pending changes…</ShimmerText>
    </View>
  );
  // section: Spread
  const spread = (
    <View style={{ width: 420, maxWidth: "100%", gap: 7 }}>
      <ShimmerText spread={0.08}>A narrow highlight band</ShimmerText>
      <ShimmerText spread={0.18}>The default highlight band</ShimmerText>
      <ShimmerText spread={0.34}>A wide highlight band</ShimmerText>
    </View>
  );
  // section: Direction and play-once
  const direction = (
    <View style={{ width: 420, maxWidth: "100%", gap: 7 }}>
      <ShimmerText direction="right">Sweeping in from the right</ShimmerText>
      <ShimmerText direction="left">Sweeping in from the left</ShimmerText>
      <ShimmerText playOnce>One sweep, then the text rests</ShimmerText>
    </View>
  );
  // section: Compose with Marker
  const marker = <Marker loading><ShimmerText>Searching the catalog…</ShimmerText></Marker>;
  // section: Compose with Attachment
  const attachment = (
    <View style={{ width: 320, maxWidth: "100%", padding: 10, borderWidth: 1, borderColor: tokens.border, borderRadius: 9, flexDirection: "row", alignItems: "center", gap: 10, backgroundColor: tokens.popover }}>
        <View style={{ width: 34, height: 34, borderRadius: 7, alignItems: "center", justifyContent: "center", backgroundColor: tokens.muted }}>
          <Icon name="file-text" size={15} color={tokens.mutedForeground} />
        </View>
        <View style={{ flex: 1, gap: 3 }}>
          <ShimmerText>report.pdf</ShimmerText>
          <ShimmerText duration={1100} style={{ fontSize: 9 }}>Uploading · 2.4 MB</ShimmerText>
        </View>
      <View style={{ width: 20, height: 20, borderRadius: 10, backgroundColor: tokens.muted }} />
    </View>
  );
  // section: Use with messages and bubbles
  const bubble = (
    <View style={{ width: 360, maxWidth: "100%", gap: 8 }}>
        <View style={{ flexDirection: "row", alignItems: "flex-start", gap: 8 }}>
          {circle(30)}
          <View style={{ flex: 1, gap: 5, padding: 10, borderRadius: 9, backgroundColor: tokens.muted }}>
            <ShimmerText>Loading the conversation…</ShimmerText>
            <ShimmerText duration={1100}>Fetching the latest messages…</ShimmerText>
            <ShimmerText duration={1800}>Preparing attachments…</ShimmerText>
          </View>
        </View>
      <View style={{ alignSelf: "flex-end", width: 210, padding: 10, borderRadius: 9, backgroundColor: tokens.muted, gap: 4 }}>
        <ShimmerText duration={900}>Sending your reply…</ShimmerText>
        <ShimmerText duration={1500} style={{ fontSize: 9 }}>Waiting for the workspace…</ShimmerText>
      </View>
    </View>
  );
  // section: Styling, theme, and reduced motion
  const styling = (
    <Row gap={12} style={{ flexWrap: "wrap" }}>
      <View style={{ width: 220, padding: 14, borderRadius: 10, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover, overflow: "hidden" }}>
        <Shimmer duration={900} style={{ width: 220 }}><View style={{ gap: 8 }}>{bar("100%", 12)}{bar("72%", 10)}</View></Shimmer>
      </View>
      <View style={{ width: 220, padding: 14, borderRadius: 10, backgroundColor: tokens.muted, overflow: "hidden" }}>
        <Shimmer duration={1400} highlightColor={tokens.primary} style={{ width: 220 }}><View style={{ gap: 8 }}>{bar("100%", 12)}{bar("72%", 10)}</View></Shimmer>
      </View>
    </Row>
  );
  // section: Card skeleton
  const cardSkeleton = (
    <Shimmer duration={1400} style={{ width: 420, maxWidth: "100%" }}>
      {card(
        <View style={{ gap: 10 }}>
          {bar("100%", 120)}
          <View style={{ flexDirection: "row", alignItems: "center", gap: 8 }}>
            {circle(28)}
            <View style={{ flex: 1, gap: 4 }}>{bar("55%", 8)}{bar("35%", 7)}</View>
          </View>
          {bar("100%", 8)}
          {bar("84%", 8)}
          <View style={{ flexDirection: "row", gap: 8, paddingTop: 2 }}>
            {bar(86, 26)}
            {bar(64, 26)}
          </View>
        </View>
      )}
    </Shimmer>
  );
  // section: List skeleton
  const listSkeleton = (
    <Shimmer style={{ width: 420, maxWidth: "100%" }}>
      <View style={{ width: 420, maxWidth: "100%", borderWidth: 1, borderColor: tokens.border, borderRadius: 10, overflow: "hidden", backgroundColor: tokens.popover }}>
        {[0, 1, 2, 3].map((index) => (
          <View style={{ flexDirection: "row", alignItems: "center", gap: 9, padding: 11, borderTopWidth: index === 0 ? 0 : 1, borderTopColor: tokens.border }}>
            {circle(28)}
            <View style={{ flex: 1, gap: 4 }}>{bar("45%", 8)}{bar("70%", 7)}</View>
            {bar(42, 18)}
          </View>
        ))}
      </View>
    </Shimmer>
  );
  // section: Table skeleton
  const tableSkeleton = (
    <Shimmer style={{ width: 480, maxWidth: "100%" }}>
      <View style={{ width: 480, maxWidth: "100%", borderWidth: 1, borderColor: tokens.border, borderRadius: 10, overflow: "hidden", backgroundColor: tokens.popover }}>
        <View style={{ flexDirection: "row", gap: 14, padding: 10, backgroundColor: tokens.muted }}>
          {["20%", "40%", "20%", "20%"].map((width) => bar(width, 7))}
        </View>
        {[0, 1, 2, 3, 4].map((index) => (
          <View style={{ flexDirection: "row", gap: 14, padding: 10, borderTopWidth: 1, borderTopColor: tokens.border }}>
            {["20%", "40%", "20%", "20%"].map((width) => bar(width, 8))}
          </View>
        ))}
      </View>
    </Shimmer>
  );
  // section: Profile skeleton
  const profileSkeleton = (
    <Shimmer style={{ width: 420, maxWidth: "100%" }}>
      <View style={{ width: 420, maxWidth: "100%", borderWidth: 1, borderColor: tokens.border, borderRadius: 12, overflow: "hidden", backgroundColor: tokens.popover }}>
        <View style={{ height: 80, backgroundColor: tokens.muted }} />
        <View style={{ padding: 14, gap: 10 }}>
          <View style={{ marginTop: -34 }}>{circle(64)}</View>
          {bar("40%", 11)}
          {bar("24%", 8)}
          <View style={{ flexDirection: "row", gap: 8, paddingTop: 4 }}>
            {bar(72, 26)}
            {bar(90, 26)}
          </View>
          {bar("100%", 8)}
          {bar("86%", 8)}
        </View>
      </View>
    </Shimmer>
  );
  // section: Dashboard skeleton
  const dashboardSkeleton = (
    <Shimmer duration={1600} style={{ width: 520, maxWidth: "100%" }}>
      <View style={{ width: 520, maxWidth: "100%", gap: 12 }}>
        <View style={{ flexDirection: "row", gap: 10 }}>
          {[0, 1, 2].map(() => (
            <View style={{ flex: 1, padding: 12, borderWidth: 1, borderColor: tokens.border, borderRadius: 9, backgroundColor: tokens.popover, gap: 8 }}>
              {bar("55%", 7)}
              {bar("72%", 20)}
              {bar("40%", 6)}
            </View>
          ))}
        </View>
        <View style={{ padding: 14, borderWidth: 1, borderColor: tokens.border, borderRadius: 9, backgroundColor: tokens.popover, gap: 12 }}>
          <View style={{ flexDirection: "row", alignItems: "center" }}>
            {bar("30%", 9)}
            <View style={{ flex: 1 }} />
            {bar(60, 8)}
          </View>
          <View style={{ height: 120, flexDirection: "row", alignItems: "flex-end", gap: 8 }}>
            {[40, 64, 82, 58, 96, 70, 88, 54, 76, 92].map((height) => (
              <View style={{ flex: 1, height, borderRadius: 4, backgroundColor: tokens.muted }} />
            ))}
          </View>
        </View>
      </View>
    </Shimmer>
  );
  // section: end
  return (
    <Page>
      <Caption>Shimmer sweeps a highlight over a placeholder while content loads; prefer Skeleton for a static shape and Spinner for an action.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic usage">{basic}</ExampleSection>
      <ExampleSection index={2} title="Duration">{duration}</ExampleSection>
      <ExampleSection index={3} title="Highlight color">{highlight}</ExampleSection>
      <ExampleSection index={4} title="Spread">{spread}</ExampleSection>
      <ExampleSection index={5} title="Direction and play-once">{direction}</ExampleSection>
      <ExampleSection index={6} title="Compose with Marker">{marker}</ExampleSection>
      <ExampleSection index={7} title="Compose with Attachment">{attachment}</ExampleSection>
      <ExampleSection index={8} title="Use with messages and bubbles">{bubble}</ExampleSection>
      <ExampleSection index={9} title="Styling, theme, and reduced motion">{styling}</ExampleSection>
      <ExampleSection index={10} title="Card skeleton">{cardSkeleton}</ExampleSection>
      <ExampleSection index={11} title="List skeleton">{listSkeleton}</ExampleSection>
      <ExampleSection index={12} title="Table skeleton">{tableSkeleton}</ExampleSection>
      <ExampleSection index={13} title="Profile skeleton">{profileSkeleton}</ExampleSection>
      <ExampleSection index={14} title="Dashboard skeleton">{dashboardSkeleton}</ExampleSection>
    </Page>
  );
}
