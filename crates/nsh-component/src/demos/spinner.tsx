//! `spinner` — sizes, status colours and loading states.
import { Text, View } from "nsh";
import { Button } from "../components/button";
import { Spinner } from "../components/spinner";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  // section: Colors
  const colors = <Row gap={16}><Spinner status="info" /><Spinner status="success" /><Spinner status="warning" /><Spinner status="danger" /></Row>;
  // section: Sizes
  const sizes = <Row gap={16} style={{ alignItems: "center" }}><Spinner size="small" /><Spinner size="medium" /><Spinner size="large" /><Spinner size={40} /></Row>;
  // section: In context
  const context = <Button label="Saving" loading variant="primary" onPress={() => {}} />;
  // section: Spinner with Custom Icon
  const custom = <Spinner icon="refresh-cw" size="large" color={tokens.info} />;
  // section: Loading States
  const states = <View style={{ gap: 10 }}><Spinner label="Loading projects…" /><Spinner overlay visible label="Loading" style={{ height: 120, borderRadius: 8 }} /></View>;
  // section: Size Variations
  const variations = <Row gap={16}><Spinner size="xsmall" /><Spinner size="small" /><Spinner size="medium" /><Spinner size="large" /></Row>;
  // section: Conditional Loading
  const conditional = <Spinner visible={false} label="Only shows while busy" />;
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
