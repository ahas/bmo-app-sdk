# The web defaults

- **What an application's web app starts from,** generated into its own tree so every build
  reads it complete (`docs/design.md`, on how reuse travels): a defaults module its
  `nuxt.config.ts` composes, and its check script.
- **Applications whose screen is a console bundle are the console family**
  (`docs/manifest.md`): the base generates their bundle's build and probe steps, not a web
  scaffold; their `client/` stays their own.

## The defaults module

- The generated `web/app-defaults.mjs` answers `appDefaults()` with the config every BMO
  application's Nuxt app holds alike: the UI and one Korean locale without a path prefix,
  the head block, the runtime config whose values the page server fills from `NUXT_*`
  environment, the fontless UI and the icon bundles, the strict types, the inline styles and
  the single-blob payload rule, the Bun preset and the dev proxy to the application's API.
- `withAppDefaults(own)` lays the application's own config over it key for key: an
  application names only what is its own, and everything unnamed stands as the default.
- What is the application's own and never a default: its css entry and its `app.config`
  theme, its pages and content, its locale words, and its route rules.

## The check script

- The generated `web/scripts/check.sh` is the whole of the web app's check: locked packages
  installed with the Chromium build its locked Playwright pins, types checked, the app built
  for the browser tests the testbed runs after it, and its unit tests run one file to a
  process. It reads the testbed's project check hook, the script named `check` in the
  application's own `package.json`.
- The application's `package.json` carries the script line that runs it, and its declared
  browser test files, the way the testbed's declaration document states.

## Wording

- Every string a person reads stands in the application's own locale file, in Korean, and
  the console's wording guide governs its terms; nothing in this repository writes Korean
  into a code file.
