//! `alert` — an inline callout.
import { signal, Text, View } from "nsh";
import { LayoutAnimation } from "nsh/animated";
import { reduceMotion } from "nsh/accessibility";
import { Alert } from "../components/alert";
import { Avatar } from "../components/avatar";
import { Button } from "../components/button";
import { tokens } from "../components/theme";
import { Caption, Column, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  // section: Usage
  const usage = (
    <Alert
      title="Heads up"
      description="This is a neutral callout. Use an alert for information that belongs with the page."
    />
  );

  // section: Default
  const normal = (
    <Alert
      title="Alert!"
      description="This is an alert with a title and description."
    />
  );

  // section: Variants
  const variants = (
    <Column gap={8}>
      <Alert variant="info" title="Information" description="A new version is available." />
      <Alert variant="success" title="Saved" description="Your changes are on the server." />
      <Alert variant="warning" title="Check the form" description="One field still needs attention." />
      <Alert variant="danger" title="Delete failed" description="The item could not be removed." />
    </Column>
  );

  // section: Custom icon
  const customIcon = (
    <Alert
      variant="warning"
      icon="shield"
      title="Security update"
      description="Update your password and enable 2FA for the workspace."
    />
  );

  // section: Alert with Title
  const title = (
    <Alert
      variant="info"
      title="A titled alert"
      description="The description sits under the title and keeps the body quiet."
    />
  );

  // section: Alert Sizes
  const sizes = (
    <Column gap={8}>
      <Alert size="small" variant="info" title="Small" description="A compact inline note." />
      <Alert size="medium" variant="info" title="Medium" description="The default callout measure." />
      <Alert size="large" variant="info" title="Large" description="More room for a longer message." />
    </Column>
  );

  // section: Closable Alerts
  const visible = signal(true);
  const dismiss = () => {
    if (!reduceMotion()) LayoutAnimation.easeInEaseOut();
    visible.set(false);
  };
  const closable = (
    <>
      {visible() ? (
        <Alert
          variant="warning"
          title="Dismissible"
          description="Dismissing removes the callout with a short native layout transition."
          closable
          onClose={dismiss}
        />
      ) : (
        <Button label="Show alert" size="small" onPress={() => visible.set(true)} />
      )}
    </>
  );

  // section: Banner
  const banner = (
    <Alert
      variant="info"
      banner
      title="Banner mode"
      description="The callout spans the edge without a corner radius."
    />
  );

  // section: Conditional Visibility
  const shown = signal(true);
  const toggle = () => {
    if (!reduceMotion()) LayoutAnimation.easeInEaseOut();
    shown.set(!shown());
  };
  const conditional = (
    <Column gap={8}>
      {shown() ? (
        <Alert
          variant="success"
          title="Connected"
          description="The workspace is synced and ready."
          action={<Button label="Disconnect" size="small" variant="ghost" onPress={toggle} />}
        />
      ) : null}
      <Row>
        <Button label={shown() ? "Hide alert" : "Show alert"} size="small" onPress={toggle} />
      </Row>
    </Column>
  );

  // section: Form Validation Errors
  const validation = (
    <Alert variant="danger" title="Check the form" description="The following fields need attention:">
      <View style={{ gap: 3 }}>
        <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.danger }}>• Email address is missing.</Text>
        <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.danger }}>• Password must be at least 12 characters.</Text>
      </View>
    </Alert>
  );

  // section: Success Notification
  const success = (
    <Alert
      variant="success"
      title="Deployment successful"
      description="Your application has been successfully deployed to the production environment."
      action={<Button label="View logs" size="small" variant="ghost" onPress={() => {}} />}
    />
  );

  // section: System Status Banner
  const systemStatus = (
    <View
      style={{
        padding: 8,
        gap: 8,
        borderRadius: 10,
        borderWidth: 1,
        borderColor: tokens.border,
        backgroundColor: tokens.background,
      }}
    >
      <Alert
        variant="success"
        size="small"
        title="Database connected"
        description="All systems operational. Last sync: 2 minutes ago."
      />
      <Alert
        variant="warning"
        size="small"
        title="API latency warning"
        description="Increased latency in US-East. Engineers are investigating."
        action={<Button label="Retry" size="small" variant="ghost" onPress={() => {}} />}
      />
    </View>
  );

  // section: Interactive Alert with Custom Action
  const actionable = (
    <Alert
      variant="info"
      title="Update your password and enable 2FA"
      description="A stronger sign-in protects this workspace."
      action={
        <>
          <Button label="Dismiss" size="small" variant="ghost" onPress={() => {}} />
          <Button label="Update" size="small" onPress={() => {}} />
        </>
      }
    />
  );

  // section: Multi-line Content with Formatting
  const multiLine = (
    <Alert
      variant="info"
      title="What changed in this release"
      description="A short summary, followed by the details that need formatting."
    >
      <View style={{ gap: 3 }}>
        <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.info }}>
          • Faster sync across devices.
        </Text>
        <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.info }}>
          • A new command bar and keyboard shortcuts.
        </Text>
        <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.info }}>
          • Updated native controls in the gallery.
        </Text>
      </View>
    </Alert>
  );

  // section: Destructive
  const destructive = (
    <Alert
      variant="danger"
      title="Payment failed"
      description="Please check your payment details:"
    >
      <View style={{ gap: 3 }}>
        <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.danger }}>• Card number and expiry</Text>
        <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.danger }}>• Billing address</Text>
        <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.danger }}>• Available funds</Text>
      </View>
    </Alert>
  );

  // section: Inverted alert
  const inverted = (
    <Alert
      inverted
      icon="sparkles"
      title="Pro Feature"
      description="This feature is only available for Pro users. Upgrade your plan to get access."
      action={
        <>
          <Button label="Dismiss" size="small" variant="ghost" onPress={() => {}} />
          <Button label="Upgrade" size="small" onPress={() => {}} />
        </>
      }
    />
  );

  // section: User message alert
  const messages = (
    <Column gap={8}>
      <Alert
        leading={<Avatar name="Alex Johnson" size="sm" />}
        title="Alex Johnson sent you a message"
        description="“Hey! I’ve finished the draft for…”"
        action={
          <>
            <Button label="View" size="small" variant="ghost" onPress={() => {}} />
            <Button label="Reply" size="small" onPress={() => {}} />
          </>
        }
      />
      <Alert
        inverted
        variant="info"
        leading={<Avatar name="Sarah Chen" size="sm" />}
        title="Sarah Chen mentioned you"
        description="“Great work on the user profile…”"
        action={
          <>
            <Button label="Dismiss" size="small" variant="ghost" onPress={() => {}} />
            <Button label="View" size="small" onPress={() => {}} />
          </>
        }
      />
    </Column>
  );
  // section: end

  return (
    <Page>
      <Caption>
        Alerts are inline: a tinted surface, a quiet icon, a clear title and an optional action
        row. Use a dialog when the message needs an answer before the page continues.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Default">{normal}</ExampleSection>
      <ExampleSection index={2} title="Variants">{variants}</ExampleSection>
      <ExampleSection index={3} title="Custom icon">{customIcon}</ExampleSection>
      <ExampleSection index={4} title="Alert with Title">{title}</ExampleSection>
      <ExampleSection index={5} title="Alert Sizes">{sizes}</ExampleSection>
      <ExampleSection index={6} title="Closable Alerts">{closable}</ExampleSection>
      <ExampleSection index={7} title="Banner">{banner}</ExampleSection>
      <ExampleSection index={8} title="Conditional Visibility">{conditional}</ExampleSection>
      <ExampleSection index={9} title="Form Validation Errors">{validation}</ExampleSection>
      <ExampleSection index={10} title="Success Notification">{success}</ExampleSection>
      <ExampleSection index={11} title="System Status Banner">{systemStatus}</ExampleSection>
      <ExampleSection index={12} title="Interactive Alert with Custom Action">{actionable}</ExampleSection>
      <ExampleSection index={13} title="Multi-line Content with Formatting">{multiLine}</ExampleSection>
      <ExampleSection index={14} title="Destructive">{destructive}</ExampleSection>
      <ExampleSection index={15} title="Inverted alert">{inverted}</ExampleSection>
      <ExampleSection index={16} title="User message alert">{messages}</ExampleSection>
    </Page>
  );
}
