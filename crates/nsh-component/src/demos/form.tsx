//! `form` — cards, sections, stacked and inline fields, validation and actions.
import { signal, Text, View } from "nsh";
import { Button } from "../components/button";
import { Checkbox } from "../components/checkbox";
import { Form, FormActions, FormField, FormHint, FormRow, FormSection } from "../components/form";
import { Icon } from "../components/icon";
import { Input } from "../components/input";
import { Select } from "../components/select";
import { Switch } from "../components/switch";
import { Textarea } from "../components/textarea";
import { alpha, tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

const fieldStyle = { flex: 1, minWidth: 150 } as const;
const controlStyle = { width: "100%" } as const;

export function Demo() {
  const name = signal("Cedar & Volt Studio LLC");
  const email = signal("");
  const city = signal("Portland");
  const state = signal("OR");
  const postal = signal("97205");
  const description = signal("Digital workspace templates, implementation support, and design system services.");
  const sampleData = signal(true);
  const errors = signal(false);

  // section: Usage
  const usage = (
    <Form
      card
      title="Edit profile"
      description="Update the details shown on your public profile."
      style={{ width: 520, maxWidth: "100%" }}
      footer={
        <>
          <FormHint>Changes are visible immediately.</FormHint>
          <Button label="Cancel" variant="ghost" size="small" onPress={() => {}} />
          <Button label="Save changes" variant="primary" size="small" onPress={() => {}} />
        </>
      }
    >
      <FormField label="Full name" required>
        <Input value={name()} onChangeText={(next) => name.set(next)} style={controlStyle} />
      </FormField>
      <FormField label="Email" description="Used for account notifications.">
        <Input value={email()} placeholder="you@example.com" onChangeText={(next) => email.set(next)} style={controlStyle} />
      </FormField>
    </Form>
  );

  // section: Basic Form
  const basic = (
    <Form
      card
      title="Get started"
      description="A single-column form with descriptions and a submit row."
      style={{ width: 460, maxWidth: "100%" }}
      footer={
        <>
          <FormHint>All fields are required.</FormHint>
          <Button label="Create account" variant="primary" size="small" onPress={() => {}} />
        </>
      }
    >
      <FormField label="Name" required>
        <Input placeholder="Ada Lovelace" style={controlStyle} />
      </FormField>
      <FormField label="Email" required description="We never share it.">
        <Input placeholder="ada@example.com" style={controlStyle} />
      </FormField>
      <FormField label="Password" required>
        <Input placeholder="At least 8 characters" masked maskToggle style={controlStyle} />
      </FormField>
    </Form>
  );

  // section: Horizontal Form Layout
  const horizontal = (
    <Form
      card
      title="Notification settings"
      description="Labels sit in a fixed column beside each control."
      style={{ width: 560, maxWidth: "100%" }}
    >
      <FormField layout="inline" labelWidth={150} label="Product updates">
        <Switch checked={sampleData()} onChange={(next) => sampleData.set(next)} />
      </FormField>
      <FormField layout="inline" labelWidth={150} label="Weekly digest" description="A summary every Monday morning.">
        <Switch defaultChecked />
      </FormField>
      <FormField layout="inline" labelWidth={150} label="Support email">
        <Input placeholder="support@example.com" style={controlStyle} />
      </FormField>
    </Form>
  );

  // section: Multi-Column Form
  const columns = (
    <Form
      card
      title="Shipping address"
      description="Two-column rows collapse into one on narrow layouts."
      style={{ width: 560, maxWidth: "100%" }}
      footer={
        <>
          <FormHint>We only ship within the United States.</FormHint>
          <Button label="Continue" variant="primary" size="small" onPress={() => {}} />
        </>
      }
    >
      <FormRow>
        <FormField label="First name" style={fieldStyle}><Input placeholder="Ada" style={controlStyle} /></FormField>
        <FormField label="Last name" style={fieldStyle}><Input placeholder="Lovelace" style={controlStyle} /></FormField>
      </FormRow>
      <FormField label="Street address"><Input placeholder="418 Market Street" style={controlStyle} /></FormField>
      <FormRow>
        <FormField label="City" style={fieldStyle}><Input value={city()} onChangeText={(next) => city.set(next)} style={controlStyle} /></FormField>
        <FormField label="State" style={{ width: 100, flexShrink: 0 }}><Input value={state()} onChangeText={(next) => state.set(next)} style={controlStyle} /></FormField>
        <FormField label="Postal code" style={{ width: 130, flexShrink: 0 }}><Input value={postal()} onChangeText={(next) => postal.set(next)} style={controlStyle} /></FormField>
      </FormRow>
    </Form>
  );

  // section: User Registration Form
  const registration = (
    <Form
      card
      title="Create an account"
      description="Validation messages appear under the field they belong to."
      style={{ width: 460, maxWidth: "100%" }}
      footer={
        <>
          <FormHint>{errors() ? "Fix the highlighted field to continue." : "By continuing you agree to the terms."}</FormHint>
          <Button label="Register" variant="primary" size="small" onPress={() => errors.set(true)} />
        </>
      }
    >
      <FormField label="Email" required error={errors() ? "Enter a valid email address." : undefined}>
        <Input placeholder="ada@example.com" invalid={errors()} style={controlStyle} />
      </FormField>
      <FormField label="Password" required description="At least 8 characters, including a number.">
        <Input placeholder="••••••••" masked maskToggle style={controlStyle} />
      </FormField>
      <Checkbox label="I agree to the terms of service" description="You can change this later in settings." />
    </Form>
  );

  // section: Settings Form with Sections
  const settings = (
    <Form
      card
      title="New App"
      description="Create a launch-ready app from a brief, source, and starter."
      style={{ width: 640, maxWidth: "100%" }}
      footer={
        <>
          <FormHint>You can change all of this later.</FormHint>
          <Button label="Cancel" variant="ghost" size="small" onPress={() => {}} />
          <Button icon="plus" label="Create App" variant="primary" size="small" onPress={() => {}} />
        </>
      }
    >
      <View style={{ padding: 12, borderRadius: 10, backgroundColor: alpha(tokens.primary, 0.12), flexDirection: "row", alignItems: "center", gap: 12 }}>
        <View style={{ width: 36, height: 36, borderRadius: 8, alignItems: "center", justifyContent: "center", backgroundColor: alpha(tokens.primary, 0.2) }}>
          <Icon name="rocket" size={17} color={tokens.primary} />
        </View>
        <View style={{ flex: 1, gap: 2 }}>
          <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Launch setup</Text>
          <Text style={{ fontSize: 15, fontWeight: "600", color: tokens.foreground }}>Response Hub</Text>
          <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Incident queues, escalation notes, and executive…</Text>
        </View>
        <View style={{ gap: 3 }}>
          <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Source · meadow-relay</Text>
          <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Runtime · Next.js App</Text>
          <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Region · US East</Text>
        </View>
      </View>
      <FormField label="Launch Brief" description="Describe the first version.">
        <Textarea value="Create a service desk app with priority queues, SLA badges, and owner handoff notes." rows={3} style={controlStyle} onChangeText={() => {}} />
      </FormField>
      <FormRow>
        <FormField label="Source" description="Repository to connect." style={fieldStyle}>
          <Select value="meadow-relay" options={[{ value: "meadow-relay", label: "meadow-relay" }, { value: "design-system", label: "design-system" }]} onChange={() => {}} style={controlStyle} />
        </FormField>
        <FormField label="Runtime" description="Where the app executes." style={fieldStyle}>
          <Select value="nextjs" options={[{ value: "nextjs", label: "Next.js App" }, { value: "node", label: "Node service" }]} onChange={() => {}} style={controlStyle} />
        </FormField>
      </FormRow>
      <FormRow>
        <FormField label="App Name" description="Display name for the app." style={fieldStyle}><Input defaultValue="Meadow Relay" style={controlStyle} /></FormField>
        <FormField label="Region" description="Deployment location." style={fieldStyle}><Select value="us-east" options={[{ value: "us-east", label: "US East" }, { value: "eu-west", label: "EU West" }]} onChange={() => {}} style={controlStyle} /></FormField>
      </FormRow>
    </Form>
  );

  // section: Contact Form
  const contact = (
    <Form
      card
      title="Contact us"
      description="We usually reply within one business day."
      style={{ width: 560, maxWidth: "100%" }}
      footer={
        <>
          <FormHint>Your message goes to the support team.</FormHint>
          <Button label="Send message" variant="primary" size="small" onPress={() => {}} />
        </>
      }
    >
      <FormRow>
        <FormField label="Name" style={fieldStyle}><Input placeholder="Ada Lovelace" style={controlStyle} /></FormField>
        <FormField label="Email" style={fieldStyle}><Input placeholder="ada@example.com" style={controlStyle} /></FormField>
      </FormRow>
      <FormField label="Topic">
        <Select value="billing" options={[{ value: "billing", label: "Billing" }, { value: "technical", label: "Technical support" }, { value: "other", label: "Other" }]} onChange={() => {}} style={controlStyle} />
      </FormField>
      <FormField label="Message" description="Include steps to reproduce if this is a bug.">
        <Textarea placeholder="How can we help?" rows={4} style={controlStyle} onChangeText={() => {}} />
      </FormField>
    </Form>
  );

  // section: Business Details
  const business = (
    <Form
      card
      title="Edit Business Details"
      description="Update the legal profile used for account review."
      style={{ width: 620, maxWidth: "100%" }}
      footer={
        <>
          <FormHint>Shown to reviewers when the business profile is checked.</FormHint>
          <Button label="Cancel" variant="ghost" size="small" onPress={() => {}} />
          <Button icon="check" label="Save changes" variant="primary" size="small" onPress={() => {}} />
        </>
      }
    >
      <FormField label="Legal name" help><Input value={name()} onChangeText={(next) => name.set(next)} style={controlStyle} /></FormField>
      <FormRow>
        <FormField label="Business type" help style={fieldStyle}><Select value="llc" options={[{ value: "llc", label: "Limited liability company" }, { value: "sole", label: "Sole proprietorship" }]} onChange={() => {}} style={controlStyle} /></FormField>
        <FormField label="Industry" help style={fieldStyle}><Select value="digital" options={[{ value: "digital", label: "Digital services" }, { value: "retail", label: "Retail" }]} onChange={() => {}} style={controlStyle} /></FormField>
      </FormRow>
      <FormField label="Street address"><Input defaultValue="418 Market Street, Floor 6" style={controlStyle} /></FormField>
      <FormRow>
        <FormField label="City" style={fieldStyle}><Input value={city()} onChangeText={(next) => city.set(next)} style={controlStyle} /></FormField>
        <FormField label="State" style={{ width: 90, flexShrink: 0 }}><Input value={state()} onChangeText={(next) => state.set(next)} style={controlStyle} /></FormField>
        <FormField label="Postal code" style={{ width: 120, flexShrink: 0 }}><Input value={postal()} onChangeText={(next) => postal.set(next)} style={controlStyle} /></FormField>
      </FormRow>
      <FormField label="Product description" help>
        <Textarea value={description()} onChangeText={(next) => description.set(next)} rows={3} style={controlStyle} />
      </FormField>
    </Form>
  );

  // section: Checkout Setup
  const checkout = (
    <Form
      card
      title="Checkout Setup"
      description="Hosted checkout settings."
      style={{ width: 620, maxWidth: "100%" }}
      footer={
        <>
          <FormHint>Draft stays private.</FormHint>
          <Button icon="save" label="Save draft" variant="ghost" size="small" onPress={() => {}} />
          <Button icon="check" label="Publish checkout" variant="primary" size="small" onPress={() => {}} />
        </>
      }
    >
      <FormSection title="Offers and customer details" divider>
        <FormField layout="inline" labelWidth={150} label="Offer Type">
          <Select value="bundle" options={[{ value: "bundle", label: "Product bundle" }, { value: "single", label: "Single product" }]} onChange={() => {}} style={controlStyle} />
        </FormField>
        <FormField layout="inline" labelWidth={150} label="Offer">
          <Select value="growth" options={[{ value: "growth", label: "Growth seat bundle" }, { value: "starter", label: "Starter bundle" }]} onChange={() => {}} style={controlStyle} />
        </FormField>
        <FormField layout="inline" labelWidth={150} label="Customer Details">
          <View style={{ gap: 6 }}>
            <Checkbox label="Collect customer name" defaultChecked />
            <Checkbox label="Ask for company name" />
            <Checkbox label="Collect delivery address" defaultChecked />
          </View>
        </FormField>
      </FormSection>
      <FormSection title="Checkout rules" divider>
        <FormField layout="inline" labelWidth={150} label="Rules">
          <View style={{ gap: 6 }}>
            <Checkbox label="Run managed authorization" description="Authorize before fulfilment." defaultChecked />
            <Checkbox label="Limit completed purchases" />
            <Checkbox label="Require phone verification" />
          </View>
        </FormField>
        <FormField layout="inline" labelWidth={150} label="Button Copy">
          <Select value="access" options={[{ value: "access", label: "Reserve access" }, { value: "buy", label: "Buy now" }]} onChange={() => {}} style={controlStyle} />
        </FormField>
        <FormField layout="inline" labelWidth={150} label="Return URL">
          <Input defaultValue="https://northstar.studio/welcome" style={controlStyle} />
        </FormField>
      </FormSection>
    </Form>
  );
  // section: end

  return (
    <Page>
      <Caption>
        Form owns the card, section rhythm, label placement and feedback; each control
        remains its own component. Cards keep long forms readable and grouped.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic Form">{basic}</ExampleSection>
      <ExampleSection index={2} title="Horizontal Form Layout">{horizontal}</ExampleSection>
      <ExampleSection index={3} title="Multi-Column Form">{columns}</ExampleSection>
      <ExampleSection index={4} title="User Registration Form">{registration}</ExampleSection>
      <ExampleSection index={5} title="Settings Form with Sections">{settings}</ExampleSection>
      <ExampleSection index={6} title="Contact Form">{contact}</ExampleSection>
      <ExampleSection index={7} title="Business Details">{business}</ExampleSection>
      <ExampleSection index={8} title="Checkout Setup">{checkout}</ExampleSection>
    </Page>
  );
}
