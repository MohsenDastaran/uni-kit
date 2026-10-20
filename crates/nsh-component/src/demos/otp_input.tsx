//! `otp-input` — one box per character, grouped codes, PINs, numeric and
//! alphanumeric modes, masked values and verification cards.
import { signal, Text, View, type JSX } from "nsh";
import { Button } from "../components/button";
import { OtpInput } from "../components/otp_input";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row, Status } from "./ui";

export function Demo() {
  const code = signal("");
  const defaultCode = signal("123456");
  const masked = signal("123456");
  const alphanumeric = signal("A1B2C3");
  const pin = signal("1234");
  const verify = signal("");
  const twoFactor = signal("");
  const completed = signal("Not submitted");

  const card = (children: JSX.Element, width = 420) => (
    <View style={{ width, maxWidth: "100%", padding: 16, gap: 12, borderRadius: 10, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover }}>
      {children}
    </View>
  );

  // section: Usage
  const usage = (
    <OtpInput value={code()} onChange={(next) => code.set(next)} groups={[3, 3]} />
  );

  // section: Verification
  const verification = card(
    <>
      <View style={{ gap: 3 }}>
        <Text style={{ fontSize: 14, fontWeight: "600", color: tokens.foreground }}>Verify your login</Text>
        <Text style={{ fontSize: 10, lineHeight: 15, color: tokens.mutedForeground }}>
          Enter the verification code we sent to your email address: m@example.com.
        </Text>
      </View>
      <View style={{ flexDirection: "row", alignItems: "center" }}>
        <Text style={{ flex: 1, fontSize: 11, fontWeight: "500", color: tokens.foreground }}>Verification code</Text>
        <Button label="Resend Code" size="xsmall" variant="ghost" icon="refresh-cw" onPress={() => {}} />
      </View>
      <OtpInput value={verify()} onChange={(next) => verify.set(next)} groups={[3, 3]} onComplete={() => completed.set("Verified")} />
      <Button label="Verify" variant="primary" width="full" onPress={() => completed.set("Verify pressed")} />
      <View style={{ flexDirection: "row", justifyContent: "center", gap: 3 }}>
        <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Having trouble signing in?</Text>
        <Text style={{ fontSize: 10, color: tokens.primary }}>Contact support</Text>
      </View>
    </>,
  );

  // section: Basic OTP Input
  const basic = (
    <OtpInput
      value={code()}
      onChange={(next) => code.set(next)}
      groups={[3, 3]}
      onComplete={(next) => completed.set(`Completed ${next}`)}
    />
  );

  // section: Digits Only
  const digits = (
    <OtpInput defaultValue="482913" length={6} />
  );

  // section: Multi-Separator
  const multi = (
    <OtpInput defaultValue="123456" groups={[2, 2, 2]} />
  );

  // section: Alphanumeric OTP
  const alpha = (
    <View style={{ gap: 7 }}>
      <OtpInput value={alphanumeric()} onChange={(next) => alphanumeric.set(next)} type="alphanumeric" groups={[3, 3]} />
      <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Accepts both letters and numbers.</Text>
    </View>
  );

  // section: PIN Entry
  const pinEntry = (
    <View style={{ gap: 7 }}>
      <Text style={{ fontSize: 12, fontWeight: "500", color: tokens.foreground }}>4-Digit PIN</Text>
      <OtpInput value={pin()} onChange={(next) => pin.set(next)} length={4} masked />
      <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Common pattern for PIN codes.</Text>
    </View>
  );

  // section: With Default Value
  const defaulted = (
    <OtpInput value={defaultCode()} onChange={(next) => defaultCode.set(next)} />
  );

  // section: Masked OTP Input
  const maskedInput = (
    <OtpInput value={masked()} onChange={(next) => masked.set(next)} masked />
  );

  // section: Different Sizes
  const sizes = (
    <View style={{ gap: 10, alignItems: "flex-start" }}>
      <OtpInput defaultValue="1234" length={4} size="small" />
      <OtpInput defaultValue="123456" size="large" groups={[3, 3]} />
    </View>
  );

  // section: Grouped Layout
  const grouped = (
    <View style={{ gap: 8 }}>
      <OtpInput defaultValue="123456" groups={[3, 3]} />
      <OtpInput defaultValue="12345678" length={8} groups={[4, 4]} />
    </View>
  );

  // section: Disabled State
  const disabled = (
    <OtpInput defaultValue="123456" disabled groups={[3, 3]} />
  );

  // section: Different Length Codes
  const lengths = (
    <Row gap={14} style={{ alignItems: "flex-start", flexWrap: "wrap" }}>
      <OtpInput defaultValue="1234" length={4} />
      <OtpInput defaultValue="123456" length={6} />
      <OtpInput defaultValue="12345678" length={8} groups={[4, 4]} />
    </Row>
  );

  // section: Handle OTP Events
  const events = (
    <View style={{ gap: 8 }}>
      <OtpInput
        value={code()}
        onChange={(next) => code.set(next)}
        onComplete={(next) => completed.set(`Completed ${next}`)}
        groups={[3, 3]}
      />
      <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>{completed()}</Text>
    </View>
  );

  // section: Programmatic Control
  const programmatic = (
    <View style={{ gap: 8 }}>
      <OtpInput value={code()} onChange={(next) => code.set(next)} groups={[3, 3]} />
      <Row gap={6}>
        <Button label="Fill 123456" size="xsmall" variant="secondary" onPress={() => code.set("123456")} />
        <Button label="Clear" size="xsmall" variant="ghost" onPress={() => code.set("")} />
      </Row>
      <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Code: {code() || "—"}</Text>
    </View>
  );

  // section: Two-Factor Authentication
  const twoFactorCard = card(
    <>
      <Text style={{ fontSize: 14, fontWeight: "600", color: tokens.foreground }}>Two-factor authentication</Text>
      <Text style={{ fontSize: 10, lineHeight: 15, color: tokens.mutedForeground }}>Enter the 6-digit code from your authenticator app.</Text>
      <OtpInput value={twoFactor()} onChange={(next) => twoFactor.set(next)} groups={[3, 3]} />
      <Row gap={6}>
        <Button label="Cancel" size="small" variant="ghost" onPress={() => {}} />
        <Button label="Continue" size="small" variant="primary" onPress={() => completed.set("2FA continued")} />
      </Row>
    </>,
  );

  // section: Resend code timer
  const resend = (
    <View style={{ gap: 8 }}>
      <OtpInput value={code()} onChange={(next) => code.set(next)} groups={[3, 3]} />
      <Row gap={6}>
        <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Didn't get the code?</Text>
        <Button label="Resend in 24s" size="xsmall" variant="ghost" disabled onPress={() => {}} />
      </Row>
    </View>
  );

  // section: Custom styling
  const custom = (
    <View style={{ gap: 8 }}>
      <OtpInput value="123456" status="error" groups={[3, 3]} />
      <Text style={{ fontSize: 9, color: tokens.danger }}>That code doesn't match. Check the latest email.</Text>
    </View>
  );
  // section: end

  return (
    <Page>
      <Caption>
        One-time codes use one native input per character, so focus follows the
        code and every group keeps its own separator.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Verification">{verification}</ExampleSection>
      <ExampleSection index={2} title="Basic OTP Input">{basic}</ExampleSection>
      <ExampleSection index={3} title="Digits Only">{digits}</ExampleSection>
      <ExampleSection index={4} title="Multi-Separator">{multi}</ExampleSection>
      <ExampleSection index={5} title="Alphanumeric OTP">{alpha}</ExampleSection>
      <ExampleSection index={6} title="PIN Entry">{pinEntry}</ExampleSection>
      <ExampleSection index={7} title="With Default Value">{defaulted}</ExampleSection>
      <ExampleSection index={8} title="Masked OTP Input">{maskedInput}</ExampleSection>
      <ExampleSection index={9} title="Different Sizes">{sizes}</ExampleSection>
      <ExampleSection index={10} title="Grouped Layout">{grouped}</ExampleSection>
      <ExampleSection index={11} title="Disabled State">{disabled}</ExampleSection>
      <ExampleSection index={12} title="Different Length Codes">{lengths}</ExampleSection>
      <ExampleSection index={13} title="Handle OTP Events">{events}</ExampleSection>
      <ExampleSection index={14} title="Programmatic Control">{programmatic}</ExampleSection>
      <ExampleSection index={15} title="Two-Factor Authentication">{twoFactorCard}</ExampleSection>
      <ExampleSection index={16} title="Resend code timer">{resend}</ExampleSection>
      <ExampleSection index={17} title="Custom styling">{custom}</ExampleSection>
      <Status>{completed()} · {code() ? code() : "empty"}</Status>
    </Page>
  );
}
