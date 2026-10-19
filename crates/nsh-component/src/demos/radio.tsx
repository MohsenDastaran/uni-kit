//! `radio` — one choice among several, from plain rows to card and grid pickers.
import { signal, View, type JSX } from "nsh";
import { Avatar } from "../components/avatar";
import { Radio, RadioGroup } from "../components/radio";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

const comfort = [
  { value: "default", label: "Default" },
  { value: "comfortable", label: "Comfortable" },
  { value: "compact", label: "Compact" },
];

export function Demo() {
  const value = signal("comfortable");
  const shipping = signal("standard");
  const language = signal("english");
  const plan = signal("pro");
  const iconChoice = signal("payments");
  const settingsChoice = signal("dark");
  const descriptionChoice = signal("comfortable");
  const invalidChoice = signal("");
  const coloredChoice = signal("blue");
  const batteryChoice = signal("medium");
  const planChoice = signal("plus");
  const contactChoice = signal("email");
  const badgeChoice = signal("stable");
  const personChoice = signal("emma");
  const customChoice = signal("daily");
  const choose = (next: string) => value.set(next);
  const stage = (children: JSX.Element, width = 420) => (
    <View style={{ width, maxWidth: "100%", padding: 16, borderWidth: 1, borderColor: tokens.border, borderRadius: 12, backgroundColor: tokens.popover }}>
      {children}
    </View>
  );
  // section: Usage
  const usage = stage(<RadioGroup options={comfort} value={value()} onChange={choose} />);
  // section: Group
  const group = stage(<RadioGroup options={comfort} value={value()} onChange={choose} />);
  // section: Standalone
  const standalone = <Radio label="One option" selected />;
  // section: Different Sizes
  const sizes = stage(
    <View style={{ gap: 14 }}>
      <RadioGroup options={comfort} value={value()} onChange={choose} size="small" />
      <RadioGroup options={comfort} value={value()} onChange={choose} />
      <RadioGroup options={comfort} value={value()} onChange={choose} size="large" />
    </View>,
  );
  // section: Disabled State
  const disabled = stage(<RadioGroup options={[{ value: "a", label: "Default" }, { value: "b", label: "Comfortable", disabled: true }, { value: "c", label: "Compact" }]} value="a" disabled />);
  // section: Settings Panel
  const settings = stage(
    <RadioGroup
      options={[{ value: "light", label: "Light" }, { value: "dark", label: "Dark" }, { value: "system", label: "System" }]}
      value={settingsChoice()}
      onChange={(next) => settingsChoice.set(next)}
      orientation="horizontal"
    />,
  );
  // section: Radio with description
  const descriptions = stage(
    <RadioGroup
        options={[
          { value: "default", label: "Default", description: "Standard spacing for most use cases." },
          { value: "comfortable", label: "Comfortable", description: "More space between elements." },
          { value: "compact", label: "Compact", description: "Minimal spacing for dense layouts." },
        ]}
        value={descriptionChoice()}
        onChange={(next) => descriptionChoice.set(next)}
      />,
    460,
  );
  // section: Invalid radio group
  const invalid = stage(
    <RadioGroup
        options={[{ value: "email", label: "Email only" }, { value: "sms", label: "SMS only" }]}
        value={invalidChoice()}
        onChange={(next) => invalidChoice.set(next)}
        invalid={!invalidChoice()}
        error="Choose where we should send the code."
      />,
  );
  // section: Colored radio group
  const colored = stage(
    <RadioGroup
        options={[
          { value: "blue", label: "Blue", tone: "info" },
          { value: "green", label: "Green", tone: "success" },
          { value: "yellow", label: "Yellow", tone: "warning" },
        ]}
        value={coloredChoice()}
        onChange={(next) => coloredChoice.set(next)}
      />,
  );
  // section: Radio group with legend and description
  const battery = (
    <View style={{ width: 440, maxWidth: "100%" }}>
      <RadioGroup
        legend="Battery Level"
        description="Choose your preferred battery level."
        options={[{ value: "high", label: "High" }, { value: "medium", label: "Medium" }, { value: "low", label: "Low" }]}
        value={batteryChoice()}
        onChange={(next) => batteryChoice.set(next)}
        variant="card"
        controlPosition="end"
      />
    </View>
  );
  // section: Card radio group with descriptions
  const plans = (
    <View style={{ width: 440, maxWidth: "100%" }}>
      <RadioGroup
        options={[
          { value: "plus", label: "Plus", description: "For individuals and small teams." },
          { value: "pro", label: "Pro", description: "For growing businesses." },
        ]}
        value={planChoice()}
        onChange={(next) => planChoice.set(next)}
        variant="card"
        controlPosition="end"
      />
    </View>
  );
  // section: Radio group in card with separators
  const delivery = (
    <View style={{ width: 420, maxWidth: "100%" }}>
      <RadioGroup
        options={[
          { value: "standard", label: "Standard Shipping" },
          { value: "express", label: "Express Shipping" },
          { value: "overnight", label: "Overnight Shipping" },
        ]}
        value={shipping()}
        onChange={(next) => shipping.set(next)}
        variant="segmented"
      />
    </View>
  );
  // section: Radio group in card with icons
  const contact = (
    <View style={{ width: 360, maxWidth: "100%" }}>
      <RadioGroup
        options={[
          { value: "email", label: "Email", icon: "mail" },
          { value: "phone", label: "Phone", icon: "phone" },
          { value: "chat", label: "Chat", icon: "message-circle" },
        ]}
        value={contactChoice()}
        onChange={(next) => contactChoice.set(next)}
        variant="card"
        controlPosition="end"
      />
    </View>
  );
  // section: Radio group in frame
  const languageGroup = (
    <View style={{ width: 320, maxWidth: "100%" }}>
      <RadioGroup
        legend="Language"
        options={[
          { value: "english", label: "English" },
          { value: "spanish", label: "Spanish" },
          { value: "french", label: "French" },
          { value: "german", label: "German" },
        ]}
        value={language()}
        onChange={(next) => language.set(next)}
        variant="segmented"
      />
    </View>
  );
  // section: Card radio group with icons
  const products = (
    <View style={{ width: 460, maxWidth: "100%" }}>
      <RadioGroup
        columns={2}
        variant="grid"
        options={[
          { value: "payments", label: "Payments", description: "Receive payments from your customers", icon: "layers" },
          { value: "invoices", label: "Invoices", description: "Create and send invoices to your customers", icon: "file-text" },
          { value: "billing", label: "Billing", description: "Manage subscriptions and billing", icon: "table" },
          { value: "reports", label: "Reports", description: "Track revenue and growth", icon: "chart-column" },
        ]}
        value={iconChoice()}
        onChange={(next) => iconChoice.set(next)}
      />
    </View>
  );
  // section: Radio group with badges
  const badges = stage(
    <RadioGroup
        options={[
          { value: "stable", label: "Stable", badge: "RECOMMENDED", badgeTone: "info" },
          { value: "beta", label: "Beta", badge: "NEW", badgeTone: "info" },
          { value: "canary", label: "Canary" },
        ]}
        value={badgeChoice()}
        onChange={(next) => badgeChoice.set(next)}
      />,
  );
  // section: Pricing radio group
  const pricing = (
    <View style={{ width: 440, maxWidth: "100%" }}>
      <RadioGroup
        variant="card"
        controlPosition="end"
        options={[
          { value: "free", label: "Free", description: "For personal projects and experiments.", meta: "$0/mo" },
          { value: "pro", label: "Pro", description: "For professionals and small teams.", meta: "$19/mo" },
          { value: "enterprise", label: "Enterprise", description: "For organizations with advanced needs.", meta: "$49/mo" },
        ]}
        value={plan()}
        onChange={(next) => plan.set(next)}
      />
    </View>
  );
  // section: Radio group with avatars
  const people = (
    <View style={{ width: 400, maxWidth: "100%" }}>
      <RadioGroup
        variant="card"
        controlPosition="end"
        options={[
          { value: "emma", label: "Emma Wilson", description: "@emmawilson", leading: <Avatar name="Emma Wilson" size="sm" /> },
          { value: "john", label: "John Doe", description: "@johndoe", leading: <Avatar name="John Doe" size="sm" /> },
        ]}
        value={personChoice()}
        onChange={(next) => personChoice.set(next)}
      />
    </View>
  );
  // section: Horizontal Layout
  const horizontal = (
    <RadioGroup options={comfort} value={value()} onChange={choose} orientation="horizontal" />
  );
  // section: Custom styling
  const custom = (
    <View style={{ width: 420, maxWidth: "100%" }}>
      <RadioGroup
        legend="Notification cadence"
        description="Choose how often we should email you."
        options={[
          { value: "instant", label: "Instant" },
          { value: "daily", label: "Daily digest" },
          { value: "weekly", label: "Weekly digest" },
        ]}
        value={customChoice()}
        onChange={(next) => customChoice.set(next)}
        variant="segmented"
      />
    </View>
  );
  // section: end
  return (
    <Page>
      <Caption>Radio buttons are for exactly one choice; a checkbox list is for independent choices.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Group">{group}</ExampleSection>
      <ExampleSection index={2} title="Standalone">{standalone}</ExampleSection>
      <ExampleSection index={3} title="Different Sizes">{sizes}</ExampleSection>
      <ExampleSection index={4} title="Disabled State">{disabled}</ExampleSection>
      <ExampleSection index={5} title="Settings Panel">{settings}</ExampleSection>
      <ExampleSection index={6} title="Radio with description">{descriptions}</ExampleSection>
      <ExampleSection index={7} title="Invalid radio group">{invalid}</ExampleSection>
      <ExampleSection index={8} title="Colored radio group">{colored}</ExampleSection>
      <ExampleSection index={9} title="Radio group with legend and description">{battery}</ExampleSection>
      <ExampleSection index={10} title="Card radio group with descriptions">{plans}</ExampleSection>
      <ExampleSection index={11} title="Radio group in card with separators">{delivery}</ExampleSection>
      <ExampleSection index={12} title="Radio group in card with icons">{contact}</ExampleSection>
      <ExampleSection index={13} title="Radio group in frame">{languageGroup}</ExampleSection>
      <ExampleSection index={14} title="Card radio group with icons">{products}</ExampleSection>
      <ExampleSection index={15} title="Radio group with badges">{badges}</ExampleSection>
      <ExampleSection index={16} title="Pricing radio group">{pricing}</ExampleSection>
      <ExampleSection index={17} title="Radio group with avatars">{people}</ExampleSection>
      <ExampleSection index={18} title="Horizontal Layout">{horizontal}</ExampleSection>
      <ExampleSection index={19} title="Custom styling">{custom}</ExampleSection>
    </Page>
  );
}
