//! `stepper` — step-by-step process flows: numbered, dotted, cards, panels,
//! pills, arrows, vertical rails and nested steps.
import { signal, Text, View } from "nsh";
import { Button } from "../components/button";
import { Stepper, StepperDetail, type StepperItem } from "../components/stepper";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

const resolution: StepperItem[] = [
  { id: "choose", label: "Choose Resolution", description: "Pick one of the resolution options", icon: "sparkles" },
  { id: "execute", label: "Execute Resolution", description: "Verify details and proceed", icon: "play" },
  { id: "verify", label: "Verify Outcome", description: "Confirm the issue is resolved", icon: "check" },
  { id: "summary", label: "Resolution Summary", description: "View actions taken", icon: "file-text" },
];

const post: StepperItem[] = [
  { label: "Damrak 112" },
  { label: "Rokin 75" },
  { label: "Nieuwzijds 112", description: "On time" },
  { label: "Spuistraat 210" },
  { label: "Haarlemmerstraat 44" },
];

const panel: StepperItem[] = [
  { label: "Step 1", description: "Cart", color: tokens.info },
  { label: "Step 2", description: "Payment", color: tokens.warning },
  { label: "Step 3", description: "Review", color: tokens.primary },
  { label: "Step 4", description: "Done", color: tokens.success },
];

const workflow: StepperItem[] = [
  { label: "Lead" },
  { label: "Contact" },
  { label: "Qualify" },
  { label: "Discovery" },
  { label: "Proposal" },
  { label: "Negotiation" },
  { label: "Approval" },
];

const phases: StepperItem[] = [
  { label: "Intake" },
  { label: "Structure" },
  { label: "Kick-off" },
  { label: "Draft" },
  { label: "Consolidate" },
  { label: "Internal check" },
  { label: "Negotiate" },
  { label: "Package" },
];

