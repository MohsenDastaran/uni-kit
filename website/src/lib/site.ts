// The site's origin, without a trailing slash.
//
// One source of truth: the `site` in `astro.config.mjs`, which Astro exposes as
// `import.meta.env.SITE`. It was a second copy of the domain, and the copy still
// said `gpui-kit.com` — which is what put the old domain in `canonical`,
// `og:url`, `og:image`, the sitemap and robots.txt long after the site had been
// renamed. The literal is the fallback for a context without the env.
const configured = import.meta.env?.SITE ?? 'https://uni-kit.dastaran.com';

export const SITE_URL = configured.replace(/\/+$/, '');
