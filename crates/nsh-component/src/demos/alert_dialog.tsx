//! `alert-dialog` — a dialog for one decision.
//!
//! Every sample opens, confirms or cancels, and closes. The `// section:` markers
//! are the website's contract: one per heading on the alert-dialog page, in the
//! demo's order, closed by `// section: end`.
import { signal, Text, View } from "nsh";
import { AlertDialog } from "../components/alert_dialog";
import { Avatar } from "../components/avatar";
import { Button } from "../components/button";
import { Icon } from "../components/icon";
import { Input } from "../components/input";
import { Tag } from "../components/tag";
import { RADIUS_MD, SPACING_SM, tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  // section: Usage
  const usageOpen = signal(false);
  const usage = (
    <>
      <Button label="Apply settings" variant="primary" onPress={() => usageOpen.set(true)} />
      <AlertDialog
        open={usageOpen()}
        icon="info"
        tone="info"
        title="Apply new settings?"
        description="The app will restart to apply them."
        confirmLabel="Apply"
        cancelLabel="Not now"
        onConfirm={() => usageOpen.set(false)}
        onCancel={() => usageOpen.set(false)}
        onClose={() => usageOpen.set(false)}
      />
    </>
  );

  // section: Default
  const defaultOpen = signal(false);
  const defaultExample = (
    <>
      <Button label="Leave workspace" onPress={() => defaultOpen.set(true)} />
      <AlertDialog
        open={defaultOpen()}
        title="Leave workspace?"
        description="Your files stay in the workspace. You can rejoin with a new invite."
        confirmLabel="Leave"
        cancelLabel="Stay"
        showClose
        onConfirm={() => defaultOpen.set(false)}
        onCancel={() => defaultOpen.set(false)}
        onClose={() => defaultOpen.set(false)}
      />
    </>
  );

  // section: Delete file
  const deleteOpen = signal(false);
  const deleteExample = (
    <>
      <Button label="Delete file" variant="danger" onPress={() => deleteOpen.set(true)} />
      <AlertDialog
        open={deleteOpen()}
        icon="trash"
        title="Delete this file?"
        description="It will be removed for everyone with access to this workspace."
        confirmLabel="Delete file"
        destructive
        onConfirm={() => deleteOpen.set(false)}
        onCancel={() => deleteOpen.set(false)}
        onClose={() => deleteOpen.set(false)}
      >
        <View
          style={{
            flexDirection: "row",
            alignItems: "center",
            gap: SPACING_SM,
            padding: SPACING_SM,
            borderRadius: RADIUS_MD,
            backgroundColor: tokens.muted,
          }}
        >
          <Icon name="file-text" size={18} color={tokens.mutedForeground} />
          <View style={{ flex: 1, gap: 1 }}>
            <Text style={{ fontSize: 13, fontWeight: "600", color: tokens.foreground }}>
              quarterly-report.pdf
            </Text>
            <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>
              2.4 MB · Shared with 3 people
            </Text>
          </View>
          <Tag label="Shared" variant="secondary" size="xsmall" />
        </View>
      </AlertDialog>
    </>
  );

  // section: Icon
  const iconOpen = signal(false);
  const iconExample = (
    <>
      <Button label="Discard draft" onPress={() => iconOpen.set(true)} />
      <AlertDialog
        open={iconOpen()}
        icon="triangle-alert"
        tone="warning"
        title="Unsaved changes"
        description="Leaving now discards the edits you made in this draft."
        confirmLabel="Discard"
        cancelLabel="Keep editing"
        destructive
        onConfirm={() => iconOpen.set(false)}
        onCancel={() => iconOpen.set(false)}
        onClose={() => iconOpen.set(false)}
      />
    </>
  );

  // section: Destructive
  const destructiveOpen = signal(false);
  const destructiveExample = (
    <>
      <Button label="Delete account" variant="danger" onPress={() => destructiveOpen.set(true)} />
      <AlertDialog
        open={destructiveOpen()}
        title="Delete account?"
        description="This permanently removes your account and every project you own. This cannot be undone."
        confirmLabel="Delete account"
        cancelLabel="Keep account"
        destructive
        onConfirm={() => destructiveOpen.set(false)}
        onCancel={() => destructiveOpen.set(false)}
        onClose={() => destructiveOpen.set(false)}
      />
    </>
  );

  // section: Without title
  const untitledOpen = signal(false);
  const untitledExample = (
    <>
      <Button label="Sync changes" onPress={() => untitledOpen.set(true)} />
      <AlertDialog
        open={untitledOpen()}
        description="Your changes are saved on this device. Continue to sync them to the cloud?"
        confirmLabel="Sync now"
        cancelLabel="Later"
        onConfirm={() => untitledOpen.set(false)}
        onCancel={() => untitledOpen.set(false)}
        onClose={() => untitledOpen.set(false)}
      />
    </>
  );

  // section: Custom footer
  const footerOpen = signal(false);
  const footerExample = (
    <>
      <Button label="Restart required" onPress={() => footerOpen.set(true)} />
      <AlertDialog
        open={footerOpen()}
        icon="refresh-cw"
        tone="success"
        title="Restart to finish"
        description="The update is ready. Restarting keeps your open files."
        showClose
        footer={
          <>
            <Button
              label="Ask me later"
              variant="ghost"
              onPress={() => footerOpen.set(false)}
              style={{ flex: 1 }}
            />
            <Button
              label="Restart app"
              variant="primary"
              onPress={() => footerOpen.set(false)}
              style={{ flex: 1 }}
            />
          </>
        }
        onClose={() => footerOpen.set(false)}
      />
    </>
  );

  // section: Custom content
  const contentOpen = signal(false);
  const contentExample = (
    <>
      <Button label="Transfer workspace" onPress={() => contentOpen.set(true)} />
      <AlertDialog
        open={contentOpen()}
        icon="users"
        tone="info"
        title="Transfer workspace"
        description="Choose the teammate who will own this workspace."
        confirmLabel="Send invite"
        cancelLabel="Cancel"
        onConfirm={() => contentOpen.set(false)}
        onCancel={() => contentOpen.set(false)}
        onClose={() => contentOpen.set(false)}
      >
        <View style={{ gap: SPACING_SM }}>
          <Input placeholder="Search teammates" prefixIcon="search" onChangeText={() => {}} />
          <View
            style={{
              flexDirection: "row",
              alignItems: "center",
              gap: SPACING_SM,
              padding: SPACING_SM,
              borderRadius: RADIUS_MD,
              borderWidth: 1,
              borderColor: tokens.border,
            }}
          >
            <Avatar name="Ada Lovelace" size="sm" />
            <View style={{ flex: 1, gap: 1 }}>
              <Text style={{ fontSize: 13, fontWeight: "600", color: tokens.foreground }}>
                Ada Lovelace
              </Text>
              <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>ada@example.com</Text>
            </View>
            <Tag label="Admin" variant="info" size="xsmall" />
          </View>
        </View>
      </AlertDialog>
    </>
  );

  // section: Notice
  const noticeOpen = signal(false);
  const noticeExample = (
    <>
      <Button label="View maintenance notice" onPress={() => noticeOpen.set(true)} />
      <AlertDialog
        open={noticeOpen()}
        icon="info"
        tone="info"
        title="Scheduled maintenance"
        description="The workspace will be read-only on Sunday from 02:00 to 03:00 UTC."
        confirmLabel="Got it"
        showCancel={false}
        onConfirm={() => noticeOpen.set(false)}
        onClose={() => noticeOpen.set(false)}
      />
    </>
  );

  // section: Session Timeout
  const sessionOpen = signal(false);
  const sessionExample = (
    <>
      <Button label="Simulate session timeout" onPress={() => sessionOpen.set(true)} />
      <AlertDialog
        open={sessionOpen()}
        icon="clock"
        tone="warning"
        title="Session expired"
        description="You were signed out after 30 minutes of inactivity. Sign in to continue where you left off."
        confirmLabel="Sign in"
        showCancel={false}
        onConfirm={() => sessionOpen.set(false)}
        onClose={() => sessionOpen.set(false)}
      />
    </>
  );

  // section: Update Available
  const updateOpen = signal(false);
  const updating = signal(false);
  const updateExample = (
    <>
      <Button label="Check for updates" onPress={() => updateOpen.set(true)} />
      <AlertDialog
        open={updateOpen()}
        icon="download"
        tone="success"
        title="Update available"
        description="Version 2.0 adds faster sync and a new command bar."
        confirmLabel={updating() ? "Updating…" : "Update now"}
        cancelLabel="Later"
        loading={updating()}
        showClose
        onConfirm={() => {
          updating.set(true);
          setTimeout(() => {
            updating.set(false);
            updateOpen.set(false);
          }, 1200);
        }}
        onCancel={() => {
          if (!updating()) updateOpen.set(false);
        }}
        onClose={() => {
          if (!updating()) updateOpen.set(false);
        }}
      />
    </>
  );
  // section: end

  return (
    <Page>
      <Caption>
        AlertDialog interrupts for one decision: a clear title, the consequence, and a footer that
        cannot be missed. A loading confirmation shows its spinner and refuses a second press.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Default">{defaultExample}</ExampleSection>
      <ExampleSection index={2} title="Delete file">{deleteExample}</ExampleSection>
      <ExampleSection index={3} title="Icon">{iconExample}</ExampleSection>
      <ExampleSection index={4} title="Destructive">{destructiveExample}</ExampleSection>
      <ExampleSection index={5} title="Without title">{untitledExample}</ExampleSection>
      <ExampleSection index={6} title="Custom footer">{footerExample}</ExampleSection>
      <ExampleSection index={7} title="Custom content">{contentExample}</ExampleSection>
      <ExampleSection index={8} title="Notice">{noticeExample}</ExampleSection>
      <ExampleSection index={9} title="Session Timeout">{sessionExample}</ExampleSection>
      <ExampleSection index={10} title="Update Available">{updateExample}</ExampleSection>
    </Page>
  );
}
