# The application manifest: `bmo-app.toml`

- **One file at the application's root,** read by `bmo-app-declare`. It is where everything
  that differs between applications stands, once. Its generated readings go into the
  repository and are committed, because the testbed and the install verb read those files as
  the commit holds them; the manifest is the source an edit answers to.
- **It is TOML,** read against the schema below; a key the schema does not name is refused,
  naming it. Paths beginning `~/` read under the home directory, as everywhere on the
  platform; `/srv/` roots stand outside the home where a web server must read the tree.

## The families

An application belongs to one of three families, and the family is what it declares itself to
have:

- **A web half (`[web]`)**: a Nuxt app of its own, rendered by a page server unit and reached
  at a public URL. Its readings carry the site build and placement, the pages unit, the web
  check script and the Nuxt defaults.
- **A console half (`[console]`)**: a bundle the service itself serves into the platform's
  console. Its readings carry the bundle's project line and build, the console reach probe and
  the optional app publication, the settings-file unit and the install's tokens, settings and
  lines.
- **Neither**: the service alone.

At most one of `[web]` and `[console]` stands in a manifest; a manifest carrying both is
refused.

## The fields

| Key | Type | Meaning |
|---|---|---|
| `app` | integer | The major of this schema: `1` |
| `name` | string | The application's name, in the small alphabet: it becomes the unit names, the deploy binary, the agency repository row and the install port prefix |
| `description` | string | One English sentence, carried into unit descriptions and crate metadata |
| `env_prefix` | string | The capitals prefix its environment variables carry (`docs/service.md`) |
| `deploy_root` | path | Where its deploy tree's link and releases stand |
| `data_root` | path | Where its runtime data stands; for a console application this is the machine's shared data root, and its own data stands in `data_folder` under it |
| `service_port` | integer | The port the service answers on |
| `listen_host` | string | What the service binds, `127.0.0.1` where none is named |
| `public_url` | string | The address people reach the application at; required where a web app stands |
| `[web]` | table | The web half: `pages_port`, the loopback port its page server answers on, and `dev_port`, the port a person's dev server takes |
| `[console]` | table | The console half, its fields below |
| `service_env` | table | Extra `Environment` lines for the service unit, key to value; `~/` reads as the home |
| `pages_env` | table | Extra `Environment` lines for the page server unit, likewise |
| `[[input]]` | table | A value only a person has, written into the install declaration whole: `name`, `file`, `what`, `secret`, `optional`, `default` |
| `[[token]]` | table | A value the install generates: `name`, `file` under the data root, and where random bytes stand in for a person's answer, `bytes` and `encoding` |
| `[[settings]]` | table | A settings file the install writes: `file` under the data root and `text` whole, with `{{port.*}}`, `{{token.*}}`, `{{value.*}}` and `{{input.*}}` placeholders the install fills |
| `[[line]]` | table | One line the install keeps in a file under the data root: `file`, `key`, `value` |
| `[tool]` | table | The application's hosted tool, its fields below: its name takes it out of the tool host on removal, and where a console half stands, it is placed and registered at every release |
| `base_libraries` | array | The base libraries the crate takes by path beside this checkout, beyond the skeleton this repository stands on; carried into the testbed's `base_libraries` check |
| `shared_versions` | string | The repository whose root holds the shared versions the manifests and the lock hold to; its name brings the `shared-versions` sweep and the `[check.shared_versions]` table into the testbed reading |
| `korean_ledger` | path | The ledger of the Korean left in code files; its name brings the `korean` sweep and the `korean_ledger` line |
| `personal_data_allowed` | array | The addresses the personal-data rule allows; its rows bring the `personal-data` sweep and the `personal_data_allowed` line |
| `test_setup` | array | Lines the suite runs on a fresh tree before any unit, carried whole into the testbed's `[test] setup` |
| `port_name` | string | The name the install declaration registers the service port under, where it is not `<name>.server` |
| `data_folder` | string | The folder under the data root holding this application's own data, where the data root is the machine's shared one: removal takes that folder and never the root |
| `checkout` | path | Where the repository's checkout stands on the machine; removal takes that line out of the machine's checkout register |
| `[[skill]]` | table | The service's own skill, passed into the install declaration (「Skills and memories」 below) |
| `[[memory]]` | table | The service's own memory, passed likewise |

