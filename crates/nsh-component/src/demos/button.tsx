//! `button` — variants, outline, sizes, icons, states, groups and loading.
//!
//! The `// section:` markers are the website's contract: one per heading on the
//! button page, in order, closed by `// section: end`.
import { signal, Text, View } from "nsh";
import { Button, type ButtonVariant } from "../components/button";
import { alpha, RADIUS_LG, tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row, Status } from "./ui";

const variants: ButtonVariant[] = [
  "default",
  "primary",
  "secondary",
  "danger",
  "warning",
  "success",
  "info",
  "ghost",
  "link",
  "text",
];

export function Demo() {
  const pressed = signal("Nothing pressed yet");
  const syncing = signal(false);
  const playing = signal(false);
  const report = (label: string) => pressed.set(label);

  // section: Usage
  const usage = (
    <Row gap={8}>
      <Button label="Save changes" icon="check" variant="primary" onPress={() => report("Save changes")} />
      <Button label="Cancel" variant="ghost" onPress={() => report("Cancel")} />
    </Row>
  );

  // section: Variants
  const variantRow = (
    <Row gap={8}>
      {variants.map((variant) => (
        <Button
          label={variant === "default" ? "Button" : variant.charAt(0).toUpperCase() + variant.slice(1)}
          variant={variant}
          onPress={() => report(`Pressed ${variant}`)}
        />
      ))}
    </Row>
  );

  // section: Outline
  const outline = (
    <Row gap={8}>
      <Button label="Outline" outline onPress={() => report("Outline")} />
      <Button label="Primary" variant="primary" outline onPress={() => report("Outline primary")} />
      <Button label="Secondary" variant="secondary" outline onPress={() => report("Outline secondary")} />
      <Button label="Danger" variant="danger" outline onPress={() => report("Outline danger")} />
    </Row>
  );

  // section: Sizes
  const sizes = (
    <Row gap={8} style={{ alignItems: "flex-end" }}>
      <Button label="Extra small" size="xsmall" onPress={() => report("xsmall")} />
      <Button label="Small" size="small" onPress={() => report("small")} />
      <Button label="Medium" size="medium" onPress={() => report("medium")} />
      <Button label="Large" size="large" onPress={() => report("large")} />
    </Row>
  );

  // section: Icons and states
  const iconsAndStates = (
    <Row gap={8}>
      <Button label="Save" icon="check" variant="primary" onPress={() => report("Save")} />
      <Button label="Next" suffixIcon="chevron-right" onPress={() => report("Next")} />
      <Button label="Selected" selected onPress={() => report("Selected")} />
      <Button label="Loading" loading />
      <Button label="Disabled" disabled />
      <Button label="Icon only" icon="plus" accessibilityLabel="Add" onPress={() => report("Add")} />
    </Row>
  );

  // section: Button group
  const group = (
    <Row gap={0}>
      <Button label="Day" groupEnd onPress={() => report("Day")} />
      <Button label="Week" groupStart groupEnd onPress={() => report("Week")} />
      <Button label="Month" groupStart onPress={() => report("Month")} />
    </Row>
  );

  // section: Progress
  const progress = (
    <Row gap={8}>
      <Button
        label={syncing() ? "Syncing…" : "Sync"}
        icon={syncing() ? "loader-circle" : "refresh-cw"}
        loading={syncing()}
        variant="primary"
        onPress={() => {
          syncing.set(true);
          setTimeout(() => syncing.set(false), 1400);
        }}
      />
      <Caption>A control that is working is visibly busy and refuses input.</Caption>
    </Row>
  );

  // section: Dropdown
  const dropdown = (
    <Row gap={8}>
      <Button label="Actions" suffixIcon="chevron-down" onPress={() => report("Actions menu")} />
      <Button label="More" icon="ellipsis" variant="secondary" onPress={() => report("More")} />
    </Row>
  );

  // section: Custom size
  const customSize = (
    <Row gap={8}>
      <Button label="Wide" width={220} onPress={() => report("Wide")} />
      <Button label="Pill" rounded={999} variant="primary" onPress={() => report("Pill")} />
      <Button label="Full width" width="full" variant="secondary" onPress={() => report("Full width")} />
    </Row>
  );

  // section: Custom color
  const customColor = (
    <Row gap={8}>
      <Button
        label="Indigo"
        style={{ backgroundColor: tokens.info, borderColor: tokens.info }}
        onPress={() => report("Indigo")}
      >
        <Text style={{ fontSize: 14, fontWeight: "500", color: "#ffffff" }}>Indigo</Text>
      </Button>
      <Button
        label="Sunset"
        style={{ borderColor: tokens.warning, backgroundColor: tokens.warning }}
        onPress={() => report("Sunset")}
      >
        <Text style={{ fontSize: 14, fontWeight: "500", color: "#171717" }}>Sunset</Text>
      </Button>
    </Row>
  );

  // section: Icon only
  const iconOnly = (
    <Row gap={8}>
      <Button icon="plus" accessibilityLabel="Add" onPress={() => report("Add")} />
      <Button icon="pencil" variant="secondary" accessibilityLabel="Edit" onPress={() => report("Edit")} />
      <Button icon="trash" variant="danger" accessibilityLabel="Delete" onPress={() => report("Delete")} />
      <Button icon="settings" variant="ghost" accessibilityLabel="Settings" onPress={() => report("Settings")} />
    </Row>
  );

  // section: Loading State with Icons
  const loadingWithIcons = (
    <Row gap={8}>
      <Button label="Uploading" icon="upload" loading variant="primary" />
      <Button label="Saving" suffixIcon="check" loading variant="secondary" />
      <Button label="Processing" loading variant="success" />
    </Row>
  );
  // section: Icon Types
  const iconTypes = (
    <Row gap={8}>
      <Button label="Download" icon="download" onPress={() => report("Download")} />
      <Button label="Continue" suffixIcon="arrow-right" variant="primary" onPress={() => report("Continue")} />
      <Button icon="heart" variant="secondary" accessibilityLabel="Favorite" onPress={() => report("Favorite")} />
      <Button label="Sparkles" icon="sparkles" variant="primary" onPress={() => report("Sparkles")} />
    </Row>
  );

  // section: Spinner Icon
  const spinnerIcon = (
    <Row gap={8}>
      <Button label="Syncing" loading variant="primary" />
      <Button label="Refreshing" icon="refresh-cw" loading variant="secondary" />
      <Button icon="loader-circle" loading accessibilityLabel="Working" />
    </Row>
  );

  // section: Dynamic Icon Updates
  const dynamicIcon = (
    <Row gap={12} style={{ alignItems: "center" }}>
      <Button
        label={playing() ? "Pause" : "Play"}
        icon={playing() ? "pause" : "play"}
        variant={playing() ? "secondary" : "primary"}
        onPress={() => playing.set(!playing())}
      />
      <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>
        {playing() ? "Playing the preview" : "Paused"}
      </Text>
    </Row>
  );

  // section: Toggle Button Group
  const toggleGroup = (
    <Row gap={0}>
      <Button label="Bold" groupEnd selected={pressed() === "Bold"} onPress={() => report("Bold")} />
      <Button label="Italic" groupStart groupEnd selected={pressed() === "Italic"} onPress={() => report("Italic")} />
      <Button label="Underline" groupStart selected={pressed() === "Underline"} onPress={() => report("Underline")} />
    </Row>
  );

  // section: Premium actions
  const premium = (
    <View
      style={{
        padding: 20,
        gap: 16,
        borderRadius: RADIUS_LG,
        borderWidth: 1,
        borderColor: alpha(tokens.primary, 0.12),
        backgroundImage: `linear-gradient(135deg, ${alpha(tokens.primary, 0.06)}, ${alpha(tokens.info, 0.12)})`,
        overflow: "hidden",
      }}
    >
      <View style={{ gap: 4 }}>
        <Text style={{ fontSize: 18, fontWeight: "600", color: tokens.foreground }}>
          Make it feel finished.
        </Text>
        <Text style={{ fontSize: 13, lineHeight: 19, color: tokens.mutedForeground }}>
          A focused action pair with a soft surface, a lift on hover and a quiet secondary path.
        </Text>
      </View>
      <Row gap={8}>
        <Button label="Upgrade" icon="sparkles" variant="primary" onPress={() => report("Upgrade")} />
        <Button label="Preview" variant="ghost" suffixIcon="external-link" onPress={() => report("Preview")} />
      </Row>
    </View>
  );

  // section: end

  return (
    <Page>
      <Caption>
        Every button carries its own message. The status line reports the last press so a
        click is visible in the gallery.
      </Caption>
      <ExampleSection index={0} title="Usage">
        {usage}
      </ExampleSection>
      <ExampleSection index={1} title="Variants">
        {variantRow}
      </ExampleSection>
      <ExampleSection index={2} title="Outline">
        {outline}
      </ExampleSection>
      <ExampleSection index={3} title="Sizes">
        {sizes}
      </ExampleSection>
      <ExampleSection index={4} title="Icons and states">
        {iconsAndStates}
      </ExampleSection>
      <ExampleSection index={5} title="Button group">
        {group}
      </ExampleSection>
      <ExampleSection index={6} title="Progress">
        {progress}
      </ExampleSection>
      <ExampleSection index={7} title="Dropdown">
        {dropdown}
      </ExampleSection>
      <ExampleSection index={8} title="Custom size">
        {customSize}
      </ExampleSection>
      <ExampleSection index={9} title="Custom color">
        {customColor}
      </ExampleSection>
      <ExampleSection index={10} title="Icon only">
        {iconOnly}
      </ExampleSection>
      <ExampleSection index={11} title="Loading State with Icons">
        {loadingWithIcons}
      </ExampleSection>
      <ExampleSection index={12} title="Icon Types">
        {iconTypes}
      </ExampleSection>
      <ExampleSection index={13} title="Spinner Icon">
        {spinnerIcon}
      </ExampleSection>
      <ExampleSection index={14} title="Dynamic Icon Updates">
        {dynamicIcon}
      </ExampleSection>
      <ExampleSection index={15} title="Toggle Button Group">
        {toggleGroup}
      </ExampleSection>
      <ExampleSection index={16} title="Premium actions">
        {premium}
      </ExampleSection>
      <Status>
        <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>{pressed()}</Text>
      </Status>
    </Page>
  );
}
