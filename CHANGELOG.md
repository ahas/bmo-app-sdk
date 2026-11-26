# Changelog

All notable changes to this project are documented in this file.
The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

## 0.1.0

### Features

- The application manifest `bmo-app.toml` and its generator `bmo-app-declare`, writing an
  application's `testbed.toml`, `install.toml`, systemd units, removal script, web check
  script and Nuxt defaults from the one declaration.
- The service skeleton crate `bmo-app-service`: the JSON answer envelope, the serving loop
  with its graceful stop, and the health verb's checks.
- The console family of the manifest: a `[console]` half, a `[tool]`, the install's `[[token]]`,
  `[[settings]]` and `[[line]]`, the testbed's sweeps and shared versions, the install port name,
  the data folder and the checkout path, with the family's generated build and deploy steps, its
  settings-conditioned unit and its guarded removal.
- `bmo-app-service`'s `serve_until`, the serving loop ended by a stop the application hands in,
  and `code_problem`, the coded failure envelope the console family's consumers read.
- The scaffold `bin/new-app.sh`, which stands a new application checkout up from nothing.
- Documents for the manifest, the install path, the removal path, the service skeleton and
  the web defaults.
