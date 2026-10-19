//! `combobox` — an input with a filtered, inline option panel (no modal).
import { signal, Text } from "nsh";
import { Avatar } from "../components/avatar";
import { Badge } from "../components/badge";
import { Button } from "../components/button";
import { Combobox, type ComboboxOption } from "../components/combobox";
import { tokens } from "../components/theme";
import { Caption, Column, ExampleSection, Page, Row } from "./ui";

const components: ComboboxOption[] = [
  { value: "button", label: "Button", description: "A press target" },
  { value: "input", label: "Input", description: "A text field" },
  { value: "card", label: "Card", description: "A content surface" },
  { value: "dialog", label: "Dialog", description: "A modal layer" },
];

const frameworks: ComboboxOption[] = [
  { value: "next", label: "Next.js" },
  { value: "nuxt", label: "Nuxt.js" },
  { value: "svelte", label: "SvelteKit" },
  { value: "remix", label: "Remix" },
  { value: "astro", label: "Astro" },
];

const members: ComboboxOption[] = [
  { value: "alex", label: "Alex Johnson", description: "Software Engineer" },
  { value: "sarah", label: "Sarah Chen", description: "Product Manager" },
  { value: "michael", label: "Michael Rodriguez", description: "UX Designer" },
  { value: "emma", label: "Emma Wilson", description: "Technical Lead" },
  { value: "david", label: "David Kim", description: "CTO" },
].map((item) => ({ ...item, leading: <Avatar name={item.label} size="xs" /> }));

const priorities: ComboboxOption[] = [
  { value: "none", label: "No Priority", icon: "ellipsis", tone: tokens.mutedForeground },
  { value: "low", label: "Low", icon: "chart-column", tone: tokens.mutedForeground },
  { value: "medium", label: "Medium", icon: "chart-column", tone: tokens.foreground },
  { value: "high", label: "High", icon: "chart-column", tone: tokens.foreground },
  { value: "urgent", label: "Urgent", icon: "triangle-alert", tone: tokens.warning },
];

const dates: ComboboxOption[] = [
  { value: "none", label: "No date", icon: "calendar" },
  { value: "tomorrow", label: "Tomorrow", icon: "calendar", note: "20/10/2026" },
  { value: "week", label: "End of week", icon: "calendar", note: "25/10/2026" },
  { value: "one-week", label: "In one week", icon: "calendar", note: "26/10/2026" },
  { value: "month", label: "End of month", icon: "calendar", note: "31/10/2026" },
  { value: "one-month", label: "In one month", icon: "calendar", note: "19/11/2026" },
  { value: "custom", label: "Custom date…", icon: "calendar" },
];

const labels: ComboboxOption[] = [
  { value: "bug", label: "Bug", icon: "circle", tone: tokens.danger },
  { value: "design", label: "Design", icon: "circle", tone: tokens.info },
  { value: "docs", label: "Docs", icon: "circle", tone: tokens.success },
  { value: "feature", label: "Feature", icon: "circle", tone: tokens.warning },
];

