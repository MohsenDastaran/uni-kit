//! `card` — a bordered surface with header, body, footer and cover.
//!
//! The `// section:` markers are the website's contract: one per heading on the
//! card page, in order, closed by `// section: end`.
import { signal, Text, View } from "nsh";
import cover from "../../examples/assets/cover.png";
import { Button } from "../components/button";
import { Card } from "../components/card";
import { Input } from "../components/input";
import { RADIUS_MD, tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const email = signal("");
  const password = signal("");
  const action = signal("No action yet");

  // section: Default
  const normal = (
    <Card
      title="Project update"
      description="The weekly summary is ready to review."
      badge="New"
    >
      <Text style={{ fontSize: 14, color: tokens.foreground }}>
        You walked 42 km this week, eight more than last week.
      </Text>
    </Card>
  );

  // section: Stacked
  const stacked = (
    <View style={{ gap: 12 }}>
      <Card title="Design system" description="Tokens, controls and documentation.">
        <Text style={{ fontSize: 14, color: tokens.foreground }}>Shared by every framework.</Text>
      </Card>
      <Card title="Native shell" description="The same controls written in TSX." badge="Draft">
        <Text style={{ fontSize: 14, color: tokens.foreground }}>Runs on every NSH platform.</Text>
      </Card>
    </View>
  );

  // section: Login
  const login = (
    <Card
      title="Welcome back"
      description="Sign in to continue to your workspace."
      footer={
        <>
          <Button label="Cancel" variant="ghost" onPress={() => action.set("Cancelled")} />
          <Button label="Sign in" variant="primary" onPress={() => action.set("Signed in")} />
        </>
      }
    >
      <View style={{ gap: 10 }}>
        <Input
          value={email()}
          placeholder="Email"
          prefixIcon="mail"
          onChangeText={(next) => email.set(next)}
        />
        <Input
          value={password()}
          placeholder="Password"
          prefixIcon="lock"
          masked
          maskToggle
          onChangeText={(next) => password.set(next)}
        />
      </View>
    </Card>
  );

  // section: Size
  const size = (
    <View style={{ gap: 12 }}>
      <Card size="small" title="Small card" description="A tighter inset and title.">
        <Text style={{ fontSize: 14, color: tokens.foreground }}>Small is 12 pt of padding.</Text>
      </Card>
      <Card size="medium" title="Medium card" description="The default inset.">
        <Text style={{ fontSize: 14, color: tokens.foreground }}>Medium is 16 pt of padding.</Text>
      </Card>
    </View>
  );

  // section: Spacing
  const spacing = (
    <View style={{ gap: 12 }}>
      <Card title="Inset 8" inset={8}>
        <Text style={{ fontSize: 14, color: tokens.foreground }}>A compact card.</Text>
      </Card>
      <Card title="Inset 24" inset={24}>
        <Text style={{ fontSize: 14, color: tokens.foreground }}>A roomy card.</Text>
      </Card>
    </View>
  );

  // section: Image
  const image = (
    <Card cover={cover} title="Mountain light" description="A cover image above the header.">
      <Text style={{ fontSize: 14, color: tokens.foreground }}>
        The cover is clipped by the card's rounded corner.
      </Text>
    </Card>
  );
  // section: end

  return (
    <Page>
      <Caption>
        Card is a presentational surface. The body is yours; the header, cover and footer
        are the parts the catalog standardises.
      </Caption>
      <ExampleSection index={0} title="Default">
        {normal}
      </ExampleSection>
      <ExampleSection index={1} title="Stacked">
        {stacked}
      </ExampleSection>
      <ExampleSection index={2} title="Login">
        {login}
      </ExampleSection>
      <ExampleSection index={3} title="Size">
        {size}
      </ExampleSection>
      <ExampleSection index={4} title="Spacing">
        {spacing}
      </ExampleSection>
      <ExampleSection index={5} title="Image">
        {image}
      </ExampleSection>
      <Status>
        <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>
          {action()} · radius {RADIUS_MD}pt
        </Text>
      </Status>
    </Page>
  );
}
