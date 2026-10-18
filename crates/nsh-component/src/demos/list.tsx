//! `list` — rows with the mobile ListView's recycling and dividers.
import { List, ListRow } from "../components/list";
import { Tag } from "../components/tag";
import { Caption, ExampleSection, Page } from "./ui";

const items = Array.from({ length: 20 }, (_, index) => ({ title: `Item ${index + 1}`, description: `Row ${index + 1}` }));

// section: Basic
const basic = <List items={items} renderItem={(item) => <ListRow title={item.title} description={item.description} />} style={{ height: 320 }} />;
// section: Inset
const inset = <List items={items.slice(0, 6)} variant="inset" renderItem={(item) => <ListRow title={item.title} />} style={{ height: 280 }} />;
// section: With trailing
const trailing = <List items={items.slice(0, 5)} renderItem={(item) => <ListRow title={item.title} trailing={<Tag label="New" variant="success" />} />} style={{ height: 260 }} />;
// section: end
export function Demo() {
  return (
    <Page>
      <Caption>List is the styled row list; VirtualList is the raw recycling List from nsh/list.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Inset">{inset}</ExampleSection>
      <ExampleSection index={2} title="With trailing">{trailing}</ExampleSection>
    </Page>
  );
}
