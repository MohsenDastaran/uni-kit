//! `form` — fields, descriptions, validation and actions.
import { signal } from "nsh";
import { Form, FormActions, FormField, FormSection } from "../components/form";
import { Input } from "../components/input";
import { Button } from "../components/button";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const email = signal("");
  const name = signal("");
  // section: Basic Form
  const basic = (
    <Form>
      <FormField label="Name" required><Input value={name()} placeholder="Ada Lovelace" onChangeText={(next) => name.set(next)} /></FormField>
      <FormField label="Email" required description="We never share it."><Input value={email()} placeholder="ada@example.com" onChangeText={(next) => email.set(next)} /></FormField>
      <FormActions><Button label="Cancel" variant="ghost" onPress={() => {}} /><Button label="Save" variant="primary" onPress={() => {}} /></FormActions>
    </Form>
  );
  // section: Horizontal Form Layout
  const horizontal = <Form layout="horizontal"><FormField label="Name"><Input placeholder="Name" /></FormField><FormField label="Email"><Input placeholder="Email" /></FormField></Form>;
  // section: Multi-Column Form
  const columns = <Form columns={2}><FormField label="First name"><Input placeholder="Ada" /></FormField><FormField label="Last name"><Input placeholder="Lovelace" /></FormField><FormField label="Email"><Input placeholder="ada@example.com" /></FormField></Form>;
  // section: User Registration Form
  const registration = (
    <FormSection title="Create an account" description="All fields are required.">
      <FormField label="Email"><Input placeholder="ada@example.com" /></FormField>
      <FormField label="Password"><Input placeholder="At least 8 characters" masked maskToggle /></FormField>
    </FormSection>
  );
  // section: end
  return (
    <Page>
      <Caption>Form owns the layout and the field metadata; each control owns its own input.</Caption>
      <ExampleSection index={0} title="Basic Form">{basic}</ExampleSection>
      <ExampleSection index={1} title="Horizontal Form Layout">{horizontal}</ExampleSection>
      <ExampleSection index={2} title="Multi-Column Form">{columns}</ExampleSection>
      <ExampleSection index={3} title="User Registration Form">{registration}</ExampleSection>
      <Status>Form state is application state; the controls are controlled.</Status>
    </Page>
  );
}
