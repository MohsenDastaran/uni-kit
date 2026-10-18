//! `description-list` — key/value rows, sizes, borders and multi-column spans.
import { DescriptionList } from "../components/description_list";
import { Caption, ExampleSection, Page } from "./ui";

const base = [
  { label: "Name", value: "Ada Lovelace" },
  { label: "Email", value: "ada@example.com" },
  { label: "Role", value: "Engineer" },
];
// section: Basic Description List
const basic = <DescriptionList items={base} />;
// section: Different Layouts
const layouts = <DescriptionList items={base} columns={2} />;
// section: With Separators
const separators = <DescriptionList items={base} separators />;
// section: Different Sizes
const sizes = <DescriptionList items={base} size="small" />;
// section: Without Borders
const borders = <DescriptionList items={base} bordered />;
// section: Complex Example with Mixed Content
const complex = (
  <DescriptionList
    bordered
    items={[
      { label: "Project", value: "uni-kit" },
      { label: "Framework", value: "NSH" },
      { label: "License", value: "Apache-2.0" },
      { label: "Components", value: "79" },
    ]}
  />
);
// section: end
export function Demo() {
  return (
    <Page>
      <Caption>Description lists are for read-only details; a form is for editing them.</Caption>
      <ExampleSection index={0} title="Basic Description List">{basic}</ExampleSection>
      <ExampleSection index={1} title="Different Layouts">{layouts}</ExampleSection>
      <ExampleSection index={2} title="With Separators">{separators}</ExampleSection>
      <ExampleSection index={3} title="Different Sizes">{sizes}</ExampleSection>
      <ExampleSection index={4} title="Without Borders">{borders}</ExampleSection>
      <ExampleSection index={5} title="Complex Example with Mixed Content">{complex}</ExampleSection>
    </Page>
  );
}
