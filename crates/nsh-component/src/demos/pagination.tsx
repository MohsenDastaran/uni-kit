//! `pagination` — page navigation.
import { signal } from "nsh";
import { Pagination } from "../components/pagination";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const page = signal(1);
  // section: Basic
  const basic = <Pagination page={page()} totalPages={10} onPageChange={(next) => page.set(next)} />;
  // section: Siblings
  const siblings = <Pagination page={page()} totalPages={20} siblingCount={2} onPageChange={(next) => page.set(next)} />;
  // section: Disabled
  const disabled = <Pagination page={1} totalPages={1} />;
  // section: end
  return (
    <Page>
      <Caption>Pagination is a controlled page index; the list it pages is the application's.</Caption>
      <ExampleSection index={0} title="Basic">{basic}</ExampleSection>
      <ExampleSection index={1} title="Siblings">{siblings}</ExampleSection>
      <ExampleSection index={2} title="Disabled">{disabled}</ExampleSection>
      <Status>Page {page()}</Status>
    </Page>
  );
}
