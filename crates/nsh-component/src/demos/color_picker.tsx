//! `color-picker` — the ReUI picker: saturation area, hue and opacity sliders,
//! a format field, featured swatches and the inline trigger. No modal.
import { signal, Text, View } from "nsh";
import { Button } from "../components/button";
import { ColorPicker, ColorSwatch, type ColorFormat } from "../components/color_picker";
import { Input } from "../components/input";
import { Badge } from "../components/badge";
import { Slider } from "../components/slider";
import { alpha, isDark, tokens } from "../components/theme";
import { Caption, Column, ExampleSection, Page, Row } from "./ui";

const normalizeHex = (value: string) => value.toUpperCase();
const isValidHex = (value: string) => /^#([0-9a-f]{3}|[0-9a-f]{6})$/i.test(value);

export function Demo() {
  const color = signal("#6366F1");
  const accent = signal("#0EA5E9");
  const canvas = signal("#F8FAFC");
  const overlay = signal("#171826");
  const opacity = signal(0.55);
  const format = signal<ColorFormat>("hex");
  const labelColor = signal("#8B5CF6");
  const labelName = signal("Needs design");
  const primary = signal("#2563EB");
  const lightValue = signal(62);
  const chroma = signal(0.214);
  const hue = signal(259.8);
  const brand = signal("#F97316");
  const palette = signal(["#EF4444", "#F97316", "#EAB308", "#22C55E", "#06B6D4", "#3B82F6", "#8B5CF6"]);
  const picked = signal<string | null>(null);

  const brandRow = (name: string, value: () => string, onPick: (next: string) => void) => (
    <Row gap={10} style={{ alignItems: "center" }}>
      <ColorSwatch color={value()} size={20} />
      <Text style={{ flex: 1, fontSize: 12, color: tokens.foreground }}>{name}</Text>
      <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>{value()}</Text>
      <Button label="Edit" size="xsmall" variant="ghost" onPress={() => onPick(value())} />
    </Row>
  );

  // section: Usage
  const usage = (
    <ColorPicker
      value={color()}
      onChange={(next) => color.set(next)}
      format={format()}
      onFormatChange={(next) => format.set(next)}
      style={{ width: 340 }}
    />
  );

  // section: Basic Color Picker
  const basic = (
    <ColorPicker
      value={color()}
      onChange={(next) => color.set(next)}
      format={format()}
      onFormatChange={(next) => format.set(next)}
      style={{ width: 340 }}
    />
  );

  // section: With Event Handling
  const events = (
    <Column gap={8} style={{ width: 340 }}>
      <ColorPicker value={color()} onChange={(next) => color.set(next)} style={{ width: 340 }} />
      <Badge label={`onChange: ${color()}`} variant="secondary" />
    </Column>
  );

  // section: Setting Default Color
  const defaults = (
    <ColorPicker
      defaultValue="#22C55E"
      onChange={(next) => color.set(next)}
      style={{ width: 340 }}
    />
  );

  // section: Different Sizes
  const sizes = (
    <Row gap={16} style={{ alignItems: "flex-start" }}>
      <ColorPicker value={color()} onChange={(next) => color.set(next)} size="small" showSpectrum={false} />
      <ColorPicker value={color()} onChange={(next) => color.set(next)} size="large" />
    </Row>
  );

  // section: With Custom Featured Colors
  const featured = (
    <ColorPicker
      value={color()}
      onChange={(next) => color.set(next)}
      featured={["#171717", "#EF4444", "#22C55E", "#3B82F6", "#8B5CF6"]}
      style={{ width: 340 }}
    />
  );

  // section: With Icon Instead of Color Square
  const iconTrigger = (
    <ColorPicker
      inline
      triggerIcon="palette"
      value={color()}
      onChange={(next) => color.set(next)}
      style={{ width: 320 }}
    />
  );

  // section: With Label
  const labelled = (
    <Column gap={6} style={{ width: 320 }}>
      <Text style={{ fontSize: 12, fontWeight: "500", color: tokens.foreground }}>Accent color</Text>
      <ColorPicker
        inline
        value={accent()}
        onChange={(next) => accent.set(next)}
        style={{ width: 320 }}
      />
      <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>
        Used for links, focus rings and primary buttons.
      </Text>
    </Column>
  );

  // section: Custom Anchor Position
  const anchor = (
    <Column gap={6} style={{ width: 320 }}>
      <Text style={{ fontSize: 12, fontWeight: "500", color: tokens.foreground }}>Canvas background</Text>
      <ColorPicker
        inline
        value={canvas()}
        onChange={(next) => canvas.set(next)}
        style={{ width: 320 }}
      />
    </Column>
  );

  // section: Theme Color
  const theme = (
    <Column gap={8} style={{ width: 340 }}>
      <ColorPicker value={tokens.primary} onChange={(next) => color.set(next)} showSpectrum={false} />
      <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>
        The picker opens on the theme's primary colour.
      </Text>
    </Column>
  );

  // section: Color Select
  const select = (
    <ColorPicker
      value={overlay()}
      onChange={(next) => overlay.set(next)}
      alpha={opacity()}
      onAlphaChange={(next) => opacity.set(next)}
      style={{ width: 340 }}
    />
  );

  // section: Color Theme Editor
  const themeEditor = (
    <Column gap={10} style={{ width: 360 }}>
      <Row gap={8} style={{ alignItems: "center" }}>
        <Text style={{ flex: 1, fontSize: 12, color: tokens.mutedForeground }}>Primary color</Text>
        <Badge label="primary" variant="outline" />
      </Row>
      {[
        { label: "L", value: lightValue, note: lightValue() },
        { label: "C", value: chroma, note: chroma() },
        { label: "H", value: hue, note: hue() },
      ].map((row) => (
        <Row gap={10}>
          <Text style={{ width: 12, fontSize: 11, color: tokens.mutedForeground }}>{row.label}</Text>
          <Slider value={row.note} onChange={(next) => row.value.set(next)} style={{ flex: 1 }} />
          <Text style={{ width: 42, fontSize: 11, textAlign: "right", color: tokens.foreground }}>{row.note}</Text>
        </Row>
      ))}
      <View
        style={{
          paddingHorizontal: 10,
          paddingVertical: 8,
          borderRadius: 6,
          borderWidth: 1,
          borderColor: tokens.border,
          backgroundColor: tokens.muted,
        }}
      >
        <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>--primary: oklch({lightValue()}% {chroma()} {hue})</Text>
      </View>
      <Row gap={6}>
        <ColorSwatch color={primary()} size={28} selected />
        <ColorSwatch color="#38BDF8" size={28} onPress={() => primary.set("#38BDF8")} />
        <ColorSwatch color="#22C55E" size={28} onPress={() => primary.set("#22C55E")} />
        <ColorSwatch color="#F59E0B" size={28} onPress={() => primary.set("#F59E0B")} />
      </Row>
    </Column>
  );

  // section: Brand Color Selector
  const brandSelector = (
    <Column gap={8} style={{ width: 340 }}>
      {brandRow("Primary", brand, () => {})}
      {brandRow("Accent", accent, () => {})}
      {brandRow("Background", canvas, () => {})}
      <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>
        Each value is the source of truth for one design token.
      </Text>
    </Column>
  );

  // section: Toolbar Color Picker
  const toolbar = (
    <Row gap={6} style={{ padding: 6, borderRadius: 10, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover }}>
      {["#171717", "#EF4444", "#F97316", "#EAB308", "#22C55E", "#3B82F6"].map((entry) => (
        <ColorSwatch
          color={entry}
          size={22}
          selected={normalizeHex(color()) === entry}
          onPress={() => color.set(entry)}
        />
      ))}
    </Row>
  );

  // section: Color Palette Builder
  const builder = (
    <Column gap={10} style={{ width: 360 }}>
      <Row gap={8} style={{ flexWrap: "wrap" }}>
        {palette().map((entry) => (
          <ColorSwatch
            color={entry}
            size={26}
            selected={picked() === entry}
            onPress={() => picked.set(entry)}
          />
        ))}
        <View
          accessibilityRole="button"
          accessibilityLabel="Add colour"
          onPress={() => palette.update((list) => [...list, color()])}
          style={{
            width: 26,
            height: 26,
            borderRadius: 13,
            borderWidth: 1,
            borderColor: tokens.border,
            alignItems: "center",
            justifyContent: "center",
          }}
        >
          <Text style={{ fontSize: 14, color: tokens.mutedForeground }}>+</Text>
        </View>
      </Row>
      <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>
        {picked() ? `Selected ${picked()} — press + to add ${color()}` : "Pick a swatch, then add the current colour."}
      </Text>
    </Column>
  );

  // section: With Color Validation
  const validation = (
    <Column gap={6} style={{ width: 340 }}>
      <ColorPicker value={color()} onChange={(next) => color.set(next)} showSpectrum={false} />
      <Text style={{ fontSize: 11, color: isValidHex(color()) ? tokens.success : tokens.danger }}>
        {isValidHex(color()) ? "A valid hex colour." : "Enter a hex colour such as #6366F1."}
      </Text>
    </Column>
  );
  // section: end

  return (
    <Page>
      <Caption>
        NSH has no platform colour dialog, so the picker is inline: a saturation area, sliders,
        a format field and swatches, with the value under the app's control.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic Color Picker">{basic}</ExampleSection>
      <ExampleSection index={2} title="With Event Handling">{events}</ExampleSection>
      <ExampleSection index={3} title="Setting Default Color">{defaults}</ExampleSection>
      <ExampleSection index={4} title="Different Sizes">{sizes}</ExampleSection>
      <ExampleSection index={5} title="With Custom Featured Colors">{featured}</ExampleSection>
      <ExampleSection index={6} title="With Icon Instead of Color Square">{iconTrigger}</ExampleSection>
      <ExampleSection index={7} title="With Label">{labelled}</ExampleSection>
      <ExampleSection index={8} title="Custom Anchor Position">{anchor}</ExampleSection>
      <ExampleSection index={9} title="Theme Color">{theme}</ExampleSection>
      <ExampleSection index={10} title="Color Select">{select}</ExampleSection>
      <ExampleSection index={11} title="Color Theme Editor">{themeEditor}</ExampleSection>
      <ExampleSection index={12} title="Brand Color Selector">{brandSelector}</ExampleSection>
      <ExampleSection index={13} title="Toolbar Color Picker">{toolbar}</ExampleSection>
      <ExampleSection index={14} title="Color Palette Builder">{builder}</ExampleSection>
      <ExampleSection index={15} title="With Color Validation">{validation}</ExampleSection>
    </Page>
  );
}
