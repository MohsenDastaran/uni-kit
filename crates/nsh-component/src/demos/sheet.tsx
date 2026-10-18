//! `sheet` — a bottom sheet over a dimmed backdrop.
import { signal } from "nsh";
import { Sheet } from "../components/sheet";
import { Button } from "../components/button";
import { Input } from "../components/input";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  const basicOpen = signal(false);
  const formOpen = signal(false);
  const scrollOpen = signal(false);
  // section: Basic
  const basic = (
    <>
      <Button label="Open sheet" variant="primary" onPress={() => basicOpen.set(true)} />
      <Sheet open={basicOpen()} title="Share the photo" onClose={() => basicOpen.set(false)}>
        <Button label="Copy link" variant="secondary" onPress={() => basicOpen.set(false)} />
        <Button label="Done" onPress={() => basicOpen.set(false)} />
      </Sheet>
    </>
  );
  // section: With title
  const title = <Button label="Open with description" onPress={() => basicOpen.set(true)} />;
  // section: Form
  const form = (
    <>
      <Button label="Open form" onPress={() => formOpen.set(true)} />
      <Sheet open={formOpen()} title="Rename" onClose={() => formOpen.set(false)} footer={<Button label="Save" onPress={() => formOpen.set(false)} />}>
        <Input placeholder="New name" onChangeText={() => {}} />
      </Sheet>
    </>
  );
  // section: Scrollable
  const scrollable = (
    <>
      <Button label="Open long sheet" onPress={() => scrollOpen.set(true)} />
      <Sheet open={scrollOpen()} title="Long content" onClose={() => scrollOpen.set(false)}>
        {Array.from({ length: 8 }, (_, index) => <Button label={`Action ${index + 1}`} variant="ghost" onPress={() => scrollOpen.set(false)} />)}
      </Sheet>
    </>
  );
  // section: Controlled
  const controlled = <Sheet open={false} title="Controlled" onClose={() => {}} />;
  // section: end
  return (
    <Page>
      <Caption>The mobile Sheet runs the drag, the backdrop and the safe-area padding natively.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="With title">{title}</ExampleSection>
      <ExampleSection index={2} title="Form">{form}</ExampleSection>
      <ExampleSection index={3} title="Scrollable">{scrollable}</ExampleSection>
      <ExampleSection index={4} title="Controlled">{controlled}</ExampleSection>
    </Page>
  );
}
