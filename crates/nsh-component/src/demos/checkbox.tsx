//! `checkbox` — independent choices, from the plain box to ReUI cards,
//! colored and circular boxes, nested groups and icon cards.
import { signal, Text, View } from "nsh";
import { Avatar } from "../components/avatar";
import { Badge } from "../components/badge";
import { Checkbox, CheckboxGroup } from "../components/checkbox";
import { Icon } from "../components/icon";
import { tokens } from "../components/theme";
import { Caption, Column, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const Info = () => <Icon name="info" size={12} color={tokens.mutedForeground} />;
  const basicState = signal(true);
  const controlled = signal(false);
  const invalid = signal(false);
  const cardChannels = signal({ push: true, email: false, sms: false });
  const permissions = signal<Record<string, boolean>>({
    admin: true,
    userManagement: true,
    viewUsers: true,
    createUsers: true,
    editUsers: false,
    roleManagement: true,
    viewRoles: true,
    assignRoles: false,
  });
  const cards = signal<Record<string, boolean>>({ payments: true, invoices: false, billing: true, reports: true });
  const features = signal({ ai: false, beta: false });
  const enableNotifications = signal(true);
  const master = signal(true);

  const childRows = () => [
    { label: "User Management", value: "userManagement", children: ["viewUsers", "createUsers", "editUsers"] },
    { label: "Role Management", value: "roleManagement", children: ["viewRoles", "assignRoles"] },
  ];

  // section: Usage
  const usage = (
    <Checkbox
      checked={basicState()}
      onChange={(next) => basicState.set(next)}
      label="Accept the terms and conditions"
      description="You can change this later in settings."
    />
  );

  // section: Basic
  const basic = (
    <Checkbox
      checked={basicState()}
      onChange={(next) => basicState.set(next)}
      label="Accept the terms and conditions"
    />
  );

  // section: Default
  const normal = (
    <Row gap={10} style={{ flexDirection: "column", maxWidth: 420 }}>
      <Checkbox defaultChecked label="Email me about updates" description="No more than once a month." />
      <Checkbox label="Product announcements" description="New features, sent roughly each quarter." />
    </Row>
  );

  // section: Sizes
  const sizes = (
    <Row gap={16}>
      <Checkbox size="small" defaultChecked label="Small" />
      <Checkbox defaultChecked label="Medium" />
      <Checkbox size="large" defaultChecked label="Large" />
    </Row>
  );

  // section: Controlled Checkbox
  const controlledView = (
    <Checkbox
      checked={controlled()}
      onChange={(next) => controlled.set(next)}
      label={controlled() ? "Checked" : "Unchecked"}
    />
  );

  // section: Disabled State
  const disabledView = (
    <Row gap={16}>
      <Checkbox disabled label="Disabled" />
      <Checkbox disabled defaultChecked label="Disabled, checked" />
      <Checkbox disabled indeterminate label="Disabled, mixed" />
    </Row>
  );

  // section: Without Label
  const withoutLabel = (
    <Row gap={10}>
      <Checkbox defaultChecked />
      <Checkbox />
      <Checkbox indeterminate />
    </Row>
  );

  // section: Labels
  const labels = (
    <Row gap={10} style={{ flexDirection: "column", maxWidth: 420 }}>
      <Checkbox defaultChecked label="Email me about updates" description="No more than once a month." />
      <Checkbox label="Enable advanced analytics" leading={<Info />} />
    </Row>
  );

  // section: Checkbox List
  const list = (
    <CheckboxGroup frame style={{ width: 420, maxWidth: "100%" }}>
      <Checkbox block defaultChecked label="Design" description="Components and tokens." />
      <Checkbox block label="Engineering" description="Platform and native shells." />
      <Checkbox block defaultChecked label="Docs" description="Guides and API reference." />
    </CheckboxGroup>
  );

  // section: Invalid checkbox
  const invalidView = (
    <Column gap={8}>
      <Checkbox label="Accept the terms" variant="card" style={{ width: 300, borderColor: tokens.danger }} />
      <Row gap={6}>
        <Icon name="circle-alert" size={12} color={tokens.danger} />
        <Text style={{ fontSize: 11, color: tokens.danger }}>You must accept the terms to continue.</Text>
      </Row>
      {invalid() ? null : null}
    </Column>
  );

  // section: Indeterminate checkbox
  const indeterminate = (
    <Checkbox indeterminate label="Indeterminate state" />
  );

  // section: Colored checkbox
  const colored = (
    <Column gap={8}>
      <Checkbox defaultChecked color={tokens.info} label="Blue checkbox" />
      <Checkbox defaultChecked color={tokens.success} label="Green checkbox" />
      <Checkbox defaultChecked color={tokens.warning} label="Yellow checkbox" />
    </Column>
  );

  // section: Circle checkbox
  const circle = (
    <Row gap={16}>
      <Checkbox shape="circle" defaultChecked label="Circle checkbox" />
      <Checkbox shape="circle" label="Circle unchecked" />
    </Row>
  );

  // section: Checkbox group
  const group = (
    <CheckboxGroup label="Notification Settings" style={{ width: 360, maxWidth: "100%" }}>
      <Checkbox label="Email notifications" checked={cardChannels().email} onChange={(next) => cardChannels.update((value) => ({ ...value, email: next }))} />
      <Checkbox label="SMS notifications" checked={cardChannels().sms} onChange={(next) => cardChannels.update((value) => ({ ...value, sms: next }))} />
      <Checkbox label="Push notifications" checked={cardChannels().push} onChange={(next) => cardChannels.update((value) => ({ ...value, push: next }))} />
    </CheckboxGroup>
  );

  // section: Card checkbox with group
  const cardWithGroup = (
    <Checkbox
      variant="card"
      block
      checked={enableNotifications()}
      onChange={(next) => enableNotifications.set(next)}
      label="Enable notifications"
      description="You can enable or disable notifications at any time."
      style={{ width: 380, maxWidth: "100%" }}
    />
  );

  // section: Checkbox group in a frame
  const framed = (
    <CheckboxGroup
      frame
      label="Notification Settings"
      style={{ width: 360, maxWidth: "100%" }}
    >
      <Checkbox variant="card" block checked={cardChannels().push} onChange={(next) => cardChannels.update((value) => ({ ...value, push: next }))} label="Push notifications" />
      <Checkbox variant="card" block checked={cardChannels().email} onChange={(next) => cardChannels.update((value) => ({ ...value, email: next }))} label="Email notifications" />
      <Checkbox variant="card" block checked={cardChannels().sms} onChange={(next) => cardChannels.update((value) => ({ ...value, sms: next }))} label="SMS notifications" />
    </CheckboxGroup>
  );

  // section: Card checkbox group with icons
  const iconCards = (
    <CheckboxGroup frame style={{ width: 380, maxWidth: "100%" }}>
      <Checkbox variant="card" block icon="bell" checked={cardChannels().push} onChange={(next) => cardChannels.update((value) => ({ ...value, push: next }))} label="Push notifications" />
      <Checkbox variant="card" block icon="mail" checked={cardChannels().email} onChange={(next) => cardChannels.update((value) => ({ ...value, email: next }))} label="Email notifications" />
      <Checkbox variant="card" block icon="message-circle" checked={cardChannels().sms} onChange={(next) => cardChannels.update((value) => ({ ...value, sms: next }))} label="SMS notifications" />
    </CheckboxGroup>
  );

  // section: Custom positioned checkbox
  const featureCards = () => {
    const items = [
      { id: "payments", label: "Payments", description: "Receive payments from your customers", icon: "circle-dollar-sign" },
      { id: "invoices", label: "Invoices", description: "Create and send invoices to your customers", icon: "file-text" },
      { id: "billing", label: "Billing", description: "Manage your billing and subscriptions", icon: "credit-card" },
      { id: "reports", label: "Reports", description: "View your reports and analytics", icon: "bar-chart-3" },
    ];
    return (
      <Row gap={8} style={{ flexWrap: "wrap", width: 380, maxWidth: "100%" }}>
        {items.map((item) => (
          <Checkbox
            variant="icon"
            icon={item.icon as never}
            block
            checked={cards()[item.id]}
            onChange={(next) => cards.update((value) => ({ ...value, [item.id]: next }))}
            label={item.label}
            description={item.description}
            style={{ width: 184 }}
          />
        ))}
      </Row>
    );
  };

  // section: Avatar based card checkbox
  const avatarCard = (
    <Checkbox
      variant="card"
      block
      icon="user"
      checked={true}
      onChange={() => {}}
      leading={<Avatar name="Emma Wilson" size="sm" />}
      label="Emma Wilson"
      description="@emmawilson"
      style={{ width: 380, maxWidth: "100%" }}
    />
  );

  // section: Payment method card checkbox
  const paymentCard = (
    <Checkbox
      variant="card"
      block
      onChange={() => {}}
      checked
      leading={
        <View style={{ flexDirection: "row" }}>
          <View style={{ width: 22, height: 22, borderRadius: 11, backgroundColor: tokens.warning, marginEnd: -6 }} />
          <View style={{ width: 22, height: 22, borderRadius: 11, backgroundColor: tokens.danger, opacity: 0.9 }} />
        </View>
      }
      label="Mastercard ending in 8888"
      description="Expires 09/25"
      style={{ width: 380, maxWidth: "100%" }}
    />
  );

  // section: Checkbox with label and tooltip info
  const withInfo = (
    <Checkbox label="Enable advanced analytics" leading={<Info />} />
  );

  // section: Checkbox with feature badge
  const badges = (
    <Column gap={8}>
      <Checkbox checked={features().ai} onChange={(next) => features.update((value) => ({ ...value, ai: next }))} label="AI-powered suggestions" badge="NEW" />
      <Checkbox checked={features().beta} onChange={(next) => features.update((value) => ({ ...value, beta: next }))} label="Beta feature access" badge="BETA" />
    </Column>
  );

  // section: Checkbox group with badge
  const badgeGroups = (
    <Column gap={8} style={{ width: 340, maxWidth: "100%" }}>
      <Checkbox variant="card" block label="AI-powered suggestions" badge="NEW" />
      <Checkbox variant="card" block label="Beta feature access" badge="BETA" />
    </Column>
  );

  // section: Inline horizontal checkbox group
  const inline = (
    <Row gap={14}>
      <Checkbox defaultChecked label="Read" />
      <Checkbox label="Write" />
      <Checkbox defaultChecked label="Delete" />
    </Row>
  );

  // section: Nested checkbox group
  const nested = () => (
    <View style={{ gap: 6, width: 420, maxWidth: "100%" }}>
      <Checkbox
        checked={permissions().admin}
        indeterminate={!permissions().admin && permissions().userManagement}
        onChange={(next) =>
          permissions.set(
            next
              ? { admin: true, userManagement: true, viewUsers: true, createUsers: true, editUsers: true, roleManagement: true, viewRoles: true, assignRoles: true }
              : { admin: false, userManagement: false, viewUsers: false, createUsers: false, editUsers: false, roleManagement: false, viewRoles: false, assignRoles: false },
          )
        }
        label="Administration"
      >
        {childRows().map((row) => (
          <View style={{ gap: 4 }}>
            <Checkbox
              checked={permissions()[row.value] ?? false}
              onChange={(next) => permissions.update((value) => ({ ...value, [row.value]: next }))}
              label={row.label}
            >
              {row.children.map((child) => (
                <Checkbox
                  checked={permissions()[child] ?? false}
                  onChange={(next) => permissions.update((value) => ({ ...value, [child]: next }))}
                  label={{
                    viewUsers: "View Users",
                    createUsers: "Create Users",
                    editUsers: "Edit Users",
                    viewRoles: "View Roles",
                    assignRoles: "Assign Roles",
                  }[child] ?? child}
                />
              ))}
            </Checkbox>
          </View>
        ))}
      </Checkbox>
    </View>
  );

  // section: Custom Tab Order
  const tabOrder = (
    <Row gap={12}>
      <Checkbox defaultChecked label="First" />
      <Checkbox label="Second" />
      <Checkbox defaultChecked label="Third" />
    </Row>
  );
  // section: end

  return (
    <Page>
      <Caption>
        A checkbox is an independent choice; a radio group is one of several. Cards and nested
        groups only change the surface, not the promise.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={2} title="Default">{normal}</ExampleSection>
      <ExampleSection index={3} title="Sizes">{sizes}</ExampleSection>
      <ExampleSection index={4} title="Controlled Checkbox">{controlledView}</ExampleSection>
      <ExampleSection index={5} title="Disabled State">{disabledView}</ExampleSection>
      <ExampleSection index={6} title="Without Label">{withoutLabel}</ExampleSection>
      <ExampleSection index={7} title="Labels">{labels}</ExampleSection>
      <ExampleSection index={8} title="Custom Tab Order">{tabOrder}</ExampleSection>
      <ExampleSection index={9} title="Checkbox List">{list}</ExampleSection>
      <ExampleSection index={10} title="Invalid checkbox">{invalidView}</ExampleSection>
      <ExampleSection index={11} title="Indeterminate checkbox">{indeterminate}</ExampleSection>
      <ExampleSection index={12} title="Colored checkbox">{colored}</ExampleSection>
      <ExampleSection index={13} title="Circle checkbox">{circle}</ExampleSection>
      <ExampleSection index={14} title="Checkbox group">{group}</ExampleSection>
      <ExampleSection index={15} title="Card checkbox with group">{cardWithGroup}</ExampleSection>
      <ExampleSection index={16} title="Checkbox group in a frame">{framed}</ExampleSection>
      <ExampleSection index={17} title="Card checkbox group with icons">{iconCards}</ExampleSection>
      <ExampleSection index={18} title="Custom positioned checkbox">{featureCards()}</ExampleSection>
      <ExampleSection index={19} title="Avatar based card checkbox">{avatarCard}</ExampleSection>
      <ExampleSection index={20} title="Payment method card checkbox">{paymentCard}</ExampleSection>
      <ExampleSection index={21} title="Checkbox with label and tooltip info">{withInfo}</ExampleSection>
      <ExampleSection index={22} title="Checkbox with feature badge">{badges}</ExampleSection>
      <ExampleSection index={23} title="Checkbox group with badge">{badgeGroups}</ExampleSection>
      <ExampleSection index={24} title="Inline horizontal checkbox group">{inline}</ExampleSection>
      <ExampleSection index={25} title="Nested checkbox group">{nested()}</ExampleSection>
    </Page>
  );
}
