//! `virtual-list` — a recycling list over thousands of rows.
import { Text, View } from "nsh";
import { VirtualList } from "../components/virtual_list";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Status } from "./ui";

const items = Array.from({ length: 5000 }, (_, index) => `Row ${index + 1}`);

// section: Basic
const basic = <VirtualList items={items} itemHeight={44} style={{ height: 360 }} />;
export function Demo() {
  // section: Custom rows
  const custom = <VirtualList items={items.slice(0, 100)} itemHeight={56} renderItem={(item, index) => <View style={{ height: 56, justifyContent: "center", paddingHorizontal: 12 }}><Text style={{ fontSize: 13, color: tokens.foreground }}>{`${index + 1}. ${item}`}</Text></View>} style={{ height: 300 }} />;
  // section: end
  return (
    <Page>
      <Caption>Only the rows near the viewport exist; scrolling recycles their cells.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Custom rows">{custom}</ExampleSection>
      <Status>5,000 rows</Status>
    </Page>
  );
}
