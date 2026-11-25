# The application manifest: `bmo-app.toml`

- **One file at the application's root,** read by `bmo-app-declare`. It is where everything
  that differs between applications stands, once. Its generated readings go into the
  repository and are committed, because the testbed and the install verb read those files as
  the commit holds them; the manifest is the source an edit answers to.
- **It is TOML,** read against the schema below; a key the schema does not name is refused,
  naming it. Paths beginning `~/` read under the home directory, as everywhere on the
  platform; `/srv/` roots stand outside the home where a web server must read the tree.

## The fields

| Key | Type | Meaning |
|---|---|---|
| `app` | integer | The major of this schema: `1` |
| `name` | string | The application's name, in the small alphabet: it becomes the unit names, the deploy binary, the agency repository row and the install port prefix |
| `description` | string | One English sentence, carried into unit descriptions and crate metadata |
| `env_prefix` | string | The capitals prefix its environment variables carry (`docs/service.md`) |
| `deploy_root` | path | Where its deploy tree's link and releases stand |
| `data_root` | path | Where its runtime data stands, kept apart from every release |
| `service_port` | integer | The port the service answers on |
| `listen_host` | string | What the service binds, `127.0.0.1` where none is named |
| `public_url` | string | The address people reach the application at; required where a web app stands |
| `[web]` | table | The web half: `pages_port`, the loopback port its page server answers on, and `dev_port`, the port a person's dev server takes; present where the application brings a Nuxt app |
| `service_env` | table | Extra `Environment` lines for the service unit, key to value; `~/` reads as the home |
| `pages_env` | table | Extra `Environment` lines for the page server unit, likewise |
| `[[input]]` | table | A value only a person has, written into the install declaration whole: `name`, `file`, `what`, `secret`, `optional`, `default` |
| `tool` | string | The application's hosted tool's registered name, where it registers one: its removal takes it out of the tool host first |
| `[[skill]]` | table | The service's own skill, passed into the install declaration (「Skills and memories」 below) |
| `[[memory]]` | table | The service's own memory, passed likewise |

## The generated files

Running `bmo-app-declare bmo-app.toml` in the application's checkout writes, beside the
manifest; every file says at its head what manifest it was generated from.

| File | What it holds |
|---|---|
| `testbed.toml` | The testbed's declaration for the application: its languages and projects, the static checks' base-library line, the web build and place steps where a web app stands, the deploy roots, unit glob and health verb |
| `install.toml` | The install declaration: the programs it needs, its ports under the install's shared names, the manifest's inputs, its restart line |
| `deploy/<name>.service` | The service's user unit, marked as the one the deployment stops, restarts and waits on |
| `deploy/<name>-pages.service` | The page server's unit, marked to be restarted just before it, where a web app stands |
| `scripts/remove.sh` | The application's removal, the way `docs/removal.md` states it |
| `web/scripts/check.sh` | The web app's check, where a web app stands |
| `web/app-defaults.mjs` | The Nuxt defaults read by the app's own config, where a web app stands (`docs/web.md`) |

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
