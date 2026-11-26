# The application command: `bmo app`

- **One familiar place for application work.** The unit command `bmo-app` declares the `app`
  nouns the entry command dispatches. It fronts the two tools this repository already carries —
  the scaffold and the generator — and keeps their vocabulary: each verb takes the words its
  tool takes and answers with its output and its exit codes.
- **Every noun carries the `home` mark.** The applications, their manifests and the checkout
  register stand on the operations machine, so a work node asks through the bridge.
- **Asked `nouns`, the command answers its four noun lines** and reads nothing else.

## Where it finds things

- **The machine's applications** are the checkouts of the machine's checkout register — the
  file `BMO_TESTBED_CHECKOUTS` names, else the data root's `testbed/checkouts.toml`, in the
  format the testbed states — that carry a `bmo-app.toml` at the root. A manifest's `name`
  names its application. The checkout a verb answers with is the one the manifest's `checkout`
  names where it names one, and the register's line otherwise.
- **This repository's own checkout** is the one `BMO_APP_SDK` names where it is set, else the
  register entry that carries the scaffold. The scaffold applies the same test and refuses a
  directory that is not this repository's checkout.
- **A register that stands nowhere holds no applications.** A register that does not read is
  refused with 2, as the testbed's rule states it.

## The verbs

| Verb | What it answers | Exit |
|---|---|---|
| `app new <name> --port <n> [--pages-port <n>] [--dev-port <n>] [--public-url <url>] [--description <text>] [<path>]` | The scaffold's answer, then the steps left to a person | The scaffold's: 0; 2 for the words it refuses or a target that stands; 1 where its own step fails |
| `app declare [<name>] [--check]` | The generator's answer: the wrote line, or the readings the manifest no longer stands for | The generator's: 0 wrote or in step; 1 out of step or a manifest that does not read; 2 for words that do not read |
| `app list` | The applications the machine carries, one a line: name, checkout, in step or out | 0 where the register read, whatever it found; 2 where it did not |
| `app check [<name>]` | The declaration check and the repository's own check for one application or all | 0 where every part passed; 1 where any failed; 2 where there was nothing to run on |

- **`app new` runs the scaffold and nothing else.** It hands the words to the scaffold in this
  repository's checkout — `--name` set from the positional name, the path last — and streams the
  scaffold's own answer. Where no path is given, the new checkout stands beside the SDK's, named
  for the application: sibling checkouts are how applications and this base sit side by side.
  Once the scaffold ends well, the command says the steps left to a person, in order: write the
  domain; check the checkout; give the repository a remote; register the checkout in the
  machine's register and the repository with the agency; ask for the install and the first
  release through the supervisor. The command takes none of those steps itself.
- **`app declare` renders through the generator's crate itself,** so its reports and exit codes
  are the generator's, written by one implementation. `--check` names every reading the manifest
  no longer stands for and writes nothing; without it the readings are written beside the
  manifest. With no `<name>` the application is the checkout the command runs in; with one, it
  is found in the register.
- **`app list` reports and does not fail on what it finds:** 0 whenever the register read. An
  application whose manifest does not read stands on its line with the reason; its readings are
  none the less reported as unreadable.
- **`app check` runs both parts for every application it was asked for** — the declaration
  check, then the repository's own check through the testbed's check verb, run inside that
  application's checkout — and answers once at the end. It fails where any part failed.
- **The command lends; it owns nothing.** The scaffold's, the generator's and the testbed's
  words stand as those tools write them; the command's own lines open with its verb's name.

## How it reaches a machine

- **The repository's deployment links the command into the folder holding `bmo`,** where the
  entry command finds unit commands. The link names this repository's deploy tree, whose own
  link stands across releases, so it names whichever release is live and never dangles once an
  older one is pruned. The repository runs no service, starts no unit and installs nothing:
  this linked command is its only deployment.
