//! `input` — a single-line field with prefix/suffix slots, clear, password,
//! sizes, validation and the invalid/disabled states.
//!
//! The `// section:` markers are the website's contract: one per heading on the
//! input page, in order, closed by `// section: end`.
import { signal, Text, View } from "nsh";
import { Input } from "../components/input";
import { Button } from "../components/button";
import { alpha, RADIUS_LG, RADIUS_MD, tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row, Status } from "./ui";

export function Demo() {
  const basic = signal("");
  const prefixed = signal("");
  const password = signal("");
  const small = signal("");
  const events = signal("No event yet");
  const escaped = signal("");
  const amount = signal("");
  const token = signal("");
  const email = signal("");
  const filter = signal("Components");

  // section: Usage
  const usage = (
    <View style={{ gap: 8 }}>
      <Input
        value={basic()}
        placeholder="Type something…"
        onChangeText={(next) => basic.set(next)}
        onSubmit={(text) => events.set(`Submitted: ${text}`)}
        cleanable
      />
      <Caption>Value: {basic() || "empty"}</Caption>
    </View>
  );

  // section: Basic
  const simple = (
    <Input
      value={basic()}
      placeholder="Basic input"
      onChangeText={(next) => basic.set(next)}
      cleanable
    />
  );

  // section: Prefix, suffix and clear button
  const decorated = (
    <View style={{ gap: 10 }}>
      <Input
        value={prefixed()}
        placeholder="Search"
        prefixIcon="search"
        suffixIcon="command"
        cleanable
        onChangeText={(next) => prefixed.set(next)}
      />
      <Input
        value={amount()}
        placeholder="0.00"
        prefixText="$"
        suffixText="USD"
        horizontalAlignment="right"
        onChangeText={(next) => amount.set(next)}
      />
    </View>
  );

  // section: Password
  const secret = (
    <Input
      value={password()}
      placeholder="Password"
      prefixIcon="lock"
      masked
      maskToggle
      cleanable
      onChangeText={(next) => password.set(next)}
    />
  );

  // section: Sizes
  const sizes = (
    <View style={{ gap: 8 }}>
      <Input value={small()} placeholder="Extra small" size="xsmall" onChangeText={(next) => small.set(next)} />
      <Input value={small()} placeholder="Small" size="small" onChangeText={(next) => small.set(next)} />
      <Input value={small()} placeholder="Medium" size="medium" onChangeText={(next) => small.set(next)} />
      <Input value={small()} placeholder="Large" size="large" onChangeText={(next) => small.set(next)} />
    </View>
  );

  // section: States
  const states = (
    <View style={{ gap: 8 }}>
      <Input value="A normal value" onChangeText={() => {}} />
      <Input value="An invalid value" invalid onChangeText={() => {}} />
      <Input value="A disabled value" disabled onChangeText={() => {}} />
      <Input value="A read-only value" readOnly onChangeText={() => {}} />
    </View>
  );

  // section: Disabled Input
  const disabled = (
    <Input value="This field cannot be edited" disabled onChangeText={() => {}} />
  );

  // section: Input Validation
  const validation = (
    <View style={{ gap: 8 }}>
      <Input
        value={email()}
        placeholder="name@example.com"
        prefixIcon="mail"
        invalid={email() !== "" && !email().includes("@")}
        onChangeText={(next) => email.set(next)}
      />
      <Caption>
        {email() === ""
          ? "Type an address."
          : email().includes("@")
            ? "The address looks valid."
            : "An address needs an @."}
      </Caption>
    </View>
  );

  // section: Clean on ESC
  const cleanOnEsc = (
    <Input
      value={escaped()}
      placeholder="Press Escape to clear"
      cleanable
      cleanOnEscape
      onChangeText={(next) => escaped.set(next)}
    />
  );

  // section: Handle Input Events
  const handled = (
    <View style={{ gap: 8 }}>
      <Input
        value={token()}
        placeholder="Focus and type"
        onChangeText={(text) => {
          token.set(text);
          events.set(`Changed: ${text}`);
        }}
        onFocus={() => events.set("Focused")}
        onBlur={() => events.set("Blurred")}
        onSubmit={(text) => events.set(`Submitted: ${text}`)}
      />
      <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>{events()}</Text>
    </View>
  );

  // section: Custom Appearance
  const custom = (
    <View
      style={{
        borderWidth: 1,
        borderColor: tokens.ring,
        borderRadius: RADIUS_MD,
        paddingHorizontal: 10,
        backgroundColor: tokens.muted,
      }}
    >
      <Input
        value={basic()}
        placeholder="A field inside another frame"
        appearance={false}
        onChangeText={(next) => basic.set(next)}
      />
    </View>
  );

  // section: Atomic inline tokens
  const inline = (
    <Row gap={8}>
      <Input
        value={amount()}
        placeholder="0.00"
        prefixText="$"
        suffixText="USD"
        horizontalAlignment="right"
        onChangeText={(next) => amount.set(next)}
        style={{ width: 180 }}
      />
      <Input
        value={token()}
        placeholder="token"
        prefixIcon="hash"
        onChangeText={(next) => token.set(next)}
        style={{ width: 180 }}
      />
    </Row>
  );
  // section: Search Input
  const search = (
    <View
      style={{
        padding: 14,
        gap: 10,
        borderRadius: RADIUS_LG,
        borderWidth: 1,
        borderColor: alpha(tokens.primary, 0.12),
        backgroundColor: alpha(tokens.muted, 0.35),
      }}
    >
      <Input
        value={prefixed()}
        placeholder="Search components…"
        prefixIcon="search"
        suffixText="⌘ K"
        cleanable
        size="large"
        onChangeText={(next) => prefixed.set(next)}
        style={{ borderRadius: RADIUS_LG }}
      />
      <Row gap={6}>
        {["All", "Components", "Patterns"].map((label) => (
          <Button
            label={label}
            size="small"
            variant={filter() === label ? "primary" : "ghost"}
            onPress={() => filter.set(label)}
          />
        ))}
      </Row>
    </View>
  );

  // section: Currency Input
  const currency = (
    <View style={{ gap: 10 }}>
      <Input
        value={amount()}
        placeholder="0.00"
        prefixText="$"
        suffixText="USD"
        horizontalAlignment="right"
        size="large"
        onChangeText={(next) => amount.set(next)}
      />
      <Row gap={6}>
        {["25", "50", "100"].map((value) => (
          <Button label={`$${value}`} size="small" variant="secondary" onPress={() => amount.set(value)} />
        ))}
      </Row>
    </View>
  );

  // section: Form with Multiple Inputs
  const form = (
    <View
      style={{
        padding: 16,
        gap: 12,
        borderRadius: RADIUS_LG,
        borderWidth: 1,
        borderColor: tokens.border,
        backgroundColor: tokens.background,
      }}
    >
      <View style={{ flexDirection: "row", gap: 10 }}>
        <Input value={basic()} placeholder="First name" onChangeText={(next) => basic.set(next)} style={{ flex: 1 }} />
        <Input value={email()} placeholder="Email" prefixIcon="mail" invalid={email() !== "" && !email().includes("@")} onChangeText={(next) => email.set(next)} style={{ flex: 1 }} />
      </View>
      <Row gap={8} style={{ justifyContent: "space-between" }}>
        <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>We only use this for the receipt.</Text>
        <Button label="Create account" variant="primary" icon="arrow-right" onPress={() => {}} />
      </Row>
    </View>
  );

  // section: end

  return (
    <Page>
      <Caption>
        The native field keeps typing, the caret, selection and the software keyboard; this
        component draws the frame and the adornments around it.
      </Caption>
      <ExampleSection index={0} title="Usage">
        {usage}
      </ExampleSection>
      <ExampleSection index={1} title="Basic">
        {simple}
      </ExampleSection>
      <ExampleSection index={2} title="Prefix, suffix and clear button">
        {decorated}
      </ExampleSection>
      <ExampleSection index={3} title="Password">
        {secret}
      </ExampleSection>
      <ExampleSection index={4} title="Sizes">
        {sizes}
      </ExampleSection>
      <ExampleSection index={5} title="States">
        {states}
      </ExampleSection>
      <ExampleSection index={6} title="Disabled Input">
        {disabled}
      </ExampleSection>
      <ExampleSection index={7} title="Input Validation">
        {validation}
      </ExampleSection>
      <ExampleSection index={8} title="Clean on ESC">
        {cleanOnEsc}
      </ExampleSection>
      <ExampleSection index={9} title="Handle Input Events">
        {handled}
      </ExampleSection>
      <ExampleSection index={10} title="Custom Appearance">
        {custom}
      </ExampleSection>
      <ExampleSection index={11} title="Atomic inline tokens">
        {inline}
      </ExampleSection>
      <ExampleSection index={12} title="Search Input">
        {search}
      </ExampleSection>
      <ExampleSection index={13} title="Currency Input">
        {currency}
      </ExampleSection>
      <ExampleSection index={14} title="Form with Multiple Inputs">
        {form}
      </ExampleSection>
      <Status>
        <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>{events()}</Text>
      </Status>
    </Page>
  );
}
