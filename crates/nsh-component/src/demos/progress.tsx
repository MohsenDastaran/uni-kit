//! `progress` — linear and circular progress with values, sizes and statuses.
import { signal } from "nsh";
import { Progress, ProgressCircle } from "../components/progress";
import { Button } from "../components/button";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const value = signal(40);
  // section: Progress
  const basic = <Progress value={value()} />;
  // section: Values
  const values = <Row gap={16} style={{ flexWrap: "wrap" }}><Progress value={0} /><Progress value={25} /><Progress value={50} /><Progress value={75} /><Progress value={100} /></Row>;
  // section: Sizes and status colors
  const sizes = <Row gap={16} style={{ flexWrap: "wrap" }}><Progress value={60} size="small" /><Progress value={60} status="success" /><Progress value={60} status="warning" /><Progress value={60} status="danger" /></Row>;
  // section: Progress circle
  const circle = <Row gap={16}><ProgressCircle value={value()} /><ProgressCircle value={75} status="success" /><ProgressCircle value={40} status="danger" /></Row>;
  // section: Custom Style
  const custom = <Progress value={value()} label="Uploading" status="info" />;
  // section: end
  return (
    <Page>
      <Caption>Progress shows completion; a spinner is for work with no measurable end.</Caption>
      <ExampleSection index={0} title="Progress">{basic}</ExampleSection>
      <ExampleSection index={1} title="Values">{values}</ExampleSection>
      <ExampleSection index={2} title="Sizes and status colors">{sizes}</ExampleSection>
      <ExampleSection index={3} title="Progress circle">{circle}</ExampleSection>
      <ExampleSection index={4} title="Custom Style">{custom}</ExampleSection>
      <Row gap={8}>
        <Button label="Less" size="small" onPress={() => value.update((n) => Math.max(0, n - 10))} />
        <Button label="More" size="small" variant="primary" onPress={() => value.update((n) => Math.min(100, n + 10))} />
      </Row>
    </Page>
  );
}
