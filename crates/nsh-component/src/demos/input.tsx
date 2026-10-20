//! `input` — a single-line field with prefix/suffix slots, clear, password,
//! sizes, validation and the invalid/disabled states.
//!
//! The `// section:` markers are the website's contract: one per heading on the
//! input page, in order, closed by `// section: end`.
import { signal, Text, View } from "nsh";
import { Input } from "../components/input";
import { Button } from "../components/button";
import { Select } from "../components/select";
import { Icon } from "../components/icon";
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
  const apiKey = signal("9f82afc4b7e243d8a6c9f1e2a3b4c5d6");
  const middle = signal("Alexander");
  const secure = signal("");
  const security = signal("");
  const pulse = signal("");
  const newsletter = signal("");
  const loginEmail = signal("");
  const loginPassword = signal("");
  const cardName = signal("");
  const cardNumber = signal("");
  const expiry = signal("");
  const cvc = signal("");
  const inviteEmail = signal("");
  const inviteRole = signal("editor");
  const profileName = signal("Ada Lovelace");
  const profileCompany = signal("Analytical Engines");
  const stage = (children: any, width = 380) => (
    <View style={{ width, maxWidth: "100%" }}>{children}</View>
  );
  const score = () => [secure().length >= 8, /[0-9]/.test(secure()), /[a-z]/.test(secure()), /[A-Z]/.test(secure()), /[^A-Za-z0-9]/.test(secure())].filter(Boolean).length;

  // section: Usage
  const usage = (
    <View style={{ gap: 8, width: 420, maxWidth: "100%" }}>
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

  // section: Newsletter signup
  const newsletterCard = (
    <View style={{ width: 440, maxWidth: "100%", padding: 16, gap: 10, borderRadius: RADIUS_LG, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover }}>
      <View style={{ gap: 2 }}>
        <Text style={{ fontSize: 14, fontWeight: "600", color: tokens.foreground }}>Stay in the loop</Text>
        <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>Product updates, once a month. No noise.</Text>
      </View>
      <Row gap={8} style={{ alignItems: "center" }}>
        <Input value={newsletter()} placeholder="you@example.com" prefixIcon="mail" onChangeText={(next) => newsletter.set(next)} style={{ flex: 1 }} />
        <Button label="Subscribe" variant="primary" onPress={() => {}} />
      </Row>
      <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Unsubscribe at any time.</Text>
    </View>
  );

  // section: Search with filter
  const searchFilter = (
    <View style={{ width: 480, maxWidth: "100%", padding: 12, gap: 8, borderRadius: RADIUS_LG, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover }}>
      <Row gap={8} style={{ alignItems: "center" }}>
        <Input value={prefixed()} placeholder="Search components…" prefixIcon="search" cleanable onChangeText={(next) => prefixed.set(next)} style={{ flex: 1 }} />
        <Select options={[{ value: "all", label: "All" }, { value: "components", label: "Components" }, { value: "patterns", label: "Patterns" }]} value={filter().toLowerCase()} onChange={(next) => filter.set(next === "all" ? "All" : next === "components" ? "Components" : "Patterns")} style={{ width: 140 }} />
      </Row>
      <Row gap={6}>
        <Text style={{ flex: 1, fontSize: 10, color: tokens.mutedForeground }}>Showing results in {filter()}</Text>
        <Button label="Filters" size="xsmall" variant="ghost" icon="funnel" onPress={() => {}} />
      </Row>
    </View>
  );

  // section: Login form
  const login = (
    <View style={{ width: 380, maxWidth: "100%", padding: 18, gap: 12, borderRadius: RADIUS_LG, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover }}>
      <View style={{ gap: 2 }}>
        <Text style={{ fontSize: 15, fontWeight: "600", color: tokens.foreground }}>Welcome back</Text>
        <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>Sign in to continue to your workspace.</Text>
      </View>
      <Input label="Email" value={loginEmail()} placeholder="name@example.com" prefixIcon="mail" onChangeText={(next) => loginEmail.set(next)} />
      <Input label="Password" labelAction={<Text style={{ fontSize: 10, color: tokens.primary }}>Forgot?</Text>} value={loginPassword()} placeholder="Enter your password" masked maskToggle onChangeText={(next) => loginPassword.set(next)} />
      <Button label="Sign in" variant="primary" style={{ width: "100%" }} onPress={() => {}} />
    </View>
  );

  // section: Payment details
  const payment = (
    <View style={{ width: 440, maxWidth: "100%", padding: 16, gap: 11, borderRadius: RADIUS_LG, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover }}>
      <View style={{ flexDirection: "row", alignItems: "center" }}>
        <Text style={{ flex: 1, fontSize: 14, fontWeight: "600", color: tokens.foreground }}>Payment details</Text>
        <Icon name="lock" size={13} color={tokens.mutedForeground} />
      </View>
      <Input label="Name on card" value={cardName()} placeholder="Ada Lovelace" onChangeText={(next) => cardName.set(next)} />
      <Input label="Card number" value={cardNumber()} placeholder="4242 4242 4242 4242" prefixIcon="hash" onChangeText={(next) => cardNumber.set(next)} />
      <Row gap={10}>
        <Input label="Expiry" value={expiry()} placeholder="MM / YY" onChangeText={(next) => expiry.set(next)} style={{ width: 140 }} />
        <Input label="CVC" value={cvc()} placeholder="123" onChangeText={(next) => cvc.set(next)} style={{ width: 100 }} />
      </Row>
      <Button label="Save card" variant="primary" style={{ width: "100%" }} onPress={() => {}} />
    </View>
  );

  // section: Invite teammate
  const invite = (
    <View style={{ width: 440, maxWidth: "100%", padding: 16, gap: 11, borderRadius: RADIUS_LG, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover }}>
      <View style={{ gap: 2 }}>
        <Text style={{ fontSize: 14, fontWeight: "600", color: tokens.foreground }}>Invite a teammate</Text>
        <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>They will get an email with a link to join.</Text>
      </View>
      <Input label="Email" value={inviteEmail()} placeholder="teammate@example.com" prefixIcon="mail" onChangeText={(next) => inviteEmail.set(next)} />
      <View style={{ flexDirection: "row", gap: 8, alignItems: "flex-end" }}>
        <Select options={[{ value: "viewer", label: "Viewer" }, { value: "editor", label: "Editor" }, { value: "admin", label: "Admin" }]} value={inviteRole()} onChange={(next) => inviteRole.set(next)} style={{ width: 150 }} />
        <Button label="Send invite" variant="primary" onPress={() => {}} style={{ flex: 1 }} />
      </View>
    </View>
  );

  // section: Profile settings
  const profile = (
    <View style={{ width: 460, maxWidth: "100%", padding: 16, gap: 12, borderRadius: RADIUS_LG, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover }}>
      <Text style={{ fontSize: 14, fontWeight: "600", color: tokens.foreground }}>Profile</Text>
      <Row gap={10}>
        <Input label="Full name" value={profileName()} onChangeText={(next) => profileName.set(next)} style={{ flex: 1 }} />
        <Input label="Company" value={profileCompany()} onChangeText={(next) => profileCompany.set(next)} style={{ flex: 1 }} />
      </Row>
      <Input label="Work email" value={loginEmail()} placeholder="name@company.com" prefixIcon="mail" description="Used for receipts and invites." onChangeText={(next) => loginEmail.set(next)} />
      <Row gap={8} style={{ justifyContent: "flex-end" }}>
        <Button label="Cancel" variant="ghost" size="small" onPress={() => {}} />
        <Button label="Save changes" variant="primary" size="small" onPress={() => {}} />
      </Row>
    </View>
  );

  // section: Basic
  const simple = (
    <Input
      value={basic()}
      style={{ width: 380, maxWidth: "100%" }}
      placeholder="Basic input"
      onChangeText={(next) => basic.set(next)}
      cleanable
    />
  );

  // section: Prefix, suffix and clear button
  const decorated = (
    <View style={{ gap: 10, width: 420, maxWidth: "100%" }}>
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
      style={{ width: 380, maxWidth: "100%" }}
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
    <View style={{ gap: 8, width: 380, maxWidth: "100%" }}>
      <Input value={small()} placeholder="Extra small" size="xsmall" onChangeText={(next) => small.set(next)} />
      <Input value={small()} placeholder="Small" size="small" onChangeText={(next) => small.set(next)} />
      <Input value={small()} placeholder="Medium" size="medium" onChangeText={(next) => small.set(next)} />
      <Input value={small()} placeholder="Large" size="large" onChangeText={(next) => small.set(next)} />
    </View>
  );

  // section: States
  const states = (
    <View style={{ gap: 8, width: 420, maxWidth: "100%" }}>
      <Input value="A normal value" onChangeText={() => {}} />
      <Input value="An invalid value" invalid onChangeText={() => {}} />
      <Input value="A disabled value" disabled onChangeText={() => {}} />
      <Input value="A read-only value" readOnly onChangeText={() => {}} />
    </View>
  );

  // section: Disabled Input
  const disabled = (
    <Input value="This field cannot be edited" disabled onChangeText={() => {}} style={{ width: 380, maxWidth: "100%" }} />
  );

  // section: Input Validation
  const validation = (
    <View style={{ gap: 8, width: 380, maxWidth: "100%" }}>
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

  // section: Input label with badge
  const labelBadge = (
    <Input
      label="API Key"
      badge="New"
      value={apiKey()}
      onChangeText={(next) => apiKey.set(next)}
      style={{ width: 360, maxWidth: "100%" }}
    />
  );

  // section: Input label with optional badge
  const labelOptional = (
    <Input
      label="Middle Name"
      optional
      value={middle()}
      onChangeText={(next) => middle.set(next)}
      placeholder="Alexander"
      style={{ width: 360, maxWidth: "100%" }}
    />
  );

  // section: Link and visibility toggle
  const labelAction = (
    <Input
      label="Password"
      labelAction={<Text style={{ fontSize: 10, color: tokens.primary }}>Forgot password?</Text>}
      value={password()}
      onChangeText={(next) => password.set(next)}
      placeholder="Enter your password"
      masked
      maskToggle
      style={{ width: 360, maxWidth: "100%" }}
    />
  );

  // section: Input with horizontal orientation
  const horizontal = (
    <Input
      label="Name"
      orientation="horizontal"
      labelWidth={70}
      value="John Doe"
      onChangeText={() => {}}
      style={{ width: 380, maxWidth: "100%" }}
    />
  );

  // section: Simple error messages
  const errorMessages = (
    <View style={{ width: 360, maxWidth: "100%", gap: 7 }}>
      <Input
        label="Security Code"
        invalid
        value={security()}
        onChangeText={(next) => security.set(next)}
        placeholder="Enter your security code"
      />
      <View style={{ gap: 3 }}>
        {[
          "Code must be at least 12 characters long",
          "Code must contain at least one uppercase letter",
          "Code cannot include common words or patterns",
        ].map((line) => (
          <Row gap={4} style={{ alignItems: "center" }}>
            <Text style={{ fontSize: 10, color: tokens.danger }}>●</Text>
            <Text style={{ fontSize: 10, lineHeight: 14, color: tokens.danger }}>{line}</Text>
          </Row>
        ))}
      </View>
    </View>
  );

  // section: Password strength
  const strength = (
    <View style={{ width: 380, maxWidth: "100%", gap: 8 }}>
      <Input
        label="Secure Password"
        value={secure()}
        onChangeText={(next) => secure.set(next)}
        placeholder="Create a strong password"
        masked
        maskToggle
      />
      <View style={{ flexDirection: "row", gap: 4 }}>
        {[0, 1, 2, 3].map((index) => (
          <View style={{ flex: 1, height: 3, borderRadius: 2, backgroundColor: index < Math.min(4, Math.ceil(score() * 0.8)) ? (score() >= 4 ? tokens.success : tokens.warning) : tokens.muted }} />
        ))}
      </View>
      <View style={{ flexDirection: "row", alignItems: "center" }}>
        <Text style={{ flex: 1, fontSize: 11, fontWeight: "500", color: tokens.foreground }}>Enter a password</Text>
        <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{score()}/5 requirements met</Text>
      </View>
      <View style={{ gap: 3 }}>
        {[
          { label: "At least 8 characters", met: secure().length >= 8 },
          { label: "At least 1 number", met: /[0-9]/.test(secure()) },
          { label: "At least 1 lowercase letter", met: /[a-z]/.test(secure()) },
          { label: "At least 1 uppercase letter", met: /[A-Z]/.test(secure()) },
          { label: "At least 1 special character", met: /[^A-Za-z0-9]/.test(secure()) },
        ].map((item) => (
          <Row gap={5} style={{ alignItems: "center" }}>
            <Text style={{ fontSize: 10, color: item.met ? tokens.success : tokens.danger }}>{item.met ? "✓" : "✕"}</Text>
            <Text style={{ fontSize: 10, color: item.met ? tokens.mutedForeground : tokens.danger }}>{item.label}</Text>
          </Row>
        ))}
      </View>
    </View>
  );

  // section: Pulsing background
  const pulsing = (
    <Input
      label="Pulsing Background"
      pulse
      value={pulse()}
      onChangeText={(next) => pulse.set(next)}
      placeholder="Animation stops on focus…"
      style={{ width: 360, maxWidth: "100%" }}
    />
  );

  // section: Input types
  const types = (
    <View style={{ width: 380, maxWidth: "100%", gap: 10 }}>
      <Input label="Email" keyboardType="email-address" placeholder="name@example.com" prefixIcon="mail" />
      <Input label="URL" keyboardType="url" placeholder="https://example.com" prefixIcon="globe" />
      <Input label="Number" keyboardType="numeric" placeholder="123" />
    </View>
  );

  // section: end

  return (
    <Page>
      <Caption>
        The native field keeps typing, the caret, selection and the software keyboard; this
        component draws the frame and the adornments around it.
      </Caption>
      <ExampleSection index={6} title="Usage">
        {usage}
      </ExampleSection>
      <ExampleSection index={1} title="Newsletter signup">
        {newsletterCard}
      </ExampleSection>
      <ExampleSection index={2} title="Search with filter">
        {searchFilter}
      </ExampleSection>
      <ExampleSection index={3} title="Login form">
        {login}
      </ExampleSection>
      <ExampleSection index={4} title="Payment details">
        {payment}
      </ExampleSection>
      <ExampleSection index={5} title="Invite teammate">
        {invite}
      </ExampleSection>
      <ExampleSection index={6} title="Profile settings">
        {profile}
      </ExampleSection>
      <ExampleSection index={7} title="Basic">
        {simple}
      </ExampleSection>
      <ExampleSection index={8} title="Prefix, suffix and clear button">
        {decorated}
      </ExampleSection>
      <ExampleSection index={9} title="Password">
        {secret}
      </ExampleSection>
      <ExampleSection index={10} title="Sizes">
        {sizes}
      </ExampleSection>
      <ExampleSection index={11} title="States">
        {states}
      </ExampleSection>
      <ExampleSection index={12} title="Disabled Input">
        {disabled}
      </ExampleSection>
      <ExampleSection index={13} title="Input Validation">
        {validation}
      </ExampleSection>
      <ExampleSection index={14} title="Clean on ESC">
        {cleanOnEsc}
      </ExampleSection>
      <ExampleSection index={15} title="Handle Input Events">
        {handled}
      </ExampleSection>
      <ExampleSection index={16} title="Custom Appearance">
        {custom}
      </ExampleSection>
      <ExampleSection index={17} title="Atomic inline tokens">
        {inline}
      </ExampleSection>
      <ExampleSection index={18} title="Search Input">
        {search}
      </ExampleSection>
      <ExampleSection index={19} title="Currency Input">
        {currency}
      </ExampleSection>
      <ExampleSection index={20} title="Form with Multiple Inputs">
        {form}
      </ExampleSection>
      <ExampleSection index={21} title="Input label with badge">{labelBadge}</ExampleSection>
      <ExampleSection index={22} title="Input label with optional badge">{labelOptional}</ExampleSection>
      <ExampleSection index={23} title="Link and visibility toggle">{labelAction}</ExampleSection>
      <ExampleSection index={24} title="Input with horizontal orientation">{horizontal}</ExampleSection>
      <ExampleSection index={25} title="Simple error messages">{errorMessages}</ExampleSection>
      <ExampleSection index={26} title="Password strength">{strength}</ExampleSection>
      <ExampleSection index={27} title="Pulsing background">{pulsing}</ExampleSection>
      <ExampleSection index={28} title="Input types">{types}</ExampleSection>
      <Status>
        <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>{events()}</Text>
      </Status>
    </Page>
  );
}
