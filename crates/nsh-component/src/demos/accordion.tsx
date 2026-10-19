//! `accordion` — collapsible sections, one or several open.
import { signal, Text, View } from "nsh";
import { Accordion } from "../components/accordion";
import { Avatar } from "../components/avatar";
import { Button } from "../components/button";
import { Switch } from "../components/switch";
import { tokens } from "../components/theme";
import { Caption, Column, ExampleSection, Page, Row, Status } from "./ui";

export function Demo() {
  const notice = signal("");

  // section: Usage
  const faqOpen = signal<string[]>(["email"]);
  const usage = (
    <Accordion
      bordered
      value={faqOpen()}
      onChange={(next) => faqOpen.set(next)}
      items={[
        {
          id: "history",
          title: "Can I access my account history?",
          content: (
            <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
              Your history is kept for twelve months. Open Account → History to filter by
              date, device or action.
            </Text>
          ),
        },
        {
          id: "premium",
          title: "Premium feature information",
          description: "Locked on this plan.",
          disabled: true,
          content: (
            <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>
              Upgrade to unlock the full history.
            </Text>
          ),
        },
        {
          id: "email",
          title: "How do I update my email address?",
          content: (
            <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
              You can update your email address in your account settings. You’ll receive a
              verification email at your new address to confirm the change.
            </Text>
          ),
        },
      ]}
    />
  );

  // section: Single
  const accountOpen = signal<string[]>(["account"]);
  const single = (
    <Accordion
      type="single"
      separated
      value={accountOpen()}
      onChange={(next) => accountOpen.set(next)}
      items={[
        {
          id: "account",
          title: "Account Settings",
          description: "Profile, security and personal information.",
          icon: "settings",
          badge: "New",
          badgeTone: "success",
          content: (
            <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
              Manage your account preferences, security settings, and personal information.
              You can also configure two-factor authentication here.
            </Text>
          ),
        },
        {
          id: "privacy",
          title: "Privacy & Security",
          icon: "lock",
          content: (
            <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
              Control sessions, recovery codes and which devices can reach the account.
            </Text>
          ),
        },
        {
          id: "help",
          title: "Help & Support",
          icon: "message-circle",
          content: (
            <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
              Search the guides or open a conversation with the support team.
            </Text>
          ),
        },
      ]}
    />
  );

  // section: Multiple
  const nestedOpen = signal<string[]>(["specs"]);
  const multiple = (
    <Accordion
      bordered
      type="multiple"
      defaultValue={["details"]}
      items={[
        {
          id: "overview",
          title: "Product Overview",
          content: (
            <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
              A compact summary of what is in the box and who it is for.
            </Text>
          ),
        },
        {
          id: "details",
          title: "Additional Details",
          content: (
            <Accordion
              separated
              size="small"
              value={nestedOpen()}
              onChange={(next) => nestedOpen.set(next)}
              items={[
                {
                  id: "specs",
                  title: "Technical Specifications",
                  content: (
                    <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.mutedForeground }}>
                      Detailed technical specs including dimensions, weight, and power
                      requirements.
                    </Text>
                  ),
                },
                {
                  id: "compatibility",
                  title: "Compatibility",
                  content: (
                    <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.mutedForeground }}>
                      Works with the current and previous release.
                    </Text>
                  ),
                },
              ]}
            />
          ),
        },
        {
          id: "shipping",
          title: "Shipping & Returns",
          content: (
            <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
              Delivery takes two to four business days. Returns are open for thirty days.
            </Text>
          ),
        },
      ]}
    />
  );

  // section: Icons and custom content
  const teamOpen = signal<string[]>(["alex"]);
  const icons = (
    <Accordion
      bordered
      value={teamOpen()}
      onChange={(next) => teamOpen.set(next)}
      items={[
        {
          id: "alex",
          leading: <Avatar name="Alex Johnson" size="md" />,
          title: "Alex Johnson",
          badge: "Admin",
          badgeTone: "success",
          content: (
            <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
              Alex has full administrative access, including billing management, user
              provisioning, and security configurations.
            </Text>
          ),
        },
        {
          id: "sarah",
          leading: <Avatar name="Sarah Chen" size="md" />,
          title: "Sarah Chen",
          badge: "Viewer",
          badgeTone: "neutral",
          content: (
            <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
              Sarah can view projects and comment, but cannot change workspace settings.
            </Text>
          ),
        },
        {
          id: "michael",
          leading: <Avatar name="Michael Rodriguez" size="md" />,
          title: "Michael Rodriguez",
          badge: "Editor",
          badgeTone: "info",
          content: (
            <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
              Michael can create and edit project content, and invite collaborators.
            </Text>
          ),
        },
      ]}
    />
  );

  // section: Sizes
  const sizes = (
    <Column gap={10}>
      <Accordion
        bordered
        size="small"
        defaultValue={["compact"]}
        items={[
          {
            id: "compact",
            title: "Compact rows",
            description: "For a sidebar or a dense settings column.",
            content: (
              <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.mutedForeground }}>
                The same disclosure, with tighter padding and a smaller title.
              </Text>
            ),
          },
          {
            id: "shortcuts",
            title: "Shortcuts",
            content: (
              <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.mutedForeground }}>
                Chevron and content stay aligned at every size.
              </Text>
            ),
          },
        ]}
      />
      <Accordion
        bordered
        size="large"
        defaultValue={["roomy"]}
        items={[
          {
            id: "roomy",
            title: "Roomy rows",
            description: "For a long answer that deserves space.",
            content: (
              <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
                Padding and the title grow together; the hierarchy does not change.
              </Text>
            ),
          },
        ]}
      />
    </Column>
  );

  // section: Borderless and disabled
  const borderless = (
    <Accordion
      defaultValue={["workspace"]}
      items={[
        {
          id: "workspace",
          title: "Workspace",
          description: "No outer frame.",
          icon: "sparkles",
          content: (
            <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
              Borderless is the default. Use it inside a settings page where the surrounding
              layout already provides the surface.
            </Text>
          ),
        },
        {
          id: "legacy",
          title: "Legacy API",
          description: "Available on the Enterprise plan.",
          icon: "lock",
          disabled: true,
          content: (
            <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>
              A disabled section cannot be opened.
            </Text>
          ),
        },
      ]}
    />
  );

  // section: With Borders
  const surfaces = (
    <Column gap={12}>
      <Accordion
        bordered
        defaultValue={["billing"]}
        items={[
          {
            id: "billing",
            title: "Billing",
            description: "Plan, invoices and receipts.",
            content: (
              <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
                One surface holds the stack; the hairline between sections stays inside it.
              </Text>
            ),
          },
          {
            id: "shipping",
            title: "Shipping",
            content: (
              <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
                The open row stays quiet; content does the work.
              </Text>
            ),
          },
        ]}
      />
      <Accordion
        separated
        defaultValue={["profile"]}
        items={[
          {
            id: "profile",
            title: "Profile",
            description: "Its own card.",
            content: (
              <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
                Separated sections drop the hairline and space the cards apart.
              </Text>
            ),
          },
          {
            id: "security",
            title: "Security",
            content: (
              <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
                Two-factor, sessions and the recovery code.
              </Text>
            ),
          },
        ]}
      />
    </Column>
  );

  // section: Handle Toggle Events
  const eventsOpen = signal<string[]>([]);
  const events = (
    <Column gap={8}>
      <Accordion
        bordered
        type="multiple"
        value={eventsOpen()}
        onChange={(next) => eventsOpen.set(next)}
        items={[
          {
            id: "package",
            title: "Package",
            content: (
              <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
                The bundle, its assets and the version the registry publishes.
              </Text>
            ),
          },
          {
            id: "runtime",
            title: "Runtime",
            content: (
              <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
                The VM, its capabilities and the platform it runs on.
              </Text>
            ),
          },
        ]}
      />
      <Row>
        <Button label="Open all" size="small" onPress={() => eventsOpen.set(["package", "runtime"])} />
        <Button label="Close all" size="small" variant="ghost" onPress={() => eventsOpen.set([])} />
      </Row>
    </Column>
  );

  // section: Nested Accordions
  const nested = (
    <Accordion
      bordered
      defaultValue={["components"]}
      items={[
        {
          id: "components",
          title: "Components",
          description: "The files an app installs.",
          icon: "folder",
          content: (
            <View style={{ gap: 8 }}>
              <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
                A section holds any element, including another accordion.
              </Text>
              <Accordion
                separated
                size="small"
                defaultValue={["theme"]}
                items={[
                  {
                    id: "theme",
                    title: "theme.ts",
                    content: (
                      <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.mutedForeground }}>
                        The reactive face of the palette.
                      </Text>
                    ),
                  },
                  {
                    id: "tokens",
                    title: "tokens.ts",
                    content: (
                      <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.mutedForeground }}>
                        The two palettes and the scales.
                      </Text>
                    ),
                  },
                ]}
              />
            </View>
          ),
        },
        {
          id: "registry",
          title: "Registry",
          description: "What the CLI copies from.",
          content: (
            <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
              The manifest lists every component, its example and the files it installs.
            </Text>
          ),
        },
      ]}
    />
  );
  // section: end

  return (
    <Page>
      <Caption>
        Accordion stacks collapsible sections and coordinates them. ReUI-style surfaces:
        quiet cards, a plain chevron, compact status pills, and content that appears
        immediately.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Single">{single}</ExampleSection>
      <ExampleSection index={2} title="Multiple">{multiple}</ExampleSection>
      <ExampleSection index={3} title="Icons and custom content">{icons}</ExampleSection>
      <ExampleSection index={4} title="Sizes">{sizes}</ExampleSection>
      <ExampleSection index={5} title="Borderless and disabled">{borderless}</ExampleSection>
      <ExampleSection index={6} title="With Borders">{surfaces}</ExampleSection>
      <ExampleSection index={7} title="Handle Toggle Events">{events}</ExampleSection>
      <ExampleSection index={8} title="Nested Accordions">{nested}</ExampleSection>
      <Status>{`Open: ${faqOpen().join(", ") || "none"}${notice() ? ` · ${notice()}` : ""}`}</Status>
    </Page>
  );
}
