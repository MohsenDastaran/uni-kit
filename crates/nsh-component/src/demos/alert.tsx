//! `alert` — an inline callout.
import { signal } from "nsh";
import { Alert } from "../components/alert";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  const visible = signal(true);
  // section: Default
  const normal = <Alert title="Heads up" description="This is a neutral callout." />;
  // section: Info
  const info = <Alert variant="info" title="Information" description="A new version is available." />;
  // section: Success
  const success = <Alert variant="success" title="Saved" description="Your changes are on the server." />;
  // section: Warning
  const warning = <Alert variant="warning" title="Check the form" description="One field still needs attention." />;
  // section: Danger
  const danger = <Alert variant="danger" title="Delete failed" description="The item could not be removed." />;
  // section: With title
  const title = <Alert variant="info" title="A titled alert" description="The description sits under the title." />;
  // section: Closable
  const closable = visible() ? <Alert variant="warning" title="Dismissible" closable onClose={() => visible.set(false)} /> : null;
  // section: Banner
  const banner = <Alert variant="info" banner title="Banner mode" description="The callout spans the edge." />;
  // section: end
  return (
    <Page>
      <Caption>Alerts are inline; use a dialog when the message needs an answer before the reader continues.</Caption>
      <ExampleSection index={0} title="Default">{normal}</ExampleSection>
      <ExampleSection index={1} title="Info">{info}</ExampleSection>
      <ExampleSection index={2} title="Success">{success}</ExampleSection>
      <ExampleSection index={3} title="Warning">{warning}</ExampleSection>
      <ExampleSection index={4} title="Danger">{danger}</ExampleSection>
      <ExampleSection index={5} title="With title">{title}</ExampleSection>
      <ExampleSection index={6} title="Closable">{closable}</ExampleSection>
      <ExampleSection index={7} title="Banner">{banner}</ExampleSection>
    </Page>
  );
}
