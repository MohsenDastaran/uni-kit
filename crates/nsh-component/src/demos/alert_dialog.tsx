//! `alert-dialog` — a dialog for one decision.
import { signal } from "nsh";
import { AlertDialog } from "../components/alert_dialog";
import { Button } from "../components/button";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  const open = signal(false);
  const destructive = signal(false);
  // section: Basic
  const basic = <><Button label="Continue?" onPress={() => open.set(true)} /><AlertDialog open={open()} title="Continue?" description="This action needs confirmation." onConfirm={() => open.set(false)} onCancel={() => open.set(false)} /></>;
  // section: Destructive
  const danger = <><Button label="Delete project" variant="danger" onPress={() => destructive.set(true)} /><AlertDialog open={destructive()} title="Delete this project?" description="It cannot be undone." destructive confirmLabel="Delete" onConfirm={() => destructive.set(false)} onCancel={() => destructive.set(false)} /></>;
  // section: Loading
  const loading = <AlertDialog open={false} title="Working" loading confirmLabel="Working…" />;
  // section: Controlled
  const controlled = <AlertDialog open={false} title="Controlled" />;
  // section: end
  return (
    <Page>
      <Caption>AlertDialog is a dialog with a decision-shaped footer; it does not close on a backdrop click.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Destructive">{danger}</ExampleSection>
      <ExampleSection index={2} title="Loading">{loading}</ExampleSection>
      <ExampleSection index={3} title="Controlled">{controlled}</ExampleSection>
    </Page>
  );
}
