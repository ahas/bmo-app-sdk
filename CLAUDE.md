# Working in bmo-app-sdk

These rules hold every change to this repository.

## The documents come first

- `docs/` is the canonical statement of what this base carries; the code is one realisation
  of it. Before changing code, read the document covering that area.
- A change that would make a document wrong or incomplete repairs the document first,
  stating what is true now, and the code is written to match.
- The formats this repository writes are the platform's: the testbed's declaration and the
  install declaration belong to the testbed's documents. A schema change there lands here as
  a template change; this repository's documents state only what an application's manifest
  brings to them, never the schema whole again.
- A document states what is true now: no change history, no rationale told through an
  earlier design, no pasted source, signature or config fragment, and no file-and-line pins.

## Boundaries

- **A base never knows its consumers.** Nothing here names an application, a machine or a
  host; the generator runs on one application's manifest inside that application's checkout.
- **No lookup table standing in for polymorphism:** what differs per application stands in
  that application's manifest, and the templates hold only what is alike.
- **Mechanism here, policy there:** the skeleton supplies the answer envelope, the serving
  loop and the checks; the application supplies the router, the stores and the domain.
- **Dependencies:** crates from the registry, and of the base libraries `bmo-chain` alone, the delegation
  chain's client the skeleton hands every application. The testbed's `dependencies` check holds this.
- **No hand-written parser for a general format.** TOML is read by a TOML library; the
  manifest's own syntax is this project's reading.

## Language

- Everything in this repository is English: code, comments, strings, tests, documents,
  commit messages and branch names. No Korean in code or tests; the testbed's `korean` check
  holds this.
- No personal data: no user names, home directories, machine names or names of any agent or
  application beyond those already public. Generated files carry only paths of the home's
  share folder and `/srv/` that a manifest declared.

## Layout

- Tests live in each crate's `tests/` directory, never beside the code.
- The text every application's files are made of stands in `templates/`; the generator assembles
  the two declaration TOMLs in code and hands out everything else as a template reading.

## The stages of a change

1. **Writing:** the only check is `cargo check --all-targets`, on the narrowest crate that
   is cheaper.
2. **Review:** read the whole diff against the document governing it, then
   `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings`.
3. **Verification:** the tests touched, then `cargo test` once, output written to a file and
   the file read.

A change is not done before stage 3 has passed. A change to a template or to the generator is
verified by regenerating a real application's readings and diffing them, and its consumer's
change check is run where the job can.
