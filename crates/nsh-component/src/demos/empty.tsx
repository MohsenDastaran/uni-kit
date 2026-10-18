//! `empty` — composed empty states: media, text, actions and variants.
import { Button } from "../components/button";
import { Empty } from "../components/empty";
import { Avatar, AvatarGroup } from "../components/avatar";
import { Input } from "../components/input";
import { Caption, ExampleSection, Page, Row } from "./ui";

const action = <Button label="Create project" variant="primary" onPress={() => {}} />;

// section: Basic usage
const basic = <Empty icon="inbox" title="No messages" description="Messages you receive will appear here." action={action} />;
// section: Outline
const outline = <Empty variant="outline" icon="search" title="No results" description="Try a different search term." />;
// section: Background
const background = <Empty variant="background" icon="file" title="No files yet" description="Upload a file to get started." />;
// section: Avatar
const avatar = <Empty media={<Avatar name="Ada Lovelace" size="xl" />} title="No profile" description="Choose a profile to continue." />;
// section: Avatar group
const avatarGroup = <Empty media={<AvatarGroup items={[{ name: "Ada" }, { name: "Grace" }, { name: "Alan" }]} max={2} />} title="No team yet" description="Invite people to collaborate." />;
// section: Inputs and custom content
const custom = (
  <Empty title="Search" description="Find a component in the catalog.">
    <Input placeholder="Search components" onChangeText={() => {}} style={{ width: 280 }} />
  </Empty>
);
// section: Constrained layouts
const constrained = <Empty variant="outline" icon="image" title="Nothing to preview" style={{ maxWidth: 320 }} />;
// section: end
export function Demo() {
  return (
    <Page>
      <Caption>An empty state explains what is missing and offers the next action.</Caption>
      <ExampleSection index={0} title="Basic usage">{basic}</ExampleSection>
      <ExampleSection index={1} title="Outline">{outline}</ExampleSection>
      <ExampleSection index={2} title="Background">{background}</ExampleSection>
      <ExampleSection index={3} title="Avatar">{avatar}</ExampleSection>
      <ExampleSection index={4} title="Avatar group">{avatarGroup}</ExampleSection>
      <ExampleSection index={5} title="Inputs and custom content">{custom}</ExampleSection>
      <ExampleSection index={6} title="Constrained layouts">{constrained}</ExampleSection>
    </Page>
  );
}
