//! `scrollable` — the native scroll area in the catalog's surfaces, with the
//! axes, indicator modes, viewport reporting and large-list honesty of the iced
//! and Slint ports.
import { signal, Text, View } from "nsh";
import { Icon } from "../components/icon";
import { Scrollable } from "../components/scrollable";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row, Status } from "./ui";

const ROW_HEIGHT = 28;
const ROW_GAP = 4;

const rows = (count: number, prefix = "Row") =>
  Array.from({ length: count }, (_, index) => (
    <View style={{ height: ROW_HEIGHT, marginBottom: ROW_GAP, paddingHorizontal: 8, borderRadius: 6, backgroundColor: tokens.muted, justifyContent: "center" }}>
      <Text style={{ fontSize: 11, color: tokens.foreground }}>{prefix} {index + 1}</Text>
    </View>
  ));

const cards = (count: number) => (
  <View style={{ flexDirection: "row", gap: 8 }}>
    {Array.from({ length: count }, (_, index) => (
      <View style={{ width: 120, height: 88, borderRadius: 8, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.muted, alignItems: "center", justifyContent: "center" }}>
        <Text style={{ fontSize: 11, color: tokens.foreground }}>Card {index + 1}</Text>
      </View>
    ))}
  </View>
);

const files = [
  ["document.pdf", "2.4 MB"],
  ["presentation.pptx", "8.1 MB"],
  ["spreadsheet.xlsx", "1.2 MB"],
  ["image.jpg", "4.7 MB"],
  ["archive.zip", "12.6 MB"],
  ["notes.md", "18 KB"],
  ["diagram.svg", "64 KB"],
  ["report.csv", "920 KB"],
];

const messages = [
  ["Alex", "The release is queued for 14:00."],
  ["You", "I will watch the error rate and p95."],
  ["Alex", "The migration takes about eight minutes."],
  ["You", "Rollback window is open until 14:30."],
  ["Alex", "Search rebuild is finished."],
  ["You", "Promoting both services together."],
  ["Alex", "The tag is cut and the URL is posted."],
];

const bothPane = (width: number, height: number, contentWidth: number, rowCount: number) => (
  <Scrollable horizontal variant="card" style={{ width, height }}>
    <View style={{ width: contentWidth }}>
      <Scrollable padding={6} style={{ width: contentWidth, height }}>
        <View style={{ gap: 8 }}>
          {Array.from({ length: rowCount }, (_, row) => (
            <View style={{ flexDirection: "row", gap: 8 }}>
              {Array.from({ length: 8 }, (_, column) => (
                <View style={{ width: 120, height: 64, borderRadius: 8, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.muted, alignItems: "center", justifyContent: "center" }}>
                  <Text style={{ fontSize: 11, color: tokens.foreground }}>{row + 1},{column + 1}</Text>
                </View>
              ))}
            </View>
          ))}
        </View>
      </Scrollable>
    </View>
  </Scrollable>
);