### `[console]`

| Key | Type | Meaning |
|---|---|---|
| `project` | string | The folder holding the bundle's sources, its manifest and its lock, outside any package workspace |
| `install` | string | The command that lays the bundle's locked packages |
| `bundle` | string | The command that builds the bundle; `bun run bundle` where none is named |
| `built` | string | `deploy` builds the bundle inside the release before the flip; `build` builds it during the build and writes it into the release being built. `deploy` where none is named |
| `release_out` | string | For `built = "build"`: the environment variable naming where in the release the bundle goes |
| `check_script` | string | The project's check script, where the testbed's declaration names it |
| `typecheck` | boolean | The bundle is typechecked as a static step too |
| `reach` | boolean | The service answers `console-reach` and the deployment probes the console face once the service is up |
| `publish` | boolean | The screen is an app on the delegation chain: the deployment publishes the release's `deploy/app.toml` once the service answers ready |

### `[tool]`

| Key | Type | Meaning |
|---|---|---|
| `name` | string | The tool's registered name, what the tool host and the removal find it by |
| `dir` | string | The folder holding its `tool.toml`, manual and schemas, e.g. `tools/calendar`; where a console half stands, its contents are placed and registered at every release |
| `binary` | string | The executable placed for it; the service's own binary where none is named |
| `service` | string | The label it registers under; the tool name where none is named |
| `capability` | string | The capability its entries ask; `room.tools` where none is named |
| `env` | boolean | The deployment runs the service's `tool-env` verb before the flip, so the tool and the other units read the service's address and token |

## The generated files

Running `bmo-app-declare bmo-app.toml` in the application's checkout writes, beside the
manifest; every file says at its head what manifest it was generated from.

| File | What it holds |
|---|---|
| `testbed.toml` | The testbed's declaration for the application: its languages and projects, the static checks, the build's binaries and steps, the deploy roots, unit glob, health verb, console probe, and the family's deploy and build steps |
| `install.toml` | The install declaration: the programs it needs, its ports, the manifest's inputs, tokens, settings and lines, its skills and memories, its restart line |
| `deploy/<name>.service` | The service's user unit, marked as the one the deployment stops, restarts and waits on |
| `deploy/<name>-pages.service` | The page server's unit, marked to be restarted just before it, where a web app stands |
| `scripts/remove.sh` | The application's removal, the way `docs/removal.md` states it |
| `web/scripts/check.sh` | The web app's check, where a web app stands |
| `web/app-defaults.mjs` | The Nuxt defaults read by the app's own config, where a web app stands (`docs/web.md`) |

What each family adds to those readings stands in the documents that own the reads:
`docs/service.md` for the unit and the binary's shape, `docs/install.md` for the install
declaration, `docs/web.md` for the web scaffold, and here for the testbed's steps:

- A **web half** carries the site build and its placement into the release, and its project
  stands at `web/`.
- A **console half** carries `cargo` and `bun` as its languages, the bundle's folder as its
  one project, the bundle's build (`built = "deploy"` as a before-flip deploy step, `build` as
  a build-time pair that names the release's place through `release_out`), the optional
  typecheck step, the reach probe, the publication of the release's app declaration, and the
  tool's env, placement and registration steps.
- The **service alone** tests as Cargo only, with no project line and no steps.

`bmo-app-declare --check bmo-app.toml` writes nothing and exits nonzero naming every reading
the manifest no longer stands for; a job that edits a manifest regenerates and commits in the
same change, and the change check catches one left behind.

## Skills and memories

- **A live service declares its skill and its memories in its own manifest,** as `[[skill]]`
  and `[[memory]]` tables, and the generator carries them whole into its `install.toml`: the
  lifecycle file that already says what a service brings onto a machine and takes off it. The
  generator carries these tables whole and reads none of them; the install and removal paths
  that act on them belong to the platform (the service-declared skills and memories work
  states their shape, their immutability and their withdrawal), and this repository states
  only where they sit.
- **They name the service by its `name`,** which is what every reader finds it by; a memory's
  subject and a skill's home carry that name.
