//! `color-picker` — featured swatches, a hex field and alpha.
import { signal } from "nsh";
import { ColorPicker } from "../components/color_picker";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const color = signal("#3b82f6");
  // section: Usage
  const usage = <ColorPicker value={color()} onChange={(next) => color.set(next)} />;
  // section: Different Sizes
  const sizes = <ColorPicker value={color()} onChange={(next) => color.set(next)} size="large" />;
  // section: With Custom Featured Colors
  const featured = <ColorPicker value={color()} onChange={(next) => color.set(next)} featured={["#171717", "#ef4444", "#22c55e", "#3b82f6"]} />;
  // section: Theme Color
  const theme = <ColorPicker value={tokens.primary} onChange={(next) => color.set(next)} />;
  // section: Color Select
  const select = <ColorPicker value={color()} onChange={(next) => color.set(next)} showAlpha />;
  // section: end
  return (
    <Page>
      <Caption>NSH has no platform colour dialog; the picker is swatches and a hex field, with the value under the app's control.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Different Sizes">{sizes}</ExampleSection>
      <ExampleSection index={2} title="With Custom Featured Colors">{featured}</ExampleSection>
      <ExampleSection index={3} title="Theme Color">{theme}</ExampleSection>
      <ExampleSection index={4} title="Color Select">{select}</ExampleSection>
      <Status>Selected: {color()}</Status>
    </Page>
  );
}
