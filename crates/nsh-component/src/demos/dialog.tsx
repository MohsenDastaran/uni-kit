//! `dialog` — dialogs over the page, with a soft enter/exit on the panel.
import { Scroll, signal, Text, View } from "nsh";
import { Dialog } from "../components/dialog";
import { Button } from "../components/button";
import { Input } from "../components/input";
import { Switch } from "../components/switch";
import { Icon } from "../components/icon";
import { tokens } from "../components/theme";
import { Caption, Column, ExampleSection, Page, Row } from "./ui";

const paragraphs = [
  "Your workspace keeps every draft, comment and attachment in one place, and the history is written to the audit log as you work.",
  "Members of this workspace can review the change before it is published. Reviewers can leave comments on any line, and the author is notified.",
  "Once two reviewers approve, the change is published to the shared environment. Rolling back keeps the published version available for fourteen days.",
  "Exports include the audit trail, the reviewer names, and the timestamps of each step, so compliance can read the record without opening the app.",
];

export function Demo() {
  const basicOpen = signal(false);
  const scrollOpen = signal(false);
  const stickyOpen = signal(false);
  const noCloseOpen = signal(false);
  const customCloseOpen = signal(false);
  const fullOpen = signal(false);
  const formOpen = signal(false);
  const iconOpen = signal(false);
  const nestedOpen = signal(false);
  const nestedInner = signal(false);
  const cookiesOpen = signal(false);
  const name = signal("");
  const essential = signal(true);
  const analytics = signal(true);
  const marketing = signal(false);
  const lastAction = signal("No action yet");

  // section: Usage
  const usage = (
    <>
      <Button label="Open dialog" variant="primary" onPress={() => basicOpen.set(true)} />
      <Dialog
        open={basicOpen()}
        title="Edit profile"
        description="Make a change and save."
        onClose={() => basicOpen.set(false)}
        footer={
          <>
            <Button label="Cancel" variant="ghost" onPress={() => basicOpen.set(false)} />
            <Button label="Save" onPress={() => basicOpen.set(false)} />
          </>
        }
      >
        <Input placeholder="Name" onChangeText={() => {}} />
      </Dialog>
    </>
  );

  // section: Basic dialog
  const basic = (
    <>
      <Button label="Open a basic dialog" onPress={() => basicOpen.set(true)} />
      <Dialog
        open={basicOpen()}
        title="Basic dialog"
        description="A title, a description and no body. The panel fades in and settles 8pt higher."
        onClose={() => basicOpen.set(false)}
        footer={<Button label="Close" onPress={() => basicOpen.set(false)} />}
      />
    </>
  );

  // section: Scrollable Dialog
  const scroll = (
    <>
      <Button label="Open scrollable dialog" onPress={() => scrollOpen.set(true)} />
      <Dialog
        open={scrollOpen()}
        title="Change summary"
        description="Read the whole change before you publish it."
        onClose={() => scrollOpen.set(false)}
        footer={
          <>
            <Button label="Cancel" variant="ghost" onPress={() => scrollOpen.set(false)} />
            <Button label="Publish" onPress={() => scrollOpen.set(false)} />
          </>
        }
      >
        <Scroll style={{ maxHeight: 240 }}><View style={{ gap: 10 }}>
          {paragraphs.map((paragraph) => (
            <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
              {paragraph}
            </Text>
          ))}
          {paragraphs.map((paragraph) => (
            <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
              {paragraph}
            </Text>
          ))}
          </View></Scroll>
      </Dialog>
    </>
  );

  // section: Action Buttons
  const sticky = (
    <>
      <Button label="Open sticky footer" onPress={() => stickyOpen.set(true)} />
      <Dialog
        open={stickyOpen()}
        size="large"
        title="Review change"
        description="The footer stays with the dialog while the body scrolls."
        onClose={() => stickyOpen.set(false)}
        footer={
          <>
            <Button label="Request changes" variant="ghost" onPress={() => stickyOpen.set(false)} />
            <Button label="Approve" onPress={() => stickyOpen.set(false)} />
          </>
        }
      >
        <Scroll style={{ maxHeight: 220 }}><View style={{ gap: 10 }}>
          {paragraphs.map((paragraph) => (
            <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>{paragraph}</Text>
          ))}
          </View></Scroll>
      </Dialog>
    </>
  );

  // section: Dialog Options
  const noClose = (
    <>
      <Button label="No close button" onPress={() => noCloseOpen.set(true)} />
      <Dialog
        open={noCloseOpen()}
        title="Delete workspace"
        description="This cannot be undone. Type the workspace name to confirm."
        showClose={false}
        onClose={() => noCloseOpen.set(false)}
        footer={
          <>
            <Button label="Cancel" variant="ghost" onPress={() => noCloseOpen.set(false)} />
            <Button label="Delete" variant="danger" onPress={() => noCloseOpen.set(false)} />
          </>
        }
      >
        <Input placeholder="workspace-name" onChangeText={() => {}} />
      </Dialog>
    </>
  );

  // section: Custom close button
  const customClose = (
    <>
      <Button label="Custom close button" onPress={() => customCloseOpen.set(true)} />
      <Dialog
        open={customCloseOpen()}
        title="Cookie preferences"
        description="A custom close control sits in the header."
        showClose={false}
        onClose={() => customCloseOpen.set(false)}
        footer={<Button label="Save preferences" onPress={() => customCloseOpen.set(false)} />}
      >
        <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
          Choose which categories the site may use. Essential cookies are always on.
        </Text>
      </Dialog>
    </>
  );

  // section: Custom Styling
  const full = (
    <>
      <Button label="Full-screen dialog" onPress={() => fullOpen.set(true)} />
      <Dialog
        open={fullOpen()}
        size="large"
        title="Full-screen fluid dialog"
        description="A wide panel for editing long content."
        onClose={() => fullOpen.set(false)}
        style={{ width: 900, height: 560 }}
        footer={
          <>
            <Button label="Discard" variant="ghost" onPress={() => fullOpen.set(false)} />
            <Button label="Save draft" onPress={() => fullOpen.set(false)} />
          </>
        }
      >
        <View style={{ flex: 1, gap: 10 }}>
          <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
            The body takes the remaining height; the header and footer stay put.
          </Text>
          <View
            style={{
              flex: 1,
              borderWidth: 1,
              borderColor: tokens.border,
              borderRadius: 8,
              backgroundColor: tokens.muted,
            }}
          />
        </View>
      </Dialog>
    </>
  );

  // section: Form dialog
  const form = (
    <>
      <Button label="Open form dialog" onPress={() => formOpen.set(true)} />
      <Dialog
        open={formOpen()}
        title="Create project"
        description="Give the project a name; you can rename it later."
        onClose={() => formOpen.set(false)}
        footer={
          <>
            <Button label="Cancel" variant="ghost" onPress={() => formOpen.set(false)} />
            <Button
              label="Create"
              onPress={() => {
                formOpen.set(false);
                lastAction.set(`Created ${name() || "Untitled"}`);
              }}
            />
          </>
        }
      >
        <Column gap={8}>
          <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>Name</Text>
          <Input value={name()} placeholder="Design system" onChangeText={(next) => name.set(next)} />
        </Column>
      </Dialog>
    </>
  );

  // section: Dialog with icon
  const icon = (
    <>
      <Button label="Open dialog with icon" onPress={() => iconOpen.set(true)} />
      <Dialog
        open={iconOpen()}
        icon="circle-check"
        tone="success"
        title="Deployment complete"
        description="Version 3.4.2 is live in every region."
        onClose={() => iconOpen.set(false)}
        footer={<Button label="View logs" onPress={() => iconOpen.set(false)} />}
      />
    </>
  );

  // section: Nested dialogs
  const nested = (
    <>
      <Button label="Open nested dialogs" onPress={() => nestedOpen.set(true)} />
      <Dialog
        open={nestedOpen()}
        title="Invite a teammate"
        description="Send an invitation to the workspace."
        onClose={() => nestedOpen.set(false)}
        footer={
          <Button
            label="Choose a role"
            onPress={() => {
              nestedOpen.set(false);
              nestedInner.set(true);
            }}
          />
        }
      />
      <Dialog
        open={nestedInner()}
        title="Choose a role"
        description="The second dialog opens above the page."
        onClose={() => nestedInner.set(false)}
        footer={<Button label="Done" onPress={() => nestedInner.set(false)} />}
      />
    </>
  );

  // section: Custom Padding
  const cookies = (
    <>
      <Button label="Open cookie preferences" onPress={() => cookiesOpen.set(true)} />
      <Dialog
        open={cookiesOpen()}
        title="Cookie Preferences"
        description="You can enable or disable different categories of cookies."
        onClose={() => cookiesOpen.set(false)}
        footer={
          <>
            <Button label="Save Preferences" variant="ghost" onPress={() => cookiesOpen.set(false)} />
            <Button label="Accept All" onPress={() => cookiesOpen.set(false)} />
          </>
        }
      >
        <Column gap={12}>
          <Row gap={16} style={{ alignItems: "flex-start" }}>
            <Column gap={2} style={{ flex: 1 }}>
              <Text style={{ fontSize: 13, fontWeight: "600", color: tokens.foreground }}>Essential Cookies</Text>
              <Text style={{ fontSize: 12, lineHeight: 17, color: tokens.mutedForeground }}>
                Required for the website to function properly. Cannot be disabled.
              </Text>
            </Column>
            <Switch checked={essential()} disabled onChange={() => {}} />
          </Row>
          <Row gap={16} style={{ alignItems: "flex-start" }}>
            <Column gap={2} style={{ flex: 1 }}>
              <Text style={{ fontSize: 13, fontWeight: "600", color: tokens.foreground }}>Analytics Cookies</Text>
              <Text style={{ fontSize: 12, lineHeight: 17, color: tokens.mutedForeground }}>
                Help us understand how visitors interact with our website.
              </Text>
            </Column>
            <Switch checked={analytics()} onChange={(next) => analytics.set(next)} />
          </Row>
          <Row gap={16} style={{ alignItems: "flex-start" }}>
            <Column gap={2} style={{ flex: 1 }}>
              <Text style={{ fontSize: 13, fontWeight: "600", color: tokens.foreground }}>Marketing Cookies</Text>
              <Text style={{ fontSize: 12, lineHeight: 17, color: tokens.mutedForeground }}>
                Used to deliver personalized advertisements and track ad campaign performance.
              </Text>
            </Column>
            <Switch checked={marketing()} onChange={(next) => marketing.set(next)} />
          </Row>
        </Column>
      </Dialog>
    </>
  );

  // section: Close Dialog Programmatically
  const sizesOpen = signal<"small" | "medium" | "large" | null>(null);
  const sizes = (
    <>
      <Row gap={8}>
        <Button label="Small" size="small" onPress={() => sizesOpen.set("small")} />
        <Button label="Medium" size="small" onPress={() => sizesOpen.set("medium")} />
        <Button label="Large" size="small" onPress={() => sizesOpen.set("large")} />
      </Row>
      <Dialog
        open={sizesOpen() !== null}
        size={sizesOpen() ?? "medium"}
        title={`${sizesOpen() ?? "Medium"} dialog`}
        description="Widths: 320, 440 and 560 points."
        onClose={() => sizesOpen.set(null)}
        footer={<Button label="Close" onPress={() => sizesOpen.set(null)} />}
      />
    </>
  );

  // section: Where dialogs render
  const overlayOpen = signal(false);
  const overlay = (
    <>
      <Button label="Open overlay-click dialog" onPress={() => overlayOpen.set(true)} />
      <Dialog
        open={overlayOpen()}
        title="Click the backdrop"
        description="Pressing Escape or the backdrop closes this dialog."
        overlayClosable
        onClose={() => overlayOpen.set(false)}
        footer={<Button label="Done" onPress={() => overlayOpen.set(false)} />}
      />
    </>
  );
  // section: end

  return (
    <Page>
      <Caption>
        The panel carries its own enter and exit: it fades, rises and settles, and leaving plays
        the reverse before the layer disappears. Escape and the backdrop close it when asked.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic dialog">{basic}</ExampleSection>
      <ExampleSection index={2} title="Scrollable Dialog">{scroll}</ExampleSection>
      <ExampleSection index={3} title="Action Buttons">{sticky}</ExampleSection>
      <ExampleSection index={4} title="Dialog Options">{noClose}</ExampleSection>
      <ExampleSection index={5} title="Custom close button">{customClose}</ExampleSection>
      <ExampleSection index={6} title="Custom Styling">{full}</ExampleSection>
      <ExampleSection index={7} title="Form dialog">{form}</ExampleSection>
      <ExampleSection index={8} title="Dialog with icon">{icon}</ExampleSection>
      <ExampleSection index={9} title="Nested dialogs">{nested}</ExampleSection>
      <ExampleSection index={10} title="Custom Padding">{cookies}</ExampleSection>
      <ExampleSection index={11} title="Close Dialog Programmatically">{sizes}</ExampleSection>
      <ExampleSection index={12} title="Where dialogs render">{overlay}</ExampleSection>
    </Page>
  );
}
