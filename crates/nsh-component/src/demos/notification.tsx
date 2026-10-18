//! `notification` — in-app toasts.
import { signal } from "nsh";
import { NotificationHost, notify } from "../components/notification";
import { Button } from "../components/button";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const count = signal(0);
  // section: Basic
  const basic = <Button label="Show notification" variant="primary" onPress={() => { notify({ title: "Saved", description: "Your changes are on the server." }); count.update((n) => n + 1); }} />;
  // section: Variants
  const variants = <Row gap={8}>{["info", "success", "warning", "danger"].map((variant) => <Button label={variant} onPress={() => notify({ title: variant, description: `A ${variant} toast.`, variant: variant as "info" | "success" | "warning" | "danger" })} />)}</Row>;
  // section: With action
  const action = <Button label="Show with description" onPress={() => notify({ title: "Upload complete", description: "report.pdf is ready.", variant: "success" })} />;
  // section: Auto dismiss
  const auto = <Button label="Short toast" onPress={() => notify({ title: "Copied", duration: 1500 })} />;
  // section: Position
  const position = <Button label="Corner toast" onPress={() => notify({ title: "Top right", description: "The host is an absolute layer." })} />;
  // section: end
  return (
    <Page>
      <NotificationHost />
      <Caption>NSH has no system toast API; this host is an in-app layer, so it works on every backend.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Variants">{variants}</ExampleSection>
      <ExampleSection index={2} title="With action">{action}</ExampleSection>
      <ExampleSection index={3} title="Auto dismiss">{auto}</ExampleSection>
      <ExampleSection index={4} title="Position">{position}</ExampleSection>
      <Caption>Shown: {count()}</Caption>
    </Page>
  );
}
