//! `text-view` — a small Markdown renderer built from Text and View.
import { TextView } from "../components/text_view";
import { Caption, ExampleSection, Page } from "./ui";

const markdown = `# Release notes\n\nA short summary of what changed.\n\n## Components\n\n- Button\n- Card\n- Input\n\n> The source is yours once it is copied.\n\n\`\`\`tsx\n<Button label="Save" />\n\`\`\``;

// section: Markdown
const markdownView = <TextView content={markdown} />;
// section: HTML
const html = <TextView content="HTML is not rendered by an NSH app; this component reads Markdown-like text." />;
// section: Clamp to a number of lines
const clamp = <TextView content={markdown} maxLines={4} />;
// section: Link Click Handling
const link = <TextView content="Links are plain text in this partial port; use a Text span with onPress for a link." />;
// section: end
export function Demo() {
  return (
    <Page>
      <Caption>TextView renders headings, paragraphs, lists, quotes and fenced code; inline emphasis and HTML are not part of this port.</Caption>
      <ExampleSection index={0} title="Markdown">{markdownView}</ExampleSection>
      <ExampleSection index={1} title="HTML">{html}</ExampleSection>
      <ExampleSection index={2} title="Clamp to a number of lines">{clamp}</ExampleSection>
      <ExampleSection index={3} title="Link Click Handling">{link}</ExampleSection>
    </Page>
  );
}
