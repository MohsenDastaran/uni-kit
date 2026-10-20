//! `pagination` — the ReUI page navigation family: labels, numbers, ellipsis,
//! page info, rows-per-page, page selectors and go-to-page.
import { signal, Text } from "nsh";
import { Pagination } from "../components/pagination";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Status } from "./ui";

export function Demo() {
  const basic = signal(1);
  const pages = signal(1);
  const compact = signal(1);
  const sizes = signal(1);
  const events = signal(2);
  const ellipsis = signal(5);
  const outline = signal(2);
  const infoLeft = signal(1);
  const rows = signal(25);
  const rowsPage = signal(1);
  const selector = signal(4);
  const go = signal(2);
  const centered = signal(1);
  const carded = signal(1);
  const combo = signal(1);

  // section: Usage
  const usage = <Pagination page={basic()} totalPages={10} onPageChange={(next) => basic.set(next)} labels />;

  // section: Basic Pagination
  const basicPagination = (
    <Pagination page={pages()} totalPages={3} onPageChange={(next) => pages.set(next)} labels />
  );

  // section: Pages
  const pagesOnly = (
    <Pagination page={pages()} totalPages={3} onPageChange={(next) => pages.set(next)} />
  );

  // section: Compact Style
  const compactStyle = (
    <Pagination
      page={compact()}
      totalPages={10}
      onPageChange={(next) => compact.set(next)}
      labels
      showNumbers={false}
      showPageInfo
      pageInfoAt="start"
    />
  );

  // section: Different Sizes
  const differentSizes = (
    <Pagination
      page={sizes()}
      totalPages={10}
      onPageChange={(next) => sizes.set(next)}
      labels
      size="large"
      showNumbers
    />
  );

  // section: Disabled State
  const disabled = <Pagination page={1} totalPages={1} labels disabled />;

  // section: Handle Page Change Events
  const handleEvents = (
    <Pagination page={events()} totalPages={10} onPageChange={(next) => events.set(next)} labels showPageInfo />
  );

  // section: Ellipsis indicator
  const ellipsisIndicator = (
    <Pagination page={ellipsis()} totalPages={10} onPageChange={(next) => ellipsis.set(next)} labels />
  );

  // section: Outline style buttons
  const outlineStyle = (
    <Pagination page={outline()} totalPages={10} onPageChange={(next) => outline.set(next)} outline showNumbers />
  );

  // section: Page info on left
  const pageInfoLeft = (
    <Pagination
      page={infoLeft()}
      totalPages={10}
      onPageChange={(next) => infoLeft.set(next)}
      labels
      showPageInfo
      pageInfoAt="start"
    />
  );

  // section: Rows per page select with range info
  const rowsPerPage = (
    <Pagination
      page={rowsPage()}
      totalPages={4}
      totalItems={100}
      rowsPerPage={rows()}
      rowsPerPageOptions={[10, 25, 50, 100]}
      onRowsPerPageChange={(next) => rows.set(next)}
      onPageChange={(next) => rowsPage.set(next)}
      showNumbers={false}
      showFirstLast
      align="between"
    />
  );

  // section: Page select dropdown
  const pageSelect = (
    <Pagination
      page={selector()}
      totalPages={10}
      onPageChange={(next) => selector.set(next)}
      pageSelector
      showFirstLast
      showNumbers={false}
    />
  );

  // section: Numbered with go-to-page
  const goTo = (
    <Pagination page={go()} totalPages={10} onPageChange={(next) => go.set(next)} showNumbers goToPage />
  );

  // section: Page info on center
  const centeredInfo = (
    <Pagination
      page={centered()}
      totalPages={10}
      onPageChange={(next) => centered.set(next)}
      showNumbers={false}
      showPageInfo
      pageInfoAt="center"
    />
  );

  // section: Numbered pagination in card
  const cardedPagination = (
    <Pagination page={carded()} totalPages={12} onPageChange={(next) => carded.set(next)} card showNumbers />
  );

  // section: Page info, numbers and per-page select
  const comboPagination = (
    <Pagination
      page={combo()}
      totalPages={10}
      onPageChange={(next) => combo.set(next)}
      totalItems={100}
      rowsPerPage={10}
      rowsPerPageOptions={[10, 25, 50]}
      onRowsPerPageChange={() => {}}
      labels
      showPageInfo
      showNumbers
    />
  );

  // section: Custom styling
  const custom = (
    <Pagination
      page={centered()}
      totalPages={10}
      onPageChange={(next) => centered.set(next)}
      labels
      showPageInfo
      pageInfoAt="end"
      style={{ padding: 6, borderRadius: 8, backgroundColor: tokens.muted }}
    />
  );
  // section: end

  return (
    <Page>
      <Caption>
        Pagination is a controlled page index. The list it pages belongs to the
        application; the component owns the navigation shape and labels.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic Pagination">{basicPagination}</ExampleSection>
      <ExampleSection index={2} title="Pages">{pagesOnly}</ExampleSection>
      <ExampleSection index={3} title="Compact Style">{compactStyle}</ExampleSection>
      <ExampleSection index={4} title="Different Sizes">{differentSizes}</ExampleSection>
      <ExampleSection index={5} title="Disabled State">{disabled}</ExampleSection>
      <ExampleSection index={6} title="Handle Page Change Events">{handleEvents}</ExampleSection>
      <ExampleSection index={7} title="Ellipsis indicator">{ellipsisIndicator}</ExampleSection>
      <ExampleSection index={8} title="Outline style buttons">{outlineStyle}</ExampleSection>
      <ExampleSection index={9} title="Page info on left">{pageInfoLeft}</ExampleSection>
      <ExampleSection index={10} title="Rows per page select with range info">{rowsPerPage}</ExampleSection>
      <ExampleSection index={11} title="Page select dropdown">{pageSelect}</ExampleSection>
      <ExampleSection index={12} title="Numbered with go-to-page">{goTo}</ExampleSection>
      <ExampleSection index={13} title="Page info on center">{centeredInfo}</ExampleSection>
      <ExampleSection index={14} title="Numbered pagination in card">{cardedPagination}</ExampleSection>
      <ExampleSection index={15} title="Page info, numbers and per-page select">{comboPagination}</ExampleSection>
      <ExampleSection index={16} title="Custom styling">{custom}</ExampleSection>
      <Status>{events()} · page {basic()} of 10</Status>
    </Page>
  );
}
