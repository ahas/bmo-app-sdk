# The service skeleton: `bmo-app-service`

- **A library every BMO application's binary stands on,** for the parts of a service that
  are the same in all of them: the JSON envelope every API answer travels in, the serving
  loop that stops gracefully, and the health verb's checks. The application writes what its
  routes answer; the shape of an answer, of a stop and of a readiness read is here.

## The envelope

- `answer` takes a result and returns its response: a success is the value's own JSON with a
  200; a failure is a 500 with a body holding one `error` string.
- `not_found` is the JSON 404 every router's fallback returns, in that same body shape.
- `problem` writes that body with a status the handler names, for a failure that is not a
  server fault; an application's own 404s and 400s answer through it.
- The body type is `Problem`, and every failure's message is the display of the error, whole.

## The serving loop

- `serve` answers on a bound listener with a router and returns once shutdown begins,
  finishing the answers already under way.
- `shutdown_signal` ends on an interrupt or on the stop signal the service manager sends, so
  a deployment's restart is a stop, not a kill.

## The health checks

- `health_check` answers ok once the given address answers it 2xx.
- `site_check` also asks the root page of that address's site, so a service that answers
  health but cannot hand a page on through its page server fails, which is what the
  deployment waits to learn. The generated units and the install's health read use the
  application's own health verb, and an application with a web half reads it off
  `site_check`.
- A request of a check that stands without answer for five seconds is a failed check, not a
  wait: the deployment asks again.

## The binary's shape

These are the conventions the units generated from a manifest hand to the binary; an
application follows them or names its own in its manifest's `service_env`.

- The binary is named for the application and answers two verbs at least: `serve`, and
  `health` with a `--url` flag defaulting to the loopback health address and reading the
  `<PREFIX>_HEALTH_URL` environment.
- Its settings are flags that each read an environment variable under the manifest's
  capitals `env_prefix`: listen `<PREFIX>_LISTEN`, data `<PREFIX>_DATA_DIR`; a web
  application's `serve` takes `<PREFIX>_SITE_DIR`, `<PREFIX>_PAGES_URL` and
  `<PREFIX>_PUBLIC_URL` too, which is what the generated units set.
- The health address of an application with a web half is `http://127.0.0.1:<service>/api/health`;
  the API nest is `/api` and the health route `/api/health`.
- The page server answers on the loopback only; the public port is the service's alone.

## Versions

- The crate's version is a placeholder that never moves; consumers read the committed
  checkout, as every sibling path dependency stands on this machine.
