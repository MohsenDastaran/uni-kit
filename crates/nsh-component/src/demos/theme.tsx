//! `theme` — the shared design tokens, shown as swatches.
import { Text, View } from "nsh";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

function Swatch(props: { label: string; value: () => string }) {
  return (
    <View style={{ alignItems: "center", gap: 4, width: 112 }}>
      <View
        style={{
          width: 112,
          height: 56,
          borderRadius: 6,
          backgroundColor: props.value(),
          borderWidth: 1,
          borderColor: tokens.border,
        }}
      />
      <Text style={{ fontSize: 12, color: tokens.foreground }}>{props.label}</Text>
      <Text style={{ fontSize: 11, color: tokens.mutedForeground, fontFamily: "monospace" }}>
        {props.value()}
      </Text>
    </View>
  );
}

export function Demo() {
  // section: Gradient Backgrounds
  const gradients = (
    <View style={{ gap: 8 }}>
      <Text style={{ fontSize: 14, color: tokens.foreground }}>
        The NSH components use flat token colours; gradients are an application choice.
      </Text>
      <View style={{ flexDirection: "row", gap: 8 }}>
        <View style={{ width: 120, height: 56, borderRadius: 6, backgroundImage: "linear-gradient(135deg, #54aeff, #8250df)" }} />
        <View style={{ width: 120, height: 56, borderRadius: 6, backgroundImage: "linear-gradient(135deg, #22c55e, #eab308)" }} />
        <View style={{ width: 120, height: 56, borderRadius: 6, backgroundImage: "linear-gradient(135deg, #f87171, #facc15)" }} />
      </View>
    </View>
  );
  // section: Theme Registry
  const registry = (
    <Row gap={12} style={{ flexWrap: "wrap" }}>
      <Swatch label="background" value={() => tokens.background} />
      <Swatch label="foreground" value={() => tokens.foreground} />
      <Swatch label="primary" value={() => tokens.primary} />
      <Swatch label="secondary" value={() => tokens.secondary} />
      <Swatch label="muted" value={() => tokens.muted} />
      <Swatch label="accent" value={() => tokens.accent} />
      <Swatch label="danger" value={() => tokens.danger} />
      <Swatch label="info" value={() => tokens.info} />
      <Swatch label="success" value={() => tokens.success} />
      <Swatch label="warning" value={() => tokens.warning} />
      <Swatch label="border" value={() => tokens.border} />
      <Swatch label="ring" value={() => tokens.ring} />
    </Row>
  );
  // section: end
  return (
    <Page>
      <Caption>
        The same tokens the GPUI, Slint and iced ports use; theme.ts exposes them as getters
        over the appearance, and theme.json gives nsh/ui/mobile the same palette.
      </Caption>
      <ExampleSection index={0} title="Gradient Backgrounds">{gradients}</ExampleSection>
      <ExampleSection index={1} title="Theme Registry">{registry}</ExampleSection>
    </Page>
  );
}
