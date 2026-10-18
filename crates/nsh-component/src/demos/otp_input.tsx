//! `otp-input` — one box per digit, with focus following the code.
import { signal, Text } from "nsh";
import { OtpInput } from "../components/otp_input";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  const code = signal("");
  // section: Usage
  const usage = <OtpInput value={code()} onChange={(next) => code.set(next)} />;
  // section: Behavior
  const behavior = <OtpInput length={4} onComplete={(next) => code.set(next)} />;
  // section: Common Patterns
  const patterns = <OtpInput length={6} size="large" />;
  // section: Programmatic Control
  const programmatic = <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>Code: {code() || "—"}</Text>;
  // section: end
  return (
    <Page>
      <Caption>A one-time code field moves focus as each digit is entered.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Behavior">{behavior}</ExampleSection>
      <ExampleSection index={2} title="Common Patterns">{patterns}</ExampleSection>
      <ExampleSection index={3} title="Programmatic Control">{programmatic}</ExampleSection>
    </Page>
  );
}
