//! `tag` — variants, sizes, outlines, custom colours and collections.
import { Tag, type TagVariant } from "../components/tag";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

const variants: TagVariant[] = ["default", "secondary", "outline", "danger", "warning", "success", "info"];

// section: Tags
const tags = <Row gap={8}><Tag label="Design" /><Tag label="Engineering" variant="secondary" /><Tag label="Docs" variant="outline" /></Row>;
// section: Tag Variants
const variantRow = <Row gap={8}>{variants.map((variant) => <Tag label={variant} variant={variant} />)}</Row>;
// section: Outline Tags
const outline = <Row gap={8}><Tag label="Draft" variant="outline" /><Tag label="Review" variant="outline" /><Tag label="Ready" variant="outline" /></Row>;
// section: Tag Sizes
const sizes = <Row gap={8} style={{ alignItems: "center" }}><Tag label="Small" size="small" /><Tag label="Medium" size="medium" /><Tag label="Large" size="large" /></Row>;
// section: Custom Colors
const colors = <Row gap={8}><Tag label="Custom" color="#8250df" /><Tag label="Sunset" color="#e76e50" /><Tag label="Mint" color="#22c55e" /></Row>;
// section: Rounded Corners
const rounded = <Row gap={8}><Tag label="Pill one" rounded /><Tag label="Pill two" rounded variant="secondary" /><Tag label="Pill three" rounded variant="outline" /></Row>;
// section: Combined Styles
const combined = <Row gap={8}><Tag label="Feature" icon="sparkles" variant="info" rounded /><Tag label="Bug" icon="circle-alert" variant="danger" /><Tag label="Done" icon="check" variant="success" closable /></Row>;
// section: Tag Collections
const collection = <Row gap={8} style={{ flexWrap: "wrap" }}>{["React", "Rust", "TSX", "GPUI", "Slint", "NSH"].map((label) => <Tag label={label} />)}</Row>;
// section: Status Dashboard Tags
const status = <Row gap={8}><Tag label="Healthy" variant="success" /><Tag label="Degraded" variant="warning" /><Tag label="Down" variant="danger" /><Tag label="Unknown" variant="secondary" /></Row>;
// section: end
export function Demo() {
  return (
    <Page>
      <Caption>Tags are compact labels; use a badge for a notification count and a button for an action.</Caption>
      <ExampleSection index={0} title="Tags">{tags}</ExampleSection>
      <ExampleSection index={1} title="Tag Variants">{variantRow}</ExampleSection>
      <ExampleSection index={2} title="Outline Tags">{outline}</ExampleSection>
      <ExampleSection index={3} title="Tag Sizes">{sizes}</ExampleSection>
      <ExampleSection index={4} title="Custom Colors">{colors}</ExampleSection>
      <ExampleSection index={5} title="Rounded Corners">{rounded}</ExampleSection>
      <ExampleSection index={6} title="Combined Styles">{combined}</ExampleSection>
      <ExampleSection index={7} title="Tag Collections">{collection}</ExampleSection>
      <ExampleSection index={8} title="Status Dashboard Tags">{status}</ExampleSection>
    </Page>
  );
}
