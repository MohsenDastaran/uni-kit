// Framework-specific SEO pages reuse the component catalogue, but each
// framework gets its own stable URL, title and landing copy. Keeping the
// metadata here prevents a page, a sitemap entry and a framework switch from
// disagreeing about names or paths.
import { componentPages, type SidebarItem } from './sidebar';
import { hasIcedPage } from './remark-iced-source.js';
import { hasNshPage } from './remark-nsh-source.js';
import { hasSlintPage } from './remark-slint-source.js';

export const FRAMEWORK_KEYS = ['gpui', 'slint', 'iced', 'nsh'] as const;
export type FrameworkKey = (typeof FRAMEWORK_KEYS)[number];
export const DEFAULT_FRAMEWORK: FrameworkKey = 'nsh';

export interface FrameworkInfo {
  name: string;
  slug: FrameworkKey;
  /** The phrase search results should associate with this framework. */
  seoTitle: string;
  seoDescription: string;
  tagline: string;
  description: string;
  platforms: string[];
  docs: string;
  repo: string;
  /** The package the registry installs under. */
  installFramework: FrameworkKey;
}

export const FRAMEWORKS_INFO: Record<FrameworkKey, FrameworkInfo> = {
  gpui: {
    name: 'GPUI',
    slug: 'gpui',
    seoTitle: 'GPUI component library — native Rust desktop UI',
    seoDescription:
      'Copy-paste GPUI components for Rust desktop apps: buttons, forms, data tables, charts, dialogs and full application screens, built for macOS, Windows and Linux.',
    tagline: 'Native Rust desktop components',
    description:
      'GPUI components for data-dense Rust desktop applications. The same catalogue is also available as source you own, with macOS, Windows and Linux targets.',
    platforms: ['macOS', 'Windows', 'Linux'],
    docs: 'https://gpui.rs',
    repo: 'https://github.com/zed-industries/zed',
    installFramework: 'gpui',
  },
  slint: {
    name: 'Slint',
    slug: 'slint',
    seoTitle: 'Slint component library — Rust and embedded UI components',
    seoDescription:
      'Copy-paste Slint components for Rust applications: forms, navigation, data tables, charts and application shells, with native and WebAssembly targets.',
    tagline: 'Declarative UI for Rust and embedded targets',
    description:
      'Slint components written for native Rust applications and the browser. Browse the catalog, run each live example, and copy the .slint source into your own app.',
    platforms: ['macOS', 'Windows', 'Linux', 'Web', 'Embedded'],
    docs: 'https://slint.dev',
    repo: 'https://github.com/slint-ui/slint',
    installFramework: 'slint',
  },
  iced: {
    name: 'Iced',
    slug: 'iced',
    seoTitle: 'Iced component library — Rust GUI components',
    seoDescription:
      'A copy-paste Iced component library for Rust: inputs, tables, navigation, charts, dialogs and composed application screens, written against iced 0.14.',
    tagline: 'Elm-inspired Rust GUI components',
    description:
      'Iced components for Rust desktop applications. Each example is live, the source is copy-paste, and the controls are built from the same design tokens as the rest of uni-kit.',
    platforms: ['macOS', 'Windows', 'Linux', 'Web'],
    docs: 'https://iced.rs',
    repo: 'https://github.com/iced-rs/iced',
    installFramework: 'iced',
  },
  nsh: {
    name: 'NSH',
    slug: 'nsh',
    seoTitle: 'NSH component library — shadcn-style TSX for Native Shell',
    seoDescription:
      'An NSH component library in TSX, inspired by shadcn/ui: copy-paste components for Native Shell apps on iOS, Android, macOS, Windows, Linux, GPUI and the web.',
    tagline: 'shadcn-style TSX components for Native Shell',
    description:
      'NSH components in TypeScript/TSX for Native Shell. One source tree runs across iOS, Android, macOS, Windows, Linux, GPUI and the web, with copy-paste source and live examples.',
    platforms: ['iOS', 'Android', 'macOS', 'Windows', 'Linux', 'GPUI', 'Web'],
    docs: 'https://nsh.dev/',
    repo: 'https://github.com/longbridge/nsh',
    installFramework: 'nsh',
  },
};

/** The frameworks a component entry actually supports, from its source pages. */
export function availableFrameworks(
  slug: string,
  example: string | false | undefined,
): FrameworkKey[] {
  return FRAMEWORK_KEYS.filter(
    (framework) =>
      framework === 'gpui' ||
      (framework === 'slint' && hasSlintPage(slug, example)) ||
      (framework === 'iced' && hasIcedPage(slug, example)) ||
      (framework === 'nsh' && hasNshPage(slug, example)),
  );
}

export const frameworkName = (framework: string): string =>
  FRAMEWORKS_INFO[framework as FrameworkKey]?.name ?? framework;

export const frameworkLandingPath = (framework: FrameworkKey): string =>
  `/frameworks/${framework}`;

export const frameworkComponentPath = (
  framework: FrameworkKey,
  slug: string,
): string => `${frameworkLandingPath(framework)}/${slug}`;

/** A framework page's sidebar, with every catalog link kept inside it. */
export function frameworkSidebar(
  sidebar: SidebarItem[],
  framework: FrameworkKey,
): SidebarItem[] {
  const rewrite = (item: SidebarItem): SidebarItem | null => {
    if (
      item.frameworkAvailable &&
      !item.frameworkAvailable.split(/\s+/).includes(framework)
    ) {
      return null;
    }
    const items = item.items
      ?.map(rewrite)
      .filter((child): child is SidebarItem => child !== null);
    if (item.items && items?.length === 0 && !item.link) return null;
    return {
      ...item,
      link: item.link
        ? item.link === '/component'
          ? frameworkLandingPath(framework)
          : item.link.startsWith('/component/')
            ? frameworkComponentPath(
                framework,
                item.link.slice('/component/'.length),
              )
            : item.link
        : undefined,
      items,
    };
  };
  return sidebar
    .map(rewrite)
    .filter((item): item is SidebarItem => item !== null);
}

/** Every component URL a framework supports, in sidebar order. */
export function frameworkComponentPages(
  sidebar: SidebarItem[],
  framework: FrameworkKey,
): { slug: string; title: string; path: string }[] {
  return componentPages(sidebar)
    .filter(
      (page) =>
        !page.frameworkAvailable ||
        page.frameworkAvailable.split(/\s+/).includes(framework),
    )
    .map((page) => ({
      slug: page.link?.slice('/component/'.length) ?? '',
      title: page.text,
      path: frameworkComponentPath(
        framework,
        page.link?.slice('/component/'.length) ?? '',
      ),
    }))
    .filter((page) => page.slug);
}
