# bmo-app-sdk

The shared base of every BMO application. An application declares itself once, in a
`bmo-app.toml`, and this repository generates its testbed declaration, its install
declaration, its systemd units, its removal script and its web scaffold from that one file,
while its binary stands on the service skeleton crate here for the answers, the serving loop
and the health checks. What an application is — its domain, its data, its words — stays its
own.

The documents state the whole: `docs/design.md` (what is shared and what stays per
application), `docs/manifest.md` (the manifest and its readings), `docs/install.md`
(installing), `docs/removal.md` (removing), `docs/service.md` (the Rust skeleton) and
`docs/web.md` (the Nuxt defaults).

## Folders

| Folder | What it holds |
|---|---|
| `crates/service` | The library `bmo-app-service`: the answer envelope, the serving loop, the health checks |
| `crates/declare` | The generator binary `bmo-app-declare`: an application's manifest read, its declarations written |
| `templates/` | The text the units, the removal script, the web check and the web defaults are made of |
| `bin/` | The scaffold that stands a new application up and the generator's build |
| `docs/` | The base's own documents: design, manifest, install, removal, service, web |

## This repository's own checks

Its suite is Cargo's, run through the testbed's queue: `bmo check` for a change,
`bmo-testbed cargo test` for the whole. It deploys nothing and installs nothing: a base
library is cloned by its consumers and built inside their builds.

## Starting a new application

From a checkout of this repository, with the target standing beside it (sibling checkouts are
how applications and this base sit side by side):

```
bmo-app-sdk/bin/new-app.sh --name <name> --port <service-port> \
  [--pages-port <n> --dev-port <n> --public-url <url>] [--description <text>] \
  ../<name>
```

It stands the checkout up with the manifest, the generated declarations, the units, the
removal script and a binary that serves and answers health, ready for the application's own
domain. Then, in order:

1. **Write the domain.** Routes and stores in `src/`, every answer through
   `bmo_app_service::answer`; the web app under `web/`, its config composed with
   `withAppDefaults` (`docs/web.md`). `docs/service.md` states the binary's shape the units
   expect.
2. **Check it.** `bmo check` inside the new checkout; its own suite follows the testbed's
   declaration that was generated for it.
3. **Give it a remote** under the organisation and push `main`.
4. **Say where it stands:** add its path to the machine's checkout register
   (the data root's `testbed/checkouts.toml`), and its row to the set index's `presets.toml`
   with the folder of the index it stands in, linking that folder to the checkout when the
   checkout itself stands elsewhere (`docs/install.md`).
5. **Register it with the agency:** `bmo repository add <name> --path <checkout>`, and set
   its label, preamble, rules and dev command on the console's repository tab.
6. **Ask for its install and its first release:** `bmo install --add <name> --dry-run` shows
   the plan whole; the install and the deployment are the supervisor's, on the owner's word.

To change any declaration later, change the manifest and run
`bmo-app-declare bmo-app.toml` in the application's checkout (built by
`bmo-app-sdk/bin/build-generator.sh`), and commit the manifest with its readings.
