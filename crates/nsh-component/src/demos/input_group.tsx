//! `input-group` — an input framed with text, icons, selects, keyboard hints
//! and actions, in the ReUI input-group patterns.
import { signal, Text, View } from "nsh";
import { Avatar } from "../components/avatar";
import { Button } from "../components/button";
import { Icon } from "../components/icon";
import {
  InputGroup,
  InputGroupAddon,
  InputGroupButton,
  InputGroupInput,
  InputGroupSelect,
  InputGroupText,
} from "../components/input_group";
import { Kbd } from "../components/kbd";
import { Spinner } from "../components/spinner";
import { Textarea } from "../components/textarea";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

const width = { width: 340, maxWidth: "100%" } as const;

export function Demo() {
  const amount = signal("0.00");
  const value = signal("shadcn");
  const search = signal("");
  const phone = signal("123 456 7890");
  const notes = signal("");
  const member = signal("shadcn");
  const query = signal("");

  // section: Usage
  const usage = (
    <InputGroup style={width}>
      <InputGroupText>$</InputGroupText>
      <InputGroupInput value={amount()} onChangeText={(next) => amount.set(next)} placeholder="0.00" />
      <InputGroupText>USD</InputGroupText>
    </InputGroup>
  );

  // section: Input with a clear button
  const clear = (
    <InputGroup style={width}>
      <InputGroupAddon tone="plain"><Icon name="search" size={13} color={tokens.mutedForeground} /></InputGroupAddon>
      <InputGroupInput value={value()} placeholder="Search…" onChangeText={(next) => value.set(next)} />
      {value() ? <InputGroupButton icon="x" accessibilityLabel="Clear" onPress={() => value.set("")} /> : null}
    </InputGroup>
  );

  // section: Parts and alignment
  const parts = (
    <InputGroup style={width}>
      <InputGroupText>https://</InputGroupText>
      <InputGroupInput placeholder="example.com" />
      <InputGroupText>.dev</InputGroupText>
    </InputGroup>
  );

  // section: Search with a category dropdown
  const category = (
    <InputGroup style={width}>
      <InputGroupInput value={search()} placeholder="Search products" onChangeText={(next) => search.set(next)} />
      <InputGroupSelect
        value="category"
        options={[
          { value: "category", label: "Category" },
          { value: "products", label: "Products" },
          { value: "orders", label: "Orders" },
        ]}
        onChange={() => {}}
        style={{ flexGrow: 0, width: 104, flexShrink: 0 }}
      />
    </InputGroup>
  );

  // section: Buttons, icons, and menus
  const iconAction = (
    <InputGroup style={width}>
      <InputGroupInput value="https://reui.com/share" readOnly />
      <InputGroupButton icon="copy" accessibilityLabel="Copy URL" onPress={() => {}} />
    </InputGroup>
  );

  // section: Phone with country code
  const country = (
    <InputGroup style={width}>
      <InputGroupSelect
        value="1"
        options={[{ value: "1", label: "+1" }, { value: "44", label: "+44" }, { value: "49", label: "+49" }]}
        onChange={() => {}}
        style={{ flexGrow: 0, width: 76, flexShrink: 0 }}
      />
      <InputGroupInput value={phone()} onChangeText={(next) => phone.set(next)} keyboardType="phone-pad" />
      <InputGroupText>✆</InputGroupText>
    </InputGroup>
  );

  // section: Search keyboard shortcut
  const keyboard = (
    <InputGroup style={width}>
      <InputGroupAddon tone="plain"><Icon name="search" size={13} color={tokens.mutedForeground} /></InputGroupAddon>
      <InputGroupInput placeholder="Search documentation…" />
      <Kbd label="⌘K" size="xsmall" />
    </InputGroup>
  );

  // section: Number with info
  const info = (
    <InputGroup style={width}>
      <InputGroupInput value="5000" keyboardType="numeric" onChangeText={() => {}} />
      <InputGroupAddon tone="plain"><Icon name="info" size={13} color={tokens.mutedForeground} /></InputGroupAddon>
    </InputGroup>
  );

  // section: Loading spinner
  const loading = (
    <InputGroup style={width}>
      <InputGroupInput value="shadcn_ui" disabled />
      <Spinner size="xsmall" color={tokens.mutedForeground} />
    </InputGroup>
  );

  // section: Textarea with a counter and submit action
  const textarea = (
    <View style={{ width: 360, maxWidth: "100%", gap: 5 }}>
      <InputGroup multiline style={{ width: 360 }}>
        <Textarea
          value={notes()}
          onChangeText={(next) => notes.set(next)}
          placeholder="Description…"
          rows={3}
          appearance={false}
          style={{ flex: 1 }}
        />
      </InputGroup>
      <View style={{ flexDirection: "row", alignItems: "center" }}>
        <Text style={{ flex: 1, fontSize: 10, color: tokens.mutedForeground }}>{notes().length}/140 characters</Text>
        <Icon name="info" size={12} color={tokens.mutedForeground} />
      </View>
    </View>
  );

  // section: AI chat composer
  const composer = (
    <View style={{ width: 420, maxWidth: "100%", padding: 10, borderRadius: 12, borderWidth: 1, borderColor: tokens.border, backgroundColor: tokens.popover, gap: 12 }}>
      <Textarea placeholder="Ask AI anything…" rows={3} appearance={false} style={{ width: "100%" }} onChangeText={() => {}} />
      <View style={{ flexDirection: "row", alignItems: "center", gap: 6 }}>
        <InputGroupAddon tone="plain"><Icon name="plus" size={13} color={tokens.mutedForeground} /></InputGroupAddon>
        {[
          { icon: "mic" as const, label: "Voice" },
          { icon: "globe" as const, label: "Search" },
        ].map((item) => (
          <InputGroupAddon tone="plain"><Icon name={item.icon} size={13} color={tokens.mutedForeground} /><Text style={{ fontSize: 10, color: tokens.mutedForeground }}>{item.label}</Text></InputGroupAddon>
        ))}
        <View style={{ flex: 1 }} />
        <InputGroupSelect
          value="gpt"
          options={[{ value: "gpt", label: "GPT-4o" }, { value: "claude", label: "Claude" }]}
          onChange={() => {}}
          style={{ width: 94, flexGrow: 0 }}
        />
        <Button icon="arrow-up" size="xsmall" variant="primary" accessibilityLabel="Send" onPress={() => {}} />
      </View>
    </View>
  );

  // section: Member row
  const memberRow = (
    <InputGroup style={width}>
      <Avatar name={member()} size="xs" />
      <InputGroupInput value={member()} onChangeText={(next) => member.set(next)} />
      <InputGroupSelect
        value="admin"
        options={[{ value: "admin", label: "Admin" }, { value: "editor", label: "Editor" }, { value: "viewer", label: "Viewer" }]}
        onChange={() => {}}
        style={{ flexGrow: 0, width: 90, flexShrink: 0 }}
      />
    </InputGroup>
  );

  // section: Label usage
  const label = (
    <View style={{ width: 360, gap: 6 }}>
      <Text style={{ fontSize: 11, fontWeight: "500", color: tokens.foreground }}>Search Components</Text>
      <InputGroup style={{ width: 360 }}>
        <InputGroupAddon tone="plain"><Icon name="search" size={13} color={tokens.mutedForeground} /></InputGroupAddon>
        <InputGroupInput value={query()} placeholder="Search…" onChangeText={(next) => query.set(next)} />
      </InputGroup>
    </View>
  );

  // section: Sizes and custom styles
  const sizes = (
    <View style={{ gap: 8, width: 340 }}>
      <InputGroup size="small" style={{ width: 300 }}>
        <InputGroupText size="small">$</InputGroupText>
        <InputGroupInput size="small" placeholder="0.00" />
        <InputGroupText size="small">USD</InputGroupText>
      </InputGroup>
      <InputGroup size="large" style={{ width: 380 }}>
        <InputGroupText size="large">$</InputGroupText>
        <InputGroupInput size="large" placeholder="0.00" />
        <InputGroupText size="large">USD</InputGroupText>
      </InputGroup>
    </View>
  );

  // section: Disabled, read-only, and validation
  const disabled = (
    <View style={{ gap: 8, width: 340 }}>
      <InputGroup disabled style={{ width: 320 }}>
        <InputGroupInput placeholder="Disabled" disabled />
      </InputGroup>
      <InputGroup invalid style={{ width: 320 }}>
        <InputGroupInput value="not-an-email" onChangeText={() => {}} />
      </InputGroup>
    </View>
  );
  // section: end

  return (
    <Page>
      <Caption>
        InputGroup draws one frame around standard controls. Addons carry a prefix,
        suffix, selector or action without changing the inner Input.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Input with a clear button">{clear}</ExampleSection>
      <ExampleSection index={2} title="Parts and alignment">{parts}</ExampleSection>
      <ExampleSection index={3} title="Search with a category dropdown">{category}</ExampleSection>
      <ExampleSection index={4} title="Buttons, icons, and menus">{iconAction}</ExampleSection>
      <ExampleSection index={5} title="Phone with country code">{country}</ExampleSection>
      <ExampleSection index={6} title="Search keyboard shortcut">{keyboard}</ExampleSection>
      <ExampleSection index={7} title="Number with info">{info}</ExampleSection>
      <ExampleSection index={8} title="Loading spinner">{loading}</ExampleSection>
      <ExampleSection index={9} title="Textarea with a counter and submit action">{textarea}</ExampleSection>
      <ExampleSection index={10} title="AI chat composer">{composer}</ExampleSection>
      <ExampleSection index={11} title="Member row">{memberRow}</ExampleSection>
      <ExampleSection index={12} title="Label usage">{label}</ExampleSection>
      <ExampleSection index={13} title="Sizes and custom styles">{sizes}</ExampleSection>
      <ExampleSection index={14} title="Disabled, read-only, and validation">{disabled}</ExampleSection>
    </Page>
  );
}
