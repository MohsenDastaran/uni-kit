//! `spinner` — sizes, status colours and loading states.
import { signal, Text, View } from "nsh";
import { Button } from "../components/button";
import { Spinner } from "../components/spinner";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const busy = signal(true);
  // section: Colors
  const colors = <Row gap={16}><Spinner status="info" /><Spinner status="success" /><Spinner status="warning" /><Spinner status="danger" /></Row>;
  // section: Sizes
  const sizes = <Row gap={16} style={{ alignItems: "center" }}><Spinner size="small" /><Spinner size="medium" /><Spinner size="large" /><Spinner size={40} /></Row>;
  // section: In context
  const context = <Button label="Saving" loading variant="primary" onPress={() => {}} />;
  // section: Spinner with Custom Icon
  const custom = <Spinner icon="refresh-cw" size="large" color={tokens.info} />;
  // section: Loading States
  const states = (
    <View style={{ width: 440, maxWidth: "100%", gap: 10 }}>
      <View style={{ paddingHorizontal: 10, paddingVertical: 8, borderWidth: 1, borderColor: tokens.border, borderRadius: 8, backgroundColor: tokens.popover }}>
        <Spinner label="Loading projects…" />
      </View>
      <View style={{ position: "relative", height: 132, borderRadius: 9, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover, overflow: "hidden" }}>
        <View style={{ padding: 14, gap: 8 }}>
          <View style={{ width: 92, height: 9, borderRadius: 4, backgroundColor: tokens.muted }} />
          <View style={{ width: "100%", height: 8, borderRadius: 4, backgroundColor: tokens.muted }} />
          <View style={{ width: "72%", height: 8, borderRadius: 4, backgroundColor: tokens.muted }} />
          <View style={{ width: 68, height: 24, borderRadius: 6, backgroundColor: tokens.muted }} />
        </View>
        <Spinner overlay visible label="Loading" style={{ borderRadius: 9 }} />
      </View>
    </View>
  );
  // section: Size Variations
  const variations = (
    <View style={{ width: 440, maxWidth: "100%", padding: 14, borderWidth: 1, borderColor: tokens.border, borderRadius: 9, backgroundColor: tokens.popover }}>
      <Row gap={20} style={{ alignItems: "center" }}>
        <Spinner size="xsmall" />
        <Spinner size="small" />
        <Spinner size="medium" />
        <Spinner size="large" />
        <Spinner size={40} />
      </Row>
    </View>
  );
  // section: Conditional Loading
  const conditional = (
    <View style={{ width: 440, maxWidth: "100%", padding: 14, borderWidth: 1, borderColor: tokens.border, borderRadius: 9, backgroundColor: tokens.popover, gap: 10 }}>
      <Row gap={10} style={{ alignItems: "center", minHeight: 30 }}>
        <Spinner visible={busy()} label="Saving changes…" />
        {!busy() ? <Text style={{ fontSize: 12, color: tokens.success }}>Saved</Text> : null}
        <View style={{ flex: 1 }} />
        <Button label={busy() ? "Finish" : "Start"} size="small" variant={busy() ? "secondary" : "primary"} onPress={() => busy.set(!busy())} />
      </Row>
      <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>{busy() ? "Spinner is mounted while the request runs." : "Request complete; the spinner is unmounted."}</Text>
    </View>
  );
  // section: end
  return (
    <Page>
      <Caption>Spinners are for an operation with no measurable progress; use Progress when the completion is known.</Caption>
      <ExampleSection index={0} title="Colors">{colors}</ExampleSection>
      <ExampleSection index={1} title="Sizes">{sizes}</ExampleSection>
      <ExampleSection index={2} title="In context">{context}</ExampleSection>
      <ExampleSection index={3} title="Spinner with Custom Icon">{custom}</ExampleSection>
      <ExampleSection index={4} title="Loading States">{states}</ExampleSection>
      <ExampleSection index={5} title="Size Variations">{variations}</ExampleSection>
      <ExampleSection index={6} title="Conditional Loading">{conditional}</ExampleSection>
    </Page>
  );
}
