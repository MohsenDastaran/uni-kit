//! `speech` — the dictation UI with an app-provided recognizer.
import { signal } from "nsh";
import { Speech } from "../components/speech";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const transcript = signal("");
  const listening = signal(false);
  // section: Basic
  const basic = <Speech transcript={transcript()} listening={listening()} onTranscript={(text) => transcript.set(text)} onListeningChange={(next) => listening.set(next)} />;
  // section: Dictate into an input
  const input = <Speech />;
  // section: Button states
  const states = <Speech listening onListeningChange={() => {}} />;
  // section: Session control
  const control = <Status>{listening() ? "listening" : "idle"} · {transcript() || "no transcript"}</Status>;
  // section: end
  return (
    <Page>
      <Caption>NSH has no microphone API. This port exposes an app-provided recognizer interface and a scripted demo; it is a partial port.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Dictate into an input">{input}</ExampleSection>
      <ExampleSection index={2} title="Button states">{states}</ExampleSection>
      <ExampleSection index={3} title="Session control">{control}</ExampleSection>
    </Page>
  );
}