export function Demo() {
  const single = signal("button");
  const multi = signal(["next", "nuxt"]);
  const member = signal("alex");
  const priority = signal("medium");
  const date = signal("tomorrow");
  const label = signal("bug");
  const team = signal(["alex", "sarah"]);
  const maxed = signal(["next", "nuxt", "svelte"]);

  // section: Usage
  const usage = (
    <Combobox
      options={components}
      value={single()}
      onChange={(next) => single.set(next)}
      width={320}
    />
  );

  // section: Basic Single-Select
  const basic = (
    <Combobox
      options={components}
      value={single()}
      onChange={(next) => single.set(next)}
      width={320}
    />
  );

  // section: Multi-Select
  const multiple = (
    <Combobox
      multiple
      options={frameworks}
      values={multi()}
      onValuesChange={(next) => multi.set(next)}
      placeholder="Select frameworks…"
      searchPlaceholder="Search frameworks…"
      width={340}
    />
  );

  // section: Pre-selected Item
  const preselected = (
    <Combobox options={components} value="input" onChange={() => {}} width={320} />
  );

  // section: Grouped Items
  const grouped = (
    <Combobox
      options={[
        { value: "button", label: "Button", description: "A press target", group: "Controls" },
        { value: "input", label: "Input", description: "A text field", group: "Controls" },
        { value: "switch", label: "Switch", description: "A binary toggle", group: "Controls" },
        { value: "card", label: "Card", description: "A content surface", group: "Surfaces" },
        { value: "dialog", label: "Dialog", description: "A modal layer", group: "Surfaces" },
      ]}
      value={single()}
      onChange={(next) => single.set(next)}
      width={320}
    />
  );

  // section: Disabled Items
  const disabled = (
    <Combobox
      options={[...components, { value: "legacy", label: "Legacy", description: "Read-only", disabled: true }]}
      value="card"
      onChange={() => {}}
      width={320}
    />
  );

  // section: Custom Check Icon
  const customCheck = (
    <Combobox
      options={[components[0], components[2], components[3]]}
      multiple
      values={["button", "card"]}
      onValuesChange={() => {}}
      width={320}
    />
  );

  // section: Footer Action
  const footer = (
    <Combobox
      options={components.slice(0, 2)}
      placeholder="Choose a component"
      width={320}
      footer={
        <Button
          icon="plus"
          label="New component"
          size="small"
          variant="ghost"
          style={{ justifyContent: "flex-start", width: "100%" }}
          onPress={() => {}}
        />
      }
    />
  );

  // section: Custom Trigger
  const custom = (
    <Combobox
      options={members}
      value={member()}
      onChange={(next) => member.set(next)}
      searchPlaceholder="Search members…"
      width={340}
    />
  );

  // section: Icons
  const icons = (
    <Combobox
      options={priorities}
      value={priority()}
      onChange={(next) => priority.set(next)}
      placeholder="Change priority"
      searchPlaceholder="Change priority"
      width={300}
    />
  );

  // section: Badges
  const badges = (
    <Combobox
      options={frameworks.map((option, index) => ({
        ...option,
        badge: index < 2 ? "POPULAR" : undefined,
      }))}
      multiple
      values={multi()}
      onValuesChange={(next) => multi.set(next)}
      placeholder="Select frameworks…"
      searchPlaceholder="Search frameworks…"
      width={340}
    />
  );

  // section: Maximum selections
  const maximum = (
    <Column gap={6} style={{ width: 340 }}>
      <Combobox
        multiple
        maxSelected={3}
        options={frameworks}
        values={maxed()}
        onValuesChange={(next) => maxed.set(next)}
        placeholder="Up to three"
        searchPlaceholder="Search frameworks…"
        width={340}
      />
      <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>
        {maxed().length} of 3 selected
      </Text>
    </Column>
  );

  // section: Pinned items
  const pinned = (
    <Combobox
      options={[
        { value: "alex", label: "Alex Johnson", description: "Software Engineer" },
        { value: "sarah", label: "Sarah Chen", description: "Product Manager" },
        { value: "michael", label: "Michael Rodriguez", description: "UX Designer" },
      ]}
      value="alex"
      onChange={() => {}}
      placeholder="Choose a member"
      width={340}
    />
  );

  // section: Rich items
  const rich = (
    <Combobox
      options={members}
      multiple
      values={multi().slice(0, 1).map(() => "alex")}
      onValuesChange={() => {}}
      placeholder="Select reviewers"
      searchPlaceholder="Search members…"
      width={360}
    />
  );

  // section: Overflow
  const overflow = (
    <Combobox
      options={[labels[0], labels[1], labels[3]]}
      value={label()}
      onChange={(next) => label.set(next)}
      placeholder="Set label"
      width={300}
    />
  );

  // section: Count
  const count = (
    <Combobox
      options={members}
      multiple
      values={team()}
      onValuesChange={(next) => team.set(next)}
      placeholder="Assign members"
      searchPlaceholder="Search members…"
      width={340}
    />
  );

  // section: Values
  const values = (
    <Column gap={6} style={{ width: 320 }}>
      <Combobox options={components} value={single()} onChange={(next) => single.set(next)} width={320} />
      <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Selected value: {single()}</Text>
    </Column>
  );

  // section: Sizes
  const sizes = (
    <Row gap={10} style={{ alignItems: "flex-start" }}>
      <Combobox options={components} defaultValue="button" onChange={() => {}} width={240} />
      <Combobox options={components} defaultValue="card" onChange={() => {}} width={280} />
    </Row>
  );

  // section: Cleanable
  const cleanable = (
    <Combobox
      options={components}
      value={single()}
      onChange={(next) => single.set(next)}
      placeholder="Choose a component"
      width={320}
    />
  );

  // section: Disabled
  const disabledTrigger = (
    <Combobox options={components} value="button" onChange={() => {}} disabled width={320} />
  );

  // section: Events
  const events = (
    <Column gap={6} style={{ width: 320 }}>
      <Combobox
        options={components}
        value={single()}
        onChange={(next) => single.set(next)}
        width={320}
      />
      <Badge label={`onChange: ${single()}`} variant="secondary" />
    </Column>
  );

  // section: Mutating Programmatically
  const mutate = (
    <Column gap={8} style={{ width: 320 }}>
      <Combobox options={components} value={single()} onChange={(next) => single.set(next)} width={320} />
      <Row gap={6}>
        <Button label="Select Input" size="small" variant="ghost" onPress={() => single.set("input")} />
        <Button label="Select Card" size="small" variant="ghost" onPress={() => single.set("card")} />
      </Row>
    </Column>
  );

  // section: Member picker
  const memberPicker = (
    <Combobox
      options={members}
      multiple
      values={team()}
      onValuesChange={(next) => team.set(next)}
      placeholder="Assign members"
      searchPlaceholder="Search members…"
      width={340}
      footer={
        <Text style={{ paddingHorizontal: 6, paddingVertical: 4, fontSize: 10, color: tokens.mutedForeground }}>
          {team().length} member{team().length === 1 ? "" : "s"} assigned
        </Text>
      }
    />
  );

  // section: Date presets
  const datePresets = (
    <Combobox
      options={dates}
      value={date()}
      onChange={(next) => date.set(next)}
      placeholder="Set date"
      width={320}
      footer={
        <Text style={{ paddingHorizontal: 6, paddingVertical: 4, fontSize: 10, color: tokens.mutedForeground }}>
          A plain list, no calendar popover.
        </Text>
      }
    />
  );
  // section: end

  return (
    <Page>
      <Caption>
        The option list is an inline panel under the trigger, not a modal: it filters as you
        type and leaves the page around it visible.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic Single-Select">{basic}</ExampleSection>
      <ExampleSection index={2} title="Multi-Select">{multiple}</ExampleSection>
      <ExampleSection index={3} title="Pre-selected Item">{preselected}</ExampleSection>
      <ExampleSection index={4} title="Grouped Items">{grouped}</ExampleSection>
      <ExampleSection index={5} title="Disabled Items">{disabled}</ExampleSection>
      <ExampleSection index={6} title="Custom Check Icon">{customCheck}</ExampleSection>
      <ExampleSection index={7} title="Footer Action">{footer}</ExampleSection>
      <ExampleSection index={8} title="Custom Trigger">{custom}</ExampleSection>
      <ExampleSection index={9} title="Icons">{icons}</ExampleSection>
      <ExampleSection index={10} title="Badges">{badges}</ExampleSection>
      <ExampleSection index={11} title="Maximum selections">{maximum}</ExampleSection>
      <ExampleSection index={12} title="Pinned items">{pinned}</ExampleSection>
      <ExampleSection index={13} title="Rich items">{rich}</ExampleSection>
      <ExampleSection index={14} title="Overflow">{overflow}</ExampleSection>
      <ExampleSection index={15} title="Count">{count}</ExampleSection>
      <ExampleSection index={16} title="Values">{values}</ExampleSection>
      <ExampleSection index={17} title="Sizes">{sizes}</ExampleSection>
      <ExampleSection index={18} title="Cleanable">{cleanable}</ExampleSection>
      <ExampleSection index={19} title="Disabled">{disabledTrigger}</ExampleSection>
      <ExampleSection index={20} title="Events">{events}</ExampleSection>
      <ExampleSection index={21} title="Mutating Programmatically">{mutate}</ExampleSection>
      <ExampleSection index={22} title="Member picker">{memberPicker}</ExampleSection>
      <ExampleSection index={23} title="Date presets">{datePresets}</ExampleSection>
    </Page>
  );
}
