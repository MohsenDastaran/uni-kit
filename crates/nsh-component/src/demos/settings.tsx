//! `settings` — grouped settings rows and pages.
import { signal } from "nsh";
import { Settings } from "../components/settings";
import { Switch } from "../components/switch";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const active = signal("general");
  const sounds = signal(true);
  const makeSections = () => [
    { id: "general", title: "General", items: [{ id: "name", label: "Name", description: "Ada Lovelace" }, { id: "sounds", label: "Sounds", control: <Switch checked={sounds()} onChange={(next) => sounds.set(next)} /> }] },
    { id: "appearance", title: "Appearance", items: [{ id: "theme", label: "Theme", description: "System" }, { id: "density", label: "Density", description: "Comfortable" }] },
    { id: "advanced", title: "Advanced", items: [{ id: "reset", label: "Reset settings" }] },
  ];
  // section: Basic Settings
  const basic = <Settings sections={makeSections()} active={active()} onChange={(next) => active.set(next)} />;
  // section: Group Variants
  const variants = <Settings sections={makeSections().slice(0, 2)} active={active()} onChange={(next) => active.set(next)} />;
  // section: Complete Settings Example
  const complete = <Settings sections={makeSections()} />;
  // section: end
  return (
    <Page>
      <Caption>Settings is layout over Group; the controls in each row are ordinary catalog controls.</Caption>
      <ExampleSection index={0} title="Basic Settings">{basic}</ExampleSection>
      <ExampleSection index={1} title="Group Variants">{variants}</ExampleSection>
      <ExampleSection index={2} title="Complete Settings Example">{complete}</ExampleSection>
      <Status>Active: {active()} · sounds {sounds() ? "on" : "off"}</Status>
    </Page>
  );
}
