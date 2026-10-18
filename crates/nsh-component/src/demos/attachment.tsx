//! `attachment` — a file/media row with lifecycle and actions.
import { signal, Text } from "nsh";
import { Attachment, AttachmentGroup } from "../components/attachment";
import cover from "../../examples/assets/cover.png";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  const progress = signal(40);
  // section: Anatomy and basic usage
  const basic = <Attachment name="report.pdf" size="1.2 MB" icon="file-text" />;
  // section: Lifecycle states
  const states = <AttachmentGroup><Attachment name="uploading.pdf" status="uploading" progress={progress()} /><Attachment name="done.pdf" status="complete" /><Attachment name="failed.pdf" status="error" onRetry={() => progress.set(0)} /></AttachmentGroup>;
  // section: Media and image previews
  const media = <Attachment name="cover.png" size="640 × 320" image={cover} status="complete" />;
  // section: Groups
  const group = <AttachmentGroup><Attachment name="one.pdf" icon="file" /><Attachment name="two.pdf" icon="file" /><Attachment name="three.pdf" icon="file" /></AttachmentGroup>;
  // section: Content and actions
  const actions = <Attachment name="notes.txt" icon="file-text" onRemove={() => {}}>{<Text style={{ fontSize: 12, color: tokens.mutedForeground }}>Ready to send</Text>}</Attachment>;
  // section: end
  return (
    <Page>
      <Caption>Attachment is a presentational row; upload state, selection and previewing stay in the app.</Caption>
      <ExampleSection index={0} title="Anatomy and basic usage">{basic}</ExampleSection>
      <ExampleSection index={1} title="Lifecycle states">{states}</ExampleSection>
      <ExampleSection index={2} title="Media and image previews">{media}</ExampleSection>
      <ExampleSection index={3} title="Groups">{group}</ExampleSection>
      <ExampleSection index={4} title="Content and actions">{actions}</ExampleSection>
    </Page>
  );
}
