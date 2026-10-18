//! `dialog` — a modal panel with a backdrop.
import { signal } from "nsh";
import { Dialog } from "../components/dialog";
import { Button } from "../components/button";
import { Input } from "../components/input";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  const basicOpen = signal(false);
  const wideOpen = signal(false);
  const controlledOpen = signal(false);
  // section: Basic
  const basic = (
    <>
      <Button label="Open dialog" variant="primary" onPress={() => basicOpen.set(true)} />
      <Dialog
        open={basicOpen()}
        title="Edit profile"
        description="Make a change and save."
        onClose={() => basicOpen.set(false)}
        footer={<><Button label="Cancel" variant="ghost" onPress={() => basicOpen.set(false)} /><Button label="Save" onPress={() => basicOpen.set(false)} /></>}
      >
        <Input placeholder="Name" onChangeText={() => {}} />
      </Dialog>
    </>
  );
  // section: With footer
  const footer = <Button label="Open with footer" onPress={() => basicOpen.set(true)} />;
  // section: Sizes
  const sizes = <Button label="Open wide dialog" onPress={() => wideOpen.set(true)} />;
  // section: Overlay click
  const overlay = (
    <Dialog open={wideOpen()} size="large" title="A wide dialog" description="The backdrop closes it." overlayClosable onClose={() => wideOpen.set(false)}>
      <Input placeholder="Search" onChangeText={() => {}} />
    </Dialog>
  );
  // section: Controlled
  const controlled = (
    <>
      <Button label="Open controlled" onPress={() => controlledOpen.set(true)} />
      <Dialog open={controlledOpen()} title="Controlled" onClose={() => controlledOpen.set(false)} />
    </>
  );
  // section: end
  return (
    <Page>
      <Caption>Dialog owns the layer and the focus; the content and the footer are yours.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="With footer">{footer}</ExampleSection>
      <ExampleSection index={2} title="Sizes">{sizes}</ExampleSection>
      <ExampleSection index={3} title="Overlay click">{overlay}</ExampleSection>
      <ExampleSection index={4} title="Controlled">{controlled}</ExampleSection>
    </Page>
  );
}