export function Demo() {
  const base = signal(1);
  const icons = signal(1);
  const centered = signal(2);
  const verticalState = signal(2);
  const basic = signal(1);
  const disabledState = signal(1);
  const clicks = signal(0);
  const form = signal(1);
  const cards = signal(1);
  const dots = signal(3);
  const panelState = signal(1);
  const arrows = signal(2);
  const pills = signal(5);
  const rail = signal(2);
  const disabledItems = signal(0);
  const stage = (children: any, width = 780) => <View style={{ width, maxWidth: "100%", gap: 12 }}>{children}</View>;
  const detail = (items: StepperItem[], active: () => number, eyebrow?: string) => {
    const step = items[Math.min(active(), items.length - 1)];
    return <StepperDetail eyebrow={eyebrow ?? `Step ${active() + 1} of ${items.length}`} title={step.label} description={step.description} />;
  };

  // section: Usage
  const usage = stage(
    <View style={{ gap: 12 }}>
      <Stepper items={resolution} selectedIndex={base()} onChange={(next) => base.set(next)} showNavigation />
      {detail(resolution, base)}
    </View>,
  );
  // section: With icons
  const withIcons = stage(
    <View style={{ gap: 12 }}>
      <Stepper items={resolution} selectedIndex={icons()} onChange={(next) => icons.set(next)} size="large" showNavigation />
      {detail(resolution, icons)}
    </View>,
  );
  // section: Text center
  const textCenter = stage(
    <View style={{ gap: 14 }}>
      <Stepper items={resolution} selectedIndex={centered()} onChange={(next) => centered.set(next)} textCenter showNavigation />
      {detail(resolution, centered)}
    </View>,
  );
  // section: Vertical
  const vertical = (
    <View style={{ width: 620, maxWidth: "100%", padding: 16, borderWidth: 1, borderColor: tokens.border, borderRadius: 10, backgroundColor: tokens.popover }}>
      <Stepper items={post} selectedIndex={verticalState()} onChange={(next) => verticalState.set(next)} orientation="vertical" showNavigation />
    </View>
  );
  // section: Sizes and disabled
  const sizes = (
    <View style={{ width: 780, maxWidth: "100%", padding: 14, borderWidth: 1, borderColor: tokens.border, borderRadius: 10, backgroundColor: tokens.popover, gap: 14 }}>
      <Row gap={24} style={{ alignItems: "flex-start", flexWrap: "wrap" }}>
        <View style={{ gap: 6 }}><Text style={{ fontSize: 9, color: tokens.mutedForeground }}>xsmall</Text><Stepper items={resolution.slice(0, 3)} selectedIndex={1} size="xsmall" /></View>
        <View style={{ gap: 6 }}><Text style={{ fontSize: 9, color: tokens.mutedForeground }}>small</Text><Stepper items={resolution.slice(0, 3)} selectedIndex={1} size="small" /></View>
        <View style={{ gap: 6 }}><Text style={{ fontSize: 9, color: tokens.mutedForeground }}>medium</Text><Stepper items={resolution.slice(0, 3)} selectedIndex={1} /></View>
        <View style={{ gap: 6 }}><Text style={{ fontSize: 9, color: tokens.mutedForeground }}>large</Text><Stepper items={resolution.slice(0, 3)} selectedIndex={1} size="large" /></View>
      </Row>
      <Stepper items={[{ label: "Available" }, { label: "Unavailable", disabled: true }, { label: "Available" }]} selectedIndex={0} />
    </View>
  );
  // section: Basic Stepper
  const basicStepper = stage(
    <View style={{ gap: 12 }}>
      <Stepper items={resolution} selectedIndex={basic()} onChange={(next) => basic.set(next)} showNavigation />
      {detail(resolution, basic, "Billing setup")}
    </View>,
  );
  // section: Disabled State
  const disabled = (
    <View style={{ width: 620, maxWidth: "100%", padding: 16, borderWidth: 1, borderColor: tokens.border, borderRadius: 10, backgroundColor: tokens.popover, gap: 12 }}>
      <Stepper items={[{ label: "Account", description: "Complete" }, { label: "Billing", description: "Unavailable", disabled: true }, { label: "Review", description: "Locked", disabled: true }]} selectedIndex={0} />
      <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Disabled steps are skipped and cannot be selected.</Text>
    </View>
  );
  // section: Handle Click Events
  const events = stage(
    <View style={{ gap: 12 }}>
      <Stepper items={resolution} selectedIndex={clicks()} onChange={(next) => clicks.set(next)} showNavigation />
      {detail(resolution, clicks, "Selected by click or nav")}
    </View>,
  );
  // section: Multi-step Form
  const multiStep = (
    <View style={{ width: 620, maxWidth: "100%", padding: 16, borderWidth: 1, borderColor: tokens.border, borderRadius: 10, backgroundColor: tokens.popover, gap: 12 }}>
      <Stepper items={resolution} selectedIndex={form()} onChange={(next) => form.set(next)} variant="number" showNavigation />
      <View style={{ padding: 12, borderRadius: 8, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.background, gap: 5 }}>
        <Text style={{ fontSize: 11, fontWeight: "600", color: tokens.foreground }}>{resolution[form()].label}</Text>
        <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>{resolution[form()].description}</Text>
      </View>
    </View>
  );
  // section: Disabled Individual Steps
  const individual = (
    <View style={{ width: 680, maxWidth: "100%", padding: 16, borderWidth: 1, borderColor: tokens.border, borderRadius: 10, backgroundColor: tokens.popover, gap: 12 }}>
      <Stepper items={[{ label: "Account", description: "Ready" }, { label: "Verify email", description: "Waiting for approval", disabled: true }, { label: "Preferences", description: "Optional" }, { label: "Finish" }]} selectedIndex={disabledItems()} onChange={(next) => disabledItems.set(next)} showNavigation />
    </View>
  );
  // section: Cards
  const cardsView = (
    <View style={{ width: 620, maxWidth: "100%", gap: 12 }}>
      <Stepper items={resolution} selectedIndex={cards()} onChange={(next) => cards.set(next)} variant="cards" vertical />
      {detail(resolution, cards, "Active step")}
    </View>
  );
  // section: Dots
  const dotsView = (
    <View style={{ width: 620, maxWidth: "100%", gap: 12 }}>
      <View style={{ alignItems: "center", gap: 8 }}>
        <Stepper items={resolution} selectedIndex={dots()} onChange={(next) => dots.set(next)} variant="dots" />
        <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Step {dots() + 1} of {resolution.length}</Text>
        <Text style={{ fontSize: 13, fontWeight: "600", color: tokens.foreground }}>{resolution[dots()].label}</Text>
        <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>{resolution[dots()].description}</Text>
        <Row gap={8}>
          <Button label="Previous" size="small" variant="secondary" disabled={dots() === 0} onPress={() => dots.update((value) => Math.max(0, value - 1))} />
          <Button label="Next" size="small" variant="primary" disabled={dots() >= resolution.length - 1} onPress={() => dots.update((value) => Math.min(resolution.length - 1, value + 1))} />
        </Row>
      </View>
    </View>
  );
  // section: Panel
  const panelView = (
    <View style={{ width: 780, maxWidth: "100%", gap: 12 }}>
      <Stepper items={panel} selectedIndex={panelState()} onChange={(next) => panelState.set(next)} variant="panel" showNavigation />
      <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Each step owns the colour of its progress edge.</Text>
    </View>
  );
  // section: Arrows
  const arrowsView = (
    <View style={{ width: 780, maxWidth: "100%", gap: 12 }}>
      <Stepper items={workflow} selectedIndex={arrows()} onChange={(next) => arrows.set(next)} variant="arrows" showNavigation />
      <StepperDetail eyebrow={`Stage ${arrows() + 1} of ${workflow.length}`} title={workflow[arrows()].label} description="Confirm the budget, the decision makers and the timeline." />
    </View>
  );
  // section: Pills
  const pillsView = (
    <View style={{ width: 780, maxWidth: "100%", gap: 12 }}>
      <Stepper items={phases} selectedIndex={pills()} onChange={(next) => pills.set(next)} variant="pills" showNavigation />
      <StepperDetail eyebrow={`Step ${pills() + 1} of ${phases.length}`} title={phases[pills()].label} description="Legal and finance review terms." />
    </View>
  );
  // section: Vertical rail
  const railView = (
    <View style={{ width: 640, maxWidth: "100%", padding: 16, borderWidth: 1, borderColor: tokens.border, borderRadius: 10, backgroundColor: tokens.popover }}>
      <Stepper items={[...workflow.slice(0, 5), { label: "Implementation", description: "Task 3 of 5", meta: "Apply Changes" }]} selectedIndex={rail()} onChange={(next) => rail.set(next)} variant="rail" vertical showNavigation />
    </View>
  );
  // section: end
  return (
    <Page>
      <Caption>Stepper is for a process with a known sequence. Use Progress when completion is measurable, and Tabs when the reader can choose any section.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="With icons">{withIcons}</ExampleSection>
      <ExampleSection index={2} title="Text center">{textCenter}</ExampleSection>
      <ExampleSection index={3} title="Vertical">{vertical}</ExampleSection>
      <ExampleSection index={4} title="Sizes and disabled">{sizes}</ExampleSection>
      <ExampleSection index={5} title="Basic Stepper">{basicStepper}</ExampleSection>
      <ExampleSection index={6} title="Disabled State">{disabled}</ExampleSection>
      <ExampleSection index={7} title="Handle Click Events">{events}</ExampleSection>
      <ExampleSection index={8} title="Multi-step Form">{multiStep}</ExampleSection>
      <ExampleSection index={9} title="Disabled Individual Steps">{individual}</ExampleSection>
      <ExampleSection index={10} title="Cards">{cardsView}</ExampleSection>
      <ExampleSection index={11} title="Dots">{dotsView}</ExampleSection>
      <ExampleSection index={12} title="Panel">{panelView}</ExampleSection>
      <ExampleSection index={13} title="Arrows">{arrowsView}</ExampleSection>
      <ExampleSection index={14} title="Pills">{pillsView}</ExampleSection>
      <ExampleSection index={15} title="Vertical rail">{railView}</ExampleSection>
    </Page>
  );
}
