# Installing an application

- **An application installs by the platform's one install verb,** the testbed's
  `bmo install --add <name>`, from the `install.toml` its manifest generated
  (`docs/manifest.md`). What that verb does with a declaration — needs, ports, inputs,
  settings, after-steps, the first release, the health read — is the testbed's own document
  to state; this one states only what an application's declaration brings.
- **Only a supervising agent runs an install.** A job that finds a service must be installed
  asks its supervisor; a job may run `--dry-run` and `--status`, which change nothing.

## What an application's declaration says

- **Needs:** the toolchains its build runs on. A web half and a plain service need Cargo and
  a C linker; a console half needs Cargo and Bun, for the bundle it builds at every release,
  and its Cargo line says it also builds the tool where the manifest names a tool of its own.
- **Ports:** the manifest's `port_name`, `<name>.server` where none is named, at the
  manifest's `service_port`, and `<name>.pages` at its `pages_port` where a web half stands.
  A declared number is the number preferred, and the units the same manifest generates bind
  exactly those numbers, so an install that must choose a different one leaves the
  application's own units on the old numbers: change the manifest, regenerate, and redeploy
  before installing again.
- **Inputs:** the manifest's `[[input]]` rows, kept in the machine's secret store under their
  named files, which the units read by path. An input already standing under the data root is
  never asked again, so an application that adopted a secret already saved on the machine
  installs without a prompt.
- **Tokens:** the manifest's `[[token]]` rows, values the install generates and keeps under
  the data root at their named files; `bytes` and `encoding` say what stands where random
  bytes replace a person's answer. An already-standing token is kept, not rolled.
- **Settings:** the manifest's `[[settings]]` rows, each a file under the data root written
  from its declared text with the `{{port.*}}`, `{{token.*}}`, `{{value.*}}` and
  `{{input.*}}` placeholders filled. A console application's service reads this file as its
  deployed settings, which each release links beside its own.
- **Lines:** the manifest's `[[line]]` rows, single `key` to `value` lines the install keeps
  in the named file under the data root, where another unit reads them (the console's own
  environment file names the applications' addresses and console tokens this way).
- **Restart:** its units, spaced.
- **Health:** the deploy declaration's health command, which the install asks once at the end
  through the deployment's own record.

## The data root of a console application

- **Its data root is the machine's shared data root,** and the application's own data stands
  in its `data_folder` under it, as the settings paths say. The install's `{{data_root}}`
  placeholder names that shared root; a web application's dedicated data root needs no
  folder, for all of it is its own.

## Standing the application up on a machine

1. The repository stands among the machine's checkouts and is named in the machine's checkout
   register, so the deploy tool finds it; the manifest's `checkout` names that line.
2. The set's index names it in its presets file, in a folder of the index; the folder holds
   the checkout itself (a link to where the application's checkout stands counts as standing
   there), since the install reads declarations from the index's folders.
3. The agency holds its repository row, so workers may be lent its checkout
   (`bmo repository add <name> --path <checkout>`).
4. `bmo install --add <name> --dry-run` first: every line `ok` or `planned` and none
   `failed` says the declaration is whole. The install itself is the supervisor's.
5. Its first release goes live the same way every repository's does, by the deployment;
   `docs/removal.md` states the road back.
