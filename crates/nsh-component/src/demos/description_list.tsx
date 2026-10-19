//! `description-list` — metadata rows, a bordered label/value table, multi-
//! column spans and rich values.
import { Text, View } from "nsh";
import { Badge } from "../components/badge";
import { DescriptionItemView, DescriptionList, type DescriptionItem } from "../components/description_list";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  const base = [
    { label: "Name", value: "Ada Lovelace" },
    { label: "Email", value: "ada@example.com" },
    { label: "Role", value: "Engineer" },
  ];

  const packageItems: DescriptionItem[] = [
    { label: "Name", value: "GPUI Component", span: 3 },
    {
      label: "Description",
      span: 3,
      content: (
        <View style={{ gap: 8 }}>
          <Text style={{ fontSize: 12, lineHeight: 19, color: tokens.foreground }}>
            UI components for building fantastic desktop applications by using GPUI.
          </Text>
          <Text style={{ fontSize: 12, lineHeight: 19, color: tokens.foreground }}>
            Contains a lot of useful UI components, such as Button, Input, Table, List, Select, DatePicker…
          </Text>
          <Text style={{ fontSize: 12, lineHeight: 19, color: tokens.foreground }}>
            You can easily create your native desktop application by using GPUI Component.
          </Text>
        </View>
      ),
    },
    { label: "Version", value: "0.1.0", span: 1 },
    { label: "License", value: "Apache-2.0", span: 1 },
    { label: "Author", value: "Longbridge", span: 1 },
    {
      label: "Repository",
      span: 2,
      content: (
        <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.primary }}>
          https://github.com/longbridge/gpui-kit
        </Text>
      ),
    },
    { label: "Category", value: "UI, Desktop, Framework", span: 1 },
    { label: "This is a long label for Platform", value: "macOS, Windows, Linux", span: 3 },
  ];

  // section: Usage
  const usage = (
    <DescriptionList
      bordered
      labelWidth={116}
      style={{ width: 560, maxWidth: "100%" }}
      items={[
        { label: "Name", value: "GPUI Component" },
        { label: "Version", value: "0.1.0" },
        { label: "License", value: "Apache-2.0" },
      ]}
    />
  );

  // section: Basic Description List
  const basic = (
    <DescriptionList bordered labelWidth={116} items={base} style={{ width: 520, maxWidth: "100%" }} />
  );

  // section: Using DescriptionItem Builder
  const builder = (
    <View style={{ width: 520, maxWidth: "100%", borderWidth: 1, borderColor: tokens.border, borderRadius: 8, padding: 12, gap: 12 }}>
      <DescriptionItemView label="Name">
        <Text style={{ fontSize: 12, color: tokens.foreground }}>Ada Lovelace</Text>
      </DescriptionItemView>
      <DescriptionItemView label="Email">
        <Text style={{ fontSize: 12, color: tokens.foreground }}>ada@example.com</Text>
      </DescriptionItemView>
      <DescriptionItemView label="Role">
        <Badge label="Engineer" variant="secondary" />
      </DescriptionItemView>
    </View>
  );

  // section: Different Layouts
  const layouts = (
    <DescriptionList bordered columns={2} labelWidth={104} items={[...base, { label: "Location", value: "London, UK" }]} style={{ width: 620, maxWidth: "100%" }} />
  );

  // section: Multiple Columns with Spans
  const spans = (
    <DescriptionList
      bordered
      columns={3}
      labelWidth={104}
      style={{ width: 700, maxWidth: "100%" }}
      items={[
        { label: "Project", value: "uni-kit", span: 2 },
        { label: "Framework", value: "NSH" },
        { label: "License", value: "Apache-2.0" },
        { label: "Components", value: "79" },
        { label: "Version", value: "0.1.0" },
        { label: "Platforms", value: "macOS, Windows, Linux", span: 3 },
      ]}
    />
  );

  // section: With Separators
  const separators = (
    <View style={{ display: "flex", flexDirection: "column", gap: 12 }}>
      <DescriptionList bordered separators items={base} style={{ width: 520, maxWidth: "100%" }} />
      <DescriptionList bordered separators={false} items={base} style={{ width: 520, maxWidth: "100%" }} />
    </View>
  );

  // section: Different Sizes
  const sizes = (
    <View style={{ display: "flex", flexDirection: "column", gap: 12 }}>
      <DescriptionList bordered size="small" items={base} style={{ width: 520, maxWidth: "100%" }} />
      <DescriptionList bordered size="large" items={base} style={{ width: 520, maxWidth: "100%" }} />
    </View>
  );

  // section: Without Borders
  const borders = (
    <DescriptionList items={base} style={{ width: 520, maxWidth: "100%" }} />
  );

  // section: Custom Label Width
  const widths = (
    <View style={{ display: "flex", flexDirection: "column", gap: 12 }}>
      <DescriptionList bordered labelWidth={80} items={base} style={{ width: 480, maxWidth: "100%" }} />
      <DescriptionList bordered labelWidth={150} items={base} style={{ width: 560, maxWidth: "100%" }} />
    </View>
  );

  // section: Rich Content with Custom Elements
  const rich = (
    <DescriptionList
      bordered
      labelWidth={130}
      style={{ width: 620, maxWidth: "100%" }}
      items={[
        { label: "Status", content: <Badge label="Active" variant="success" /> },
        { label: "Owner", content: <Text style={{ fontSize: 12, color: tokens.foreground }}>Sarah Chen · Design</Text> },
        { label: "Tags", content: <View style={{ flexDirection: "row", gap: 4 }}><Badge label="Desktop" variant="secondary" /><Badge label="Framework" variant="outline" /></View> },
        { label: "Repository", content: <Text style={{ fontSize: 12, color: tokens.primary }}>github.com/longbridge/gpui-kit</Text> },
      ]}
    />
  );

  // section: Complex Example with Mixed Content
  const complex = (
    <DescriptionList
      bordered
      columns={2}
      labelWidth={104}
      style={{ width: 620, maxWidth: "100%" }}
      items={[
        { label: "Project", value: "uni-kit" },
        { label: "Framework", value: "NSH" },
        { label: "License", value: "Apache-2.0" },
        { label: "Components", value: "79" },
        { label: "Status", content: <Badge label="Active" variant="success" /> },
        { label: "Version", value: "0.1.0" },
      ]}
    />
  );

  // section: User Profile Information
  const profile = (
    <DescriptionList
      bordered
      labelWidth={110}
      style={{ width: 600, maxWidth: "100%" }}
      items={[
        { label: "Full name", value: "Sarah Chen" },
        { label: "Email", value: "sarah.chen@acmecorp.com" },
        { label: "Department", value: "Design & User Experience" },
        { label: "Location", value: "San Francisco, CA" },
        { label: "Start date", value: "March 15, 2022" },
        { label: "Manager", value: "James Rodriguez, VP of Design" },
      ]}
    />
  );

  // section: System Information
  const system = (
    <DescriptionList
      bordered
      columns={2}
      labelWidth={104}
      style={{ width: 640, maxWidth: "100%" }}
      items={[
        { label: "Status", content: <View style={{ flexDirection: "row", alignItems: "center", gap: 5 }}><View style={{ width: 6, height: 6, borderRadius: 3, backgroundColor: tokens.success }} /><Text style={{ fontSize: 12, color: tokens.success }}>Operational</Text></View> },
        { label: "Instance", value: "c6g.2xlarge" },
        { label: "Region", value: "US East (N. Virginia)" },
        { label: "OS", value: "Amazon Linux 2023" },
        { label: "Uptime", value: "47 days, 12 hours" },
        { label: "Memory", value: "10.7 / 16 GB" },
        { label: "CPU", value: "42%" },
        { label: "Tags", value: "production · api · critical" },
      ]}
    />
  );

  // section: Product Specifications
  const product = (
    <DescriptionList
      bordered
      columns={2}
      labelWidth={104}
      style={{ width: 640, maxWidth: "100%" }}
      items={[
        { label: "Model", value: "Wireless Headphones X2" },
        { label: "SKU", value: "AUD-012" },
        { label: "Battery", value: "Up to 40 hours" },
        { label: "Connectivity", value: "Bluetooth 5.3" },
        { label: "Weight", value: "254 g" },
        { label: "Warranty", value: "2 years" },
      ]}
    />
  );

  // section: Package metadata
  const metadata = (
    <DescriptionList
      bordered
      columns={3}
      labelWidth={112}
      style={{ width: 760, maxWidth: "100%" }}
      items={packageItems}
    />
  );
  // section: end

  return (
    <Page>
      <Caption>
        Description lists are for read-only details. The bordered form is a metadata
        table: a muted label column, a flexible value column and optional spans.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic Description List">{basic}</ExampleSection>
      <ExampleSection index={2} title="Using DescriptionItem Builder">{builder}</ExampleSection>
      <ExampleSection index={3} title="Different Layouts">{layouts}</ExampleSection>
      <ExampleSection index={4} title="Multiple Columns with Spans">{spans}</ExampleSection>
      <ExampleSection index={5} title="With Separators">{separators}</ExampleSection>
      <ExampleSection index={6} title="Different Sizes">{sizes}</ExampleSection>
      <ExampleSection index={7} title="Without Borders">{borders}</ExampleSection>
      <ExampleSection index={8} title="Custom Label Width">{widths}</ExampleSection>
      <ExampleSection index={9} title="Rich Content with Custom Elements">{rich}</ExampleSection>
      <ExampleSection index={10} title="Complex Example with Mixed Content">{complex}</ExampleSection>
      <ExampleSection index={11} title="User Profile Information">{profile}</ExampleSection>
      <ExampleSection index={12} title="System Information">{system}</ExampleSection>
      <ExampleSection index={13} title="Product Specifications">{product}</ExampleSection>
      <ExampleSection index={14} title="Package metadata">{metadata}</ExampleSection>
    </Page>
  );
}
