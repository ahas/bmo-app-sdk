# Installing an application

- **An application installs by the platform's one install verb,** the testbed's
  `bmo install --add <name>`, from the `install.toml` its manifest generated
  (`docs/manifest.md`). What that verb does with a declaration — needs, ports, inputs,
  settings, after-steps, the first release, the health read — is the testbed's own document
  to state; this one states only what an application's declaration brings.
- **Only a supervising agent runs an install.** A job that finds a service must be installed
  asks its supervisor; a job may run `--dry-run` and `--status`, which change nothing.

## What an application's declaration says

- **Needs:** the toolchains its build runs on: Cargo and a C linker always, Bun where the
  manifest brings a `[web]` half.
- **Ports:** `<name>.server` at the manifest's `service_port`, and `<name>.pages` at its
  `pages_port` where there is one. A declared number is the number preferred, and the units
  the same manifest generates bind exactly those numbers, so an install that must choose a
  different one leaves the application's own units on the old numbers: change the manifest,
  regenerate, and redeploy before installing again.
- **Inputs:** the manifest's `[[input]]` rows, kept in the machine's secret store under their
  named files, which the units read by path. An input already standing under the data root is
  never asked again, so an application that adopted a secret already saved on the machine
  installs without a prompt.
- **Restart:** its units, spaced.
- **Health:** the deploy declaration's health command, which the install asks once at the end
  through the deployment's own record.

## Standing the application up on a machine

1. The repository stands among the machine's checkouts and is named in the machine's checkout
   register, so the deploy tool finds it.
2. The set's index names it in its presets file, in a folder of the index; the folder holds
   the checkout itself (a link to where the application's checkout stands counts as standing
   there), since the install reads declarations from the index's folders.
3. The agency holds its repository row, so workers may be lent its checkout
   (`bmo repository add <name> --path <checkout>`).
4. `bmo install --add <name> --dry-run` first: every line `ok` or `planned` and none
   `failed` says the declaration is whole. The install itself is the supervisor's.
5. Its first release goes live the same way every repository's does, by the deployment;
   `docs/removal.md` states the road back.
