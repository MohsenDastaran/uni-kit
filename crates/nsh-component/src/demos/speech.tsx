//! `speech` — the dictation UI with an app-provided recognizer.
import { signal, View } from "nsh";
import { Speech } from "../components/speech";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const transcript = signal("");
  const listening = signal(false);
  const waveListening = signal(false);
  const waveTranscript = signal("");
  // section: Live waveform
  const waveform = (
    <View style={{ width: 480, maxWidth: "100%" }}>
      <Speech
        transcript={waveTranscript()}
        listening={waveListening()}
        onTranscript={(text) => waveTranscript.set(text)}
        onListeningChange={(next) => waveListening.set(next)}
      />
    </View>
  );
  // section: Usage
  const usage = (
    <View style={{ width: 440, maxWidth: "100%" }}>
      <Speech />
    </View>
  );
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
      <ExampleSection index={0} title="Live waveform">{waveform}</ExampleSection>
      <ExampleSection index={1} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={2} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={3} title="Dictate into an input">{input}</ExampleSection>
      <ExampleSection index={4} title="Button states">{states}</ExampleSection>
      <ExampleSection index={5} title="Session control">{control}</ExampleSection>
    </Page>
  );
}
