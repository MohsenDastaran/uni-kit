//! `rating` — an interactive star rating.
import { signal } from "nsh";
import { Rating } from "../components/rating";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const value = signal(3);
  // section: Rate this
  const basic = <Rating value={value()} onChange={(next) => value.set(next)} />;
  // section: Controlled Rating
  const controlled = <Rating value={value()} onChange={(next) => value.set(next)} size={26} />;
  // section: Different Sizes
  const sizes = <Row gap={16}><Rating value={2} size={16} /><Rating value={3} size={20} /><Rating value={4} size={28} /></Row>;
  // section: Custom Color
  const color = <Rating value={4} color={tokens.danger} />;
  // section: Disabled State
  const disabled = <Rating value={3} disabled />;
  // section: Custom Maximum
  const maximum = <Rating value={7} max={10} size={18} />;
  // section: Read-only Display
  const readOnly = <Rating value={4} readOnly />;
  // section: end
  return (
    <Page>
      <Caption>Use a rating for a subjective score; a slider is for a continuous value.</Caption>
      <ExampleSection index={0} title="Rate this">{basic}</ExampleSection>
      <ExampleSection index={1} title="Controlled Rating">{controlled}</ExampleSection>
      <ExampleSection index={2} title="Different Sizes">{sizes}</ExampleSection>
      <ExampleSection index={3} title="Custom Color">{color}</ExampleSection>
      <ExampleSection index={4} title="Disabled State">{disabled}</ExampleSection>
      <ExampleSection index={5} title="Custom Maximum">{maximum}</ExampleSection>
      <ExampleSection index={6} title="Read-only Display">{readOnly}</ExampleSection>
    </Page>
  );
}
