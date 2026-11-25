#!/bin/sh
# Stands a new BMO application checkout up from nothing, the way this repository's README states it:
# the manifest, its generated declarations, a service that serves and answers health, and the checks.
# The application's own domain is what is left to write.
#
# Usage: new-app.sh --name <name> --port <service-port> [--pages-port N] [--dev-port N]
#                  [--public-url URL] [--description TEXT] <target-directory>
# Run it from inside this repository's checkout, or set BMO_APP_SDK to its path.
set -eu

NAME=""
PORT=""
PAGES_PORT=""
DEV_PORT=""
PUBLIC_URL=""
DESCRIPTION=""

while [ "$#" -gt 0 ]; do
  case "$1" in
    --name) NAME="$2"; shift 2 ;;
    --port) PORT="$2"; shift 2 ;;
    --pages-port) PAGES_PORT="$2"; shift 2 ;;
    --dev-port) DEV_PORT="$2"; shift 2 ;;
    --public-url) PUBLIC_URL="$2"; shift 2 ;;
    --description) DESCRIPTION="$2"; shift 2 ;;
    -*)
      echo "new-app: unknown option: $1" >&2
      exit 2
      ;;
    *) break ;;
  esac
done
TARGET="${1:-}"

if [ -z "$NAME" ] || [ -z "$PORT" ] || [ -z "$TARGET" ]; then
  echo "new-app: --name, --port and a target directory are needed" >&2
  exit 2
fi
if [ -e "$TARGET" ]; then
  echo "new-app: $TARGET already stands" >&2
  exit 2
fi

SDK="${BMO_APP_SDK:-$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)}"
if [ ! -f "$SDK/Cargo.toml" ]; then
  echo "new-app: no bmo-app-sdk checkout at $SDK (set BMO_APP_SDK)" >&2
  exit 2
fi

PREFIX=$(printf '%s' "$NAME" | tr '[:lower:]-' '[:upper:]_')
PAGES_PORT=${PAGES_PORT:-$((PORT + 1))}
DEV_PORT=${DEV_PORT:-$((PORT - 4000))}
DESCRIPTION=${DESCRIPTION:-"$NAME, a BMO application"}

# The web half stands only when a public url is named; a service-only application says none.

mkdir -p "$TARGET/src" "$TARGET/docs" "$TARGET/tests" "$TARGET/scripts"
TARGET_ABS=$(CDPATH= cd -- "$TARGET" && pwd)

# The skeleton crate is reached by the path the two checkouts stand on: a sibling reads it as
# a sibling, anywhere else it is named outright.
SDK_ABS=$(CDPATH= cd -- "$SDK" && pwd)
if [ "$(dirname "$TARGET_ABS")" = "$(dirname "$SDK_ABS")" ]; then
  SERVICE_PATH="../$(basename "$SDK_ABS")/crates/service"
else
  SERVICE_PATH="$SDK_ABS/crates/service"
fi

printf 'app = 1\nname = "%s"\ndescription = "%s"\nenv_prefix = "%s"\ndeploy_root = "~/.local/share/%s"\ndata_root = "~/.local/share/%s-data"\nservice_port = %s\n' \
  "$NAME" "$DESCRIPTION" "$PREFIX" "$NAME" "$NAME" "$PORT" >"$TARGET/bmo-app.toml"
if [ -n "$PUBLIC_URL" ]; then
  printf 'public_url = "%s"\n\n[web]\npages_port = %s\ndev_port = %s\n' \
    "$PUBLIC_URL" "$PAGES_PORT" "$DEV_PORT" >>"$TARGET/bmo-app.toml"
fi

cat >"$TARGET/Cargo.toml" <<EOF
[package]
name = "$NAME"
version = "0.1.0"
edition = "2024"
description = "$DESCRIPTION"
license = "MIT"
publish = false

[dependencies]
# The shared service skeleton: the answers, the serving loop, the health check.
bmo-app-service = { path = "$SERVICE_PATH" }
axum = "0.8.9"
clap = { version = "4.6.7", features = ["derive", "env"] }
serde = { version = "1.0.229", features = ["derive"] }
serde_json = "1.0.154"
tokio = { version = "1.53.3", features = ["macros", "net", "rt-multi-thread", "signal"] }
EOF

cat >"$TARGET/src/main.rs" <<EOF
//! \`$NAME\`: \`serve\` answers the API, \`health\` asks a running service whether it answers.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "$NAME", version, about = "$DESCRIPTION")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Serve the API until the terminal or the service manager stops it.
    Serve {
        /// The address to listen on.
        #[arg(long, env = "${PREFIX}_LISTEN", default_value = "127.0.0.1:$PORT")]
        listen: std::net::SocketAddr,
    },
    /// Answer 0 once the service at the address answers its health route.
    Health {
        #[arg(
            long,
            env = "${PREFIX}_HEALTH_URL",
            default_value = "http://127.0.0.1:$PORT/api/health"
        )]
        url: String,
    },
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let result = match Cli::parse().command {
        Command::Serve { listen } => serve(listen).await,
        Command::Health { url } => bmo_app_service::health_check(&url).await,
    };
    match result {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("$NAME: {error}");
            std::process::ExitCode::FAILURE
        }
    }
}

async fn serve(listen: std::net::SocketAddr) -> Result<(), bmo_app_service::BoxError> {
    let app = axum::Router::new().route(
        "/api/health",
        axum::routing::get(|| async { axum::Json(serde_json::json!({ "ok": true })) }),
    );
    // The domain's own routes join here, every answer through bmo_app_service::answer.
    let listener = tokio::net::TcpListener::bind(listen).await?;
    eprintln!("$NAME: listening on {listen}");
    bmo_app_service::serve(listener, app).await?;
    Ok(())
}
EOF

cat >"$TARGET/README.md" <<EOF
# $NAME

$DESCRIPTION

Stood up from bmo-app-sdk: \`bmo-app.toml\` is the one declaration; \`testbed.toml\`,
\`install.toml\`, the units and the removal script are generated from it (see the SDK's
README for the commands). What this application answers is its own to write from here.
EOF

cat >"$TARGET/.gitignore" <<EOF
/target
/web/node_modules
/web/.nuxt
/web/.output
.claude/
EOF

# The service skeleton comes by a path beside the application; git init before generating so
# paths read as siblings.
git init -q -b main "$TARGET"

# Generate the declarations. The build of the generator runs in the SDK's own checkout.
GEN=$("$SDK_ABS/bin/build-generator.sh")
"$GEN" "$TARGET_ABS/bmo-app.toml" >/dev/null

echo "new-app: $TARGET stands with its declarations; its first steps are the SDK README's."