export function Demo() {
  const virtualY = signal(0);
  const first = () => Math.floor(virtualY() / (ROW_HEIGHT + ROW_GAP)) + 1;
  const visible = Math.ceil(160 / (ROW_HEIGHT + ROW_GAP));
  const last = () => Math.min(300, first() + visible - 1);

  const sample = (label: string, area: any, width = 220) => (
    <View style={{ gap: 6 }}>
      <Text style={{ fontSize: 10, fontWeight: "500", color: tokens.mutedForeground }}>{label}</Text>
      {area}
    </View>
  );

  const vertical = (
    <Scrollable variant="card" padding={6} style={{ width: 220, height: 180 }}>
      {rows(20)}
    </Scrollable>
  );
  const horizontal = (
    <Scrollable horizontal variant="card" padding={6} style={{ width: 220, height: 112 }}>
      {cards(10)}
    </Scrollable>
  );
  const both = bothPane(260, 180, 720, 4);

  // section: Usage
  const usage = (
    <View style={{ gap: 10 }}>
      <Row gap={16} style={{ flexWrap: "wrap", alignItems: "flex-start" }}>
        {sample("Vertical", vertical)}
        {sample("Horizontal", horizontal)}
        {sample("Both", both)}
      </Row>
      <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>One horizontal bar moves every row; the inner area scrolls vertically.</Text>
    </View>
  );

  // section: Basic Scrollable Container
  const basic = (
    <Scrollable variant="card" padding={6} style={{ width: 440, height: 220 }}>
      {rows(30)}
    </Scrollable>
  );

  // section: Vertical Scrolling
  const verticalStory = (
    <View style={{ width: 440, maxWidth: "100%" }}>
      <Scrollable variant="card" padding={8} style={{ height: 240 }}>
        {rows(40)}
      </Scrollable>
    </View>
  );

  // section: Horizontal Scrolling
  const horizontalStory = (
    <View style={{ width: 440, maxWidth: "100%" }}>
      <Scrollable horizontal variant="card" padding={8} style={{ height: 120 }}>
        {cards(12)}
      </Scrollable>
    </View>
  );

  // section: Both Directions
  const bothStory = (
    <View style={{ width: 460, maxWidth: "100%" }}>
      {bothPane(460, 220, 1080, 6)}
    </View>
  );

  // section: File Browser with Scrolling
  const browser = (
    <View style={{ width: 360, maxWidth: "100%" }}>
      <Scrollable variant="card" style={{ height: 240 }}>
        <View style={{ padding: 8, gap: 2 }}>
          {files.map(([name, size]) => (
            <Row gap={8} style={{ alignItems: "center", height: 32, paddingHorizontal: 8, borderRadius: 6, backgroundColor: tokens.muted }}>
              <Icon name="file-text" size={13} color={tokens.mutedForeground} />
              <Text style={{ flex: 1, fontSize: 11, color: tokens.foreground }}>{name}</Text>
              <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{size}</Text>
            </Row>
          ))}
        </View>
      </Scrollable>
    </View>
  );

  // section: Chat Messages with Auto-scroll
  const chat = (
    <View style={{ width: 380, maxWidth: "100%" }}>
      <Scrollable variant="card" padding={10} style={{ height: 240 }} contentOffset={{ y: 9999 }}>
        <View style={{ gap: 10 }}>
          {messages.map(([name, body]) => (
            <View style={{ gap: 3 }}>
              <Row gap={5} style={{ alignItems: "center" }}>
                <Icon name="circle-user" size={12} color={tokens.mutedForeground} />
                <Text style={{ fontSize: 10, fontWeight: "600", color: tokens.foreground }}>{name}</Text>
              </Row>
              <View style={{ alignSelf: name === "You" ? "flex-end" : "flex-start", maxWidth: "88%", paddingHorizontal: 10, paddingVertical: 7, borderRadius: 9, backgroundColor: name === "You" ? tokens.primary : tokens.muted }}>
                <Text style={{ fontSize: 11, color: name === "You" ? tokens.primaryForeground : tokens.foreground }}>{body}</Text>
              </View>
            </View>
          ))}
        </View>
      </Scrollable>
    </View>
  );

  // section: Custom Scrollbars
  const custom = (
    <Row gap={16} style={{ flexWrap: "wrap" }}>
      {sample(
        "Native indicator",
        <Scrollable variant="card" padding={6} style={{ width: 200, height: 160 }}>
          {rows(20)}
        </Scrollable>,
        200,
      )}
      {sample(
        "Indicator hidden",
        <Scrollable variant="card" padding={6} indicator={false} style={{ width: 200, height: 160 }}>
          {rows(20)}
        </Scrollable>,
        200,
      )}
      {sample(
        "Inset surface",
        <Scrollable variant="inset" padding={4} style={{ width: 200, height: 160 }}>
          {rows(20)}
        </Scrollable>,
        200,
      )}
    </Row>
  );

  // section: Virtualization
  const virtualization = (
    <View style={{ width: 460, maxWidth: "100%", gap: 6 }}>
      <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Rows {first()}–{last()} of 300 are on screen; all 300 rows are built.</Text>
      <Scrollable variant="card" padding={6} style={{ height: 160 }} onScroll={(event) => virtualY.set(event.y)}>
        {rows(300)}
      </Scrollable>
    </View>
  );

  // section: Theme Customization
  const themed = (
    <Row gap={16} style={{ flexWrap: "wrap" }}>
      {sample(
        "Muted surface",
        <Scrollable variant="inset" padding={4} style={{ width: 200, height: 160 }}>
          {rows(20)}
        </Scrollable>,
        200,
      )}
      {sample(
        "Popover surface",
        <Scrollable variant="card" padding={6} style={{ width: 200, height: 160 }}>
          {rows(20)}
        </Scrollable>,
        200,
      )}
      {sample(
        "Page surface",
        <Scrollable padding={6} style={{ width: 200, height: 160, borderWidth: 1, borderColor: tokens.border, borderRadius: 8 }}>
          {rows(20)}
        </Scrollable>,
        200,
      )}
    </Row>
  );

  // section: Scrollbar Appearance
  const appearance = (
    <View style={{ width: 440, maxWidth: "100%", gap: 8 }}>
      <Scrollable variant="card" padding={8} style={{ height: 180 }}>
        {rows(24)}
      </Scrollable>
      <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>
        The indicator follows the resolved light/dark theme tokens and stays native to each platform.
      </Text>
    </View>
  );

  // section: Scrollbar Show Modes
  const showModes = (
    <Row gap={16} style={{ flexWrap: "wrap" }}>
      {sample(
        "Indicator on",
        <Scrollable variant="card" padding={6} style={{ width: 200, height: 160 }}>
          {rows(20)}
        </Scrollable>,
        200,
      )}
      {sample(
        "Indicator off",
        <Scrollable variant="card" padding={6} indicator={false} style={{ width: 200, height: 160 }}>
          {rows(20)}
        </Scrollable>,
        200,
      )}
    </Row>
  );

  // section: System Integration
  const integration = (
    <View style={{ width: 440, maxWidth: "100%", gap: 6 }}>
      <Scrollable horizontal variant="card" padding={8} snapToInterval={128} decelerationRate="fast" bounces={false} style={{ height: 112 }}>
        {cards(12)}
      </Scrollable>
      <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Cards snap to 128 pt; the platform owns momentum and bounce.</Text>
    </View>
  );

  // section: Data Table with Virtual Scrolling
  const table = (
    <View style={{ width: 500, maxWidth: "100%" }}>
      <Scrollable horizontal variant="card" style={{ height: 220 }}>
        <View style={{ minWidth: 620 }}>
          <Row style={{ height: 30, borderBottomWidth: 1, borderBottomColor: tokens.border, alignItems: "center", gap: 12, paddingHorizontal: 10, backgroundColor: tokens.muted }}>
            {["Name", "Status", "Owner", "Updated"].map((heading) => (
              <Text style={{ width: heading === "Name" ? 180 : 110, fontSize: 10, fontWeight: "600", color: tokens.mutedForeground }}>{heading}</Text>
            ))}
          </Row>
          {Array.from({ length: 40 }, (_, index) => (
            <Row style={{ height: 32, borderBottomWidth: 1, borderBottomColor: tokens.border, alignItems: "center", gap: 12, paddingHorizontal: 10 }}>
              <Text style={{ width: 180, fontSize: 10, color: tokens.foreground }}>Release {index + 1}</Text>
              <Text style={{ width: 110, fontSize: 10, color: tokens.success }}>Ready</Text>
              <Text style={{ width: 110, fontSize: 10, color: tokens.mutedForeground }}>Sofia</Text>
              <Text style={{ width: 110, fontSize: 10, color: tokens.mutedForeground }}>Today</Text>
            </Row>
          ))}
        </View>
      </Scrollable>
    </View>
  );
  // section: end

  return (
    <Page>
      <Caption>Scrollable is the native Scroll with the catalog's surfaces; the indicator, momentum and overscroll belong to the platform.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic Scrollable Container">{basic}</ExampleSection>
      <ExampleSection index={2} title="Vertical Scrolling">{verticalStory}</ExampleSection>
      <ExampleSection index={3} title="Horizontal Scrolling">{horizontalStory}</ExampleSection>
      <ExampleSection index={4} title="Both Directions">{bothStory}</ExampleSection>
      <ExampleSection index={5} title="File Browser with Scrolling">{browser}</ExampleSection>
      <ExampleSection index={6} title="Chat Messages with Auto-scroll">{chat}</ExampleSection>
      <ExampleSection index={7} title="Custom Scrollbars">{custom}</ExampleSection>
      <ExampleSection index={8} title="Virtualization">{virtualization}</ExampleSection>
      <ExampleSection index={9} title="Theme Customization">{themed}</ExampleSection>
      <ExampleSection index={10} title="Scrollbar Appearance">{appearance}</ExampleSection>
      <ExampleSection index={11} title="Scrollbar Show Modes">{showModes}</ExampleSection>
      <ExampleSection index={12} title="System Integration">{integration}</ExampleSection>
      <ExampleSection index={13} title="Data Table with Virtual Scrolling">{table}</ExampleSection>
    </Page>
  );
}
