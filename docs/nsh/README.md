# Native Shell documentation (vendored)

This directory holds the Native Shell documentation from <https://nsh.dev>, so
coding agents can read it without network access and a page cannot disappear
from the repository unnoticed.

- `llms.txt` — the entry point and index from <https://nsh.dev/llms.txt>.
- `llms-full.txt` — the same documentation in one file, from
  <https://nsh.dev/llms-full.txt>.
- `docs/`, `elements/`, `components/`, `modules/` — the English pages linked by
  `llms.txt`, at the paths the site serves (`docs.md`, `docs/cli.md`, ...).
- `zh-CN/` — the Chinese pages `llms.txt` links to, at the same relative paths.
- `SOURCE.json` — the upstream origin, the NSH release the repository pins and
  the vendoring date.

Re-download after bumping the pinned NSH release:

```sh
node script/vendor-nsh-docs.mjs          # write docs/nsh
node script/vendor-nsh-docs.mjs --check  # fail if a linked page is missing
```

The pages are third-party prose and are **not** relicensed by this repository;
they remain copyright their authors. They are included for reference, with the
source and license note above. The code in `crates/nsh-component` is this
repository's own work under the repository's Apache-2.0 license.

Start with [conventions.md](conventions.md), then use `llms.txt` to find the
module or element page you need.
