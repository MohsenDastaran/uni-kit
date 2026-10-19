//! `focus-trap` — keyboard focus stays inside a region while it is active.
//! Tab and Shift+Tab cycle through the supplied inputs; Escape closes.
import { signal, Text, View, type JSX, type YNode } from "nsh";
import { Button } from "../components/button";
import { FocusTrap } from "../components/focus_trap";
import { Input } from "../components/input";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row, Status } from "./ui";

export function Demo() {
  const usageOpen = signal(false);
  const signInOpen = signal(false);
  const projectOpen = signal(false);
  const toggleOpen = signal(false);
  const lastAction = signal("Nothing submitted yet");
  const name = signal("");
  const email = signal("");
  const password = signal("");
  const project = signal("");
  const slug = signal("");

  // Refs are assigned by the native runtime; FocusTrap uses them to move focus.
  const usageRefs: YNode[] = [];
  const signInRefs: YNode[] = [];
  const projectRefs: YNode[] = [];
  const toggleRefs: YNode[] = [];

  const field = (label: string, input: JSX.Element) => (
    <View style={{ gap: 4, width: "100%" }}>
      <Text style={{ fontSize: 10, fontWeight: "500", color: tokens.mutedForeground }}>{label}</Text>
      {input}
    </View>
  );

  // section: Usage
  const usage = (
    <>
      <Button label="Open trapped field" variant="primary" onPress={() => usageOpen.set(true)} />
      <FocusTrap active={usageOpen()} focusOrder={usageRefs} onEscape={() => usageOpen.set(false)}>
        {usageOpen() ? (
          <View style={{ width: 320, padding: 16, gap: 10, borderRadius: 12, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover }}>
            <Text style={{ fontSize: 14, fontWeight: "600", color: tokens.foreground }}>Focused field</Text>
            <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>Tab is trapped here; Escape closes.</Text>
            <Input ref={(node) => (usageRefs[0] = node)} autoFocus placeholder="Type here" onChangeText={() => {}} />
            <Button label="Close" size="small" variant="ghost" onPress={() => usageOpen.set(false)} />
          </View>
        ) : null}
      </FocusTrap>
    </>
  );

  // section: Basic Focus Trap
  const basic = (
    <>
      <Row gap={8} style={{ alignItems: "center" }}>
        <Button label="Open sign-in" variant="primary" onPress={() => signInOpen.set(true)} />
        <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>Tab / Shift+Tab moves; Esc closes.</Text>
      </Row>
      <FocusTrap active={signInOpen()} focusOrder={signInRefs} onEscape={() => signInOpen.set(false)}>
        {signInOpen() ? (
        <View style={{ width: 360, maxWidth: "100%", padding: 18, gap: 12, borderRadius: 12, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover }}>
          <View style={{ gap: 3 }}>
            <Text style={{ fontSize: 15, fontWeight: "600", color: tokens.foreground }}>Welcome back</Text>
            <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>Focus stays in this dialog while it is open.</Text>
          </View>
          {field("Name", <Input ref={(node) => (signInRefs[0] = node)} value={name()} onChangeText={(next) => name.set(next)} autoFocus placeholder="Ada Lovelace" />)}
          {field("Email", <Input ref={(node) => (signInRefs[1] = node)} value={email()} onChangeText={(next) => email.set(next)} placeholder="ada@example.com" />)}
          {field("Password", <Input ref={(node) => (signInRefs[2] = node)} value={password()} onChangeText={(next) => password.set(next)} placeholder="••••••••" masked />)}
          <Row gap={6} style={{ justifyContent: "flex-end" }}>
            <Button label="Cancel" variant="ghost" size="small" onPress={() => signInOpen.set(false)} />
            <Button label="Sign in" variant="primary" size="small" onPress={() => { lastAction.set("Signed in"); signInOpen.set(false); }} />
          </Row>
        </View>
              ) : null}
</FocusTrap>
    </>
  );

  // section: Focus Trap with Dialog
  const dialog = (
    <>
      <Button label="Edit project" variant="secondary" onPress={() => projectOpen.set(true)} />
      <FocusTrap active={projectOpen()} focusOrder={projectRefs} onEscape={() => projectOpen.set(false)}>
        {projectOpen() ? (
        <View style={{ width: 380, maxWidth: "100%", padding: 18, gap: 12, borderRadius: 12, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover }}>
          <Text style={{ fontSize: 15, fontWeight: "600", color: tokens.foreground }}>Project settings</Text>
          <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>Tab through the fields. The dialog never releases focus to the page behind it.</Text>
          {field("Project name", <Input ref={(node) => (projectRefs[0] = node)} value={project()} onChangeText={(next) => project.set(next)} autoFocus placeholder="Design system" />)}
          {field("Slug", <Input ref={(node) => (projectRefs[1] = node)} value={slug()} onChangeText={(next) => slug.set(next)} placeholder="design-system" />)}
          <Row gap={6} style={{ justifyContent: "flex-end" }}>
            <Button label="Close" variant="ghost" size="small" onPress={() => projectOpen.set(false)} />
            <Button label="Save" variant="primary" size="small" onPress={() => { lastAction.set("Project saved"); projectOpen.set(false); }} />
          </Row>
        </View>
              ) : null}
</FocusTrap>
    </>
  );

  // section: Conditional Focus Trap
  const conditional = (
    <>
      <Row gap={8} style={{ alignItems: "center" }}>
        <Button
          label={toggleOpen() ? "Disable trap" : "Enable trap"}
          variant={toggleOpen() ? "primary" : "secondary"}
          onPress={() => toggleOpen.set(!toggleOpen())}
        />
        <Status>{toggleOpen() ? "Trap active" : "Trap inactive"}</Status>
      </Row>
      <FocusTrap active={toggleOpen()} focusOrder={toggleRefs} onEscape={() => toggleOpen.set(false)}>
        {toggleOpen() ? (
        <View style={{ width: 320, maxWidth: "100%", padding: 18, gap: 10, borderRadius: 12, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover }}>
          <Text style={{ fontSize: 14, fontWeight: "600", color: tokens.foreground }}>Quick search</Text>
          <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>Press Tab to move through the fields, Esc to close the trap.</Text>
          <Input ref={(node) => (toggleRefs[0] = node)} autoFocus placeholder="Search components" onChangeText={() => {}} />
          <Input ref={(node) => (toggleRefs[1] = node)} placeholder="Filter by tag" onChangeText={() => {}} />
          <Button label="Close" size="small" variant="ghost" onPress={() => toggleOpen.set(false)} />
        </View>
              ) : null}
</FocusTrap>
    </>
  );
  // section: end

  return (
    <Page>
      <Caption>
        NSH has no tab-focus API. FocusTrap uses the native modal layer to make the
        rest of the app inert, and a focus order to cycle inputs with Tab and
        Shift+Tab. Escape asks the app to close.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic Focus Trap">{basic}</ExampleSection>
      <ExampleSection index={2} title="Focus Trap with Dialog">{dialog}</ExampleSection>
      <ExampleSection index={3} title="Conditional Focus Trap">{conditional}</ExampleSection>
      <Status>Last action: {lastAction()}</Status>
    </Page>
  );
}
