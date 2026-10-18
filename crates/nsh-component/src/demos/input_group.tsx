//! `input-group` — an input framed with text, icons or actions.
import { signal } from "nsh";
import { InputGroup, InputGroupButton, InputGroupInput, InputGroupText } from "../components/input_group";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  const value = signal("");
  const search = signal("");
  // section: Input with a clear button
  const clear = <InputGroup><InputGroupInput value={value()} placeholder="Search" onChangeText={(next) => value.set(next)} /><InputGroupButton label="Clear" onPress={() => value.set("")} /></InputGroup>;
  // section: Parts and alignment
  const parts = <InputGroup><InputGroupText>https://</InputGroupText><InputGroupInput placeholder="example.com" /><InputGroupText>.dev</InputGroupText></InputGroup>;
  // section: Buttons, icons, and menus
  const buttons = <InputGroup><InputGroupInput value={search()} placeholder="Filter" onChangeText={(next) => search.set(next)} /><InputGroupButton label="Go" onPress={() => {}} /></InputGroup>;
  // section: Disabled, read-only, and validation
  const disabled = <InputGroup disabled><InputGroupInput placeholder="Disabled" /></InputGroup>;
  // section: Sizes and custom styles
  const sizes = <InputGroup size="large"><InputGroupInput placeholder="Large" /></InputGroup>;
  // section: end
  return (
    <Page>
      <Caption>InputGroup draws the frame; the input inside is the standard Input.</Caption>
      <ExampleSection index={0} title="Input with a clear button">{clear}</ExampleSection>
      <ExampleSection index={1} title="Parts and alignment">{parts}</ExampleSection>
      <ExampleSection index={2} title="Buttons, icons, and menus">{buttons}</ExampleSection>
      <ExampleSection index={3} title="Disabled, read-only, and validation">{disabled}</ExampleSection>
      <ExampleSection index={4} title="Sizes and custom styles">{sizes}</ExampleSection>
    </Page>
  );
}
