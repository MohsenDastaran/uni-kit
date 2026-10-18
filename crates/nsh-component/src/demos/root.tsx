//! `root` — the application surface NSH's mount() provides.
import { Text, View } from "nsh";
import { Root } from "../components/root";
import { Button } from "../components/button";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  // section: Root View
  const view = (
    <View style={{ height: 200, borderRadius: 8, overflow: "hidden", borderWidth: 1, borderColor: tokens.border }}>
      <Root>
        <View style={{ flex: 1, alignItems: "center", justifyContent: "center", gap: 10 }}>
          <Text style={{ fontSize: 16, fontWeight: "600", color: tokens.foreground }}>Inside the root</Text>
          <Button label="An action" onPress={() => {}} />
        </View>
      </Root>
    </View>
  );
  // section: end
  return (
    <Page>
      <Caption>NSH's mount() renders the app into the window; Root gives the catalog a named surface that paints the page background and tokens.</Caption>
      <ExampleSection index={0} title="Root View">{view}</ExampleSection>
    </Page>
  );
}
