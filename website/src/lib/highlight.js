import { createHighlighter } from 'shiki';
import { shikiConfig } from './markdown.js';

// `<Code>` renders a component, so its output cannot be handed to a Vue tab as
// a string. The Blocks pages need that, so they highlight here with the same
// themes and grammar the markdown pipeline uses -- a stock theme would be the
// one code sample on the site with different colours.
const { themes, defaultColor, langs } = shikiConfig;

const slint = langs.find((entry) => typeof entry === 'object');

let highlighter;
async function get() {
  highlighter ??= await createHighlighter({
    themes: [themes.light, themes.dark],
    langs: ['rust', slint],
  });
  return highlighter;
}

/** Highlighted HTML for a snippet, themed like every other code block. */
export async function highlight(code, lang = 'rust') {
  if (!code) return '';
  const instance = await get();
  const html = instance.codeToHtml(code.trim(), { lang, themes, defaultColor });
  // Astro's own renderer marks its output `astro-code`, and that is the class
  // the stylesheet switches for dark mode. Shiki knows nothing about it.
  return html.replace('<pre class="shiki', '<pre class="astro-code shiki');
}
