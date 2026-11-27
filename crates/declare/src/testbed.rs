//! The repository's testbed declaration a manifest stands for: its checks, its build and its deployment.

use std::fmt::Write as _;

use crate::*;

/// The testbed's declaration: what stands alike in every family, then the family's own lines.
pub(crate) fn render_testbed(m: &Manifest) -> String {
    let mut out = String::from(GENERATED_HEAD);
    let _ = write!(
        out,
        "\ntestbed = 0\nname = {name}\n\n# The shared base whose generator and skeletons this repository stands on.\nupstreams = [\"bmo-app-sdk\"]\n\n[test]\n",
        name = quote(&m.name)
    );
    out.push_str(match m.family() {
        Family::Web(_) => "languages = [\"cargo\", \"bun\", \"browser\"]\n",
        Family::Console(_) => "languages = [\"cargo\", \"bun\"]\n",
        Family::Plain => "languages = [\"cargo\"]\n",
    });
    if !m.test_setup.is_empty() {
        let _ = write!(
            out,
            "# What a fresh tree lacks, laid before any unit runs.\nsetup = [{}]\n",
            quoted_list(m.test_setup.iter().map(String::as_str))
        );
    }
    match m.family() {
        Family::Web(_) => out.push_str(
            "\n[projects]\n# The web app is a project of its own, its locked packages read by its own check script.\ndiscover = false\npaths = [\"web\"]\n",
        ),
        Family::Console(console) => {
            let _ = write!(
                out,
                "\n[projects]\n# The console bundle is a project of its own, its manifest and lockfile outside any package workspace.\npaths = [{}]\n",
                quote(&console.project)
            );
            if let Some(script) = &console.check_script {
                let _ = writeln!(out, "check_script = {}", quote(script));
            }
        }
        Family::Plain => {}
    }
    let console_tool = match m.family() {
        Family::Console(_) => m.tool.as_ref(),
        _ => None,
    };
    if let Some(tool) = console_tool {
        let _ = write!(
            out,
            "\n# The tool the service places on the tool host.\n[tools]\ndir = {}\n",
            quote(tool_root(&tool.dir()))
        );
    }
    out.push_str(&render_check(m));
    let mut binaries = vec![m.name.as_str()];
    if let Some(tool) = console_tool
        && let Some(binary) = &tool.binary
    {
        // The tool's own binary builds beside the service's.
        binaries.push(binary.as_str());
    }
    let _ = write!(out, "\n[build]\nbinaries = [{}]\n", quoted_list(binaries));
    match m.family() {
        Family::Web(_) => out.push_str(
            "\n# The web app, built in the build tree and copied into the release being built as its\n\
             # site folder: the release beside the build tree, named by the first twelve hex digits\n\
             # of the commit.\n\
             [[build.step]]\n\
             id = \"site\"\n\
             dir = \"web\"\n\
             command = \"export PATH=\\\"$HOME/.bun/bin:/usr/local/bin:$PATH\\\" && bun install --frozen-lockfile && bun run build\"\n\n\
             [[build.step]]\n\
             id = \"place\"\n\
             command = \"site=\\\"$(git rev-parse --show-toplevel)/../deploy-releases/$(git rev-parse HEAD | cut -c1-12)/site\\\" && mkdir -p \\\"$site\\\" && cp -a web/.output/. \\\"$site/\\\"\"\n",
        ),
        Family::Console(console) if console.built == Built::Build => {
            let bundle_command = format!(
                "{}=\"$(git rev-parse --show-toplevel)/../deploy-releases/$(git rev-parse HEAD | cut -c1-12)/console\" {}",
                console.release_out.as_deref().unwrap_or_default(),
                console.bundle
            );
            let _ = write!(
                out,
                "\n# The console bundle, installed and built in the build tree and written into the\n\
                 # release being built, where the service reads its console folder from.\n\
                 {}\n{}",
                build_step(
                    "console-install",
                    &console.project,
                    None,
                    &quote(&console.install)
                ),
                build_step(
                    "console-bundle",
                    &console.project,
                    Some("wide"),
                    &quote(&bundle_command)
                )
            );
        }
        _ => {}
    }
    out.push_str(&render_deploy(m));
    out
}

/// The parent of a tool's folder, the root the testbed's `[tools]` section names.
pub(crate) fn tool_root(dir: &str) -> &str {
    dir.rsplit_once('/').map_or(dir, |(root, _)| root)
}

/// The static-check section: the sweeps, the ledgers, the base libraries and the family's
/// own steps.
pub(crate) fn render_check(m: &Manifest) -> String {
    let mut out = String::from("\n[check]\n");
    let mut quick: Vec<&str> = Vec::new();
    if m.korean_ledger.is_some() {
        quick.push("korean");
    }
    if matches!(m.family(), Family::Console(_)) {
        quick.extend(["layout", "dependencies", "cargo-profile"]);
    }
    if m.shared_versions.is_some() {
        quick.push("shared-versions");
    }
    if !m.personal_data_allowed.is_empty() {
        quick.push("personal-data");
    }
    if !quick.is_empty() {
        let _ = write!(
            out,
            "# The sweeps that read the tree before anything runs.\nquick = [{}]\n",
            quoted_list(quick)
        );
    }
    if !m.personal_data_allowed.is_empty() {
        let _ = write!(
            out,
            "# The addresses the personal-data rule allows.\npersonal_data_allowed = [{}]\n",
            quoted_list(m.personal_data_allowed.iter().map(String::as_str))
        );
    }
    if let Some(ledger) = &m.korean_ledger {
        let _ = write!(
            out,
            "# The debt of Korean left in a code file; a file it does not name is allowed none.\nkorean_ledger = {}\n",
            quote(ledger)
        );
    }
    let mut libraries = vec!["bmo-app-service"];
    libraries.extend(m.base_libraries.iter().map(String::as_str));
    let _ = write!(
        out,
        "# The base library the service crate takes by a path beside this checkout.\nbase_libraries = [{}]\n",
        quoted_list(libraries)
    );
    if let Some(repository) = &m.shared_versions {
        let _ = write!(
            out,
            "\n# The repository whose root holds the shared versions the manifests and the lock hold to.\n[check.shared_versions]\nrepository = {}\n",
            quote(repository)
        );
    }
    if let Family::Console(console) = m.family()
        && console.typecheck
    {
        let _ = write!(
            out,
            "\n# The console bundle is typechecked against its own install, as its check script does.\n[[check.step]]\nid = \"{}-typecheck\"\ndir = {}\ncommand = {}\nwhen = [{}]\n",
            console.project,
            quote(&console.project),
            quote(&format!("{} && bun run typecheck", console.install)),
            quote(&format!("{}/**", console.project))
        );
    }
    out
}

/// The deploy section and the family's steps, in the order they stand in the reading.
pub(crate) fn render_deploy(m: &Manifest) -> String {
    let mut out = render_deploy_steps(m);
    if m.holder.is_some() {
        let declare = script_command(&[
            "set -eu",
            "brain=\"$HOME/.local/share/bmo-brain/deploy/bin/bmo-brain\"",
            "[ -x \"$brain\" ] || exit 0",
            &format!("\"$brain\" holder declare {HOLDER_FILE}"),
        ]);
        let _ = write!(
            out,
            "\n# This service's standing line in every agent's brain, declared once the service is up (the brain's\n\
             # docs/boundary-api.md, 「Declarations and holders」); a machine with no brain passes.\n{}",
            deploy_step("holder-declare", "after-health", &declare)
        );
    }
    out
}

/// The file a holder's set stands in, in the repository and in every release.
pub const HOLDER_FILE: &str = "deploy/brain-holder.toml";

fn render_deploy_steps(m: &Manifest) -> String {
    let mut out = String::from(
        "\n# The service's deploy tree and its data, kept apart from every release.\n[deploy]\n",
    );
    let _ = write!(
        out,
        "root = {}\nlink = \"deploy\"\ndata_root = {}\n",
        quote(&m.deploy_root),
        quote(&m.data_root)
    );
    if matches!(m.family(), Family::Console(_))
        && let Some(first) = m.settings.first()
    {
        let _ = write!(
            out,
            "# The deployed settings each release links beside its own.\nenv_links = [{{ link = \"settings.local.toml\", target = {} }}]\n",
            quote(&first.file)
        );
    }
    let _ = write!(
        out,
        "units = [\"deploy/*.service\"]\nhealth = {}\n",
        quote(&format!("{}/deploy/bin/{} health", m.deploy_root, m.name))
    );
    let Family::Console(console) = m.family() else {
        return out;
    };
    if console.reach {
        let _ = write!(
            out,
            "\n# The screen the service brings to a console, whose face the deployment probes once\n\
             # the service is up.\n[deploy.console]\nreach = {}\n",
            quote(&format!("bin/{} console-reach", m.name))
        );
    }
    if console.built == Built::Deploy {
        let bundle = script_command(&[
            "set -eu",
            &format!("cd {}", console.project),
            &console.install,
            &console.bundle,
        ]);
        let _ = write!(
            out,
            "\n# The console bundle the service serves from the release: its locked packages laid\n\
             # and its folder built inside the release, where the settings' console folder reads it.\n{}",
            deploy_step("console-bundle", "before-flip", &bundle)
        );
    }
    if console.publish {
        let publish = script_command(&["set -eu", "bmo pocket app deploy/app.toml"]);
        let _ = write!(
            out,
            "\n# The app the console screen is, published on the delegation chain once the service\n\
             # answers ready, from the declaration the release carries.\n{}",
            deploy_step("publish-app", "after-health", &publish)
        );
    }
    let Some(tool) = m.tool.as_ref() else {
        return out;
    };
    let dir = tool.dir();
    if tool.env {
        let _ = write!(
            out,
            "\n# The env file the tool and the other units read the service's address and token from.\n{}",
            deploy_step(
                "tool-env",
                "before-flip",
                &quote(&format!("bin/{} tool-env", m.name))
            )
        );
    }
    let place_command = script_command(&[
        "set -eu",
        &format!("rm -rf {dir}/.placed"),
        &format!("mkdir -p {dir}/.placed"),
        &format!(
            "cp bin/{} {dir}/tool.toml {dir}/manual.md {dir}/*.json {dir}/.placed/",
            tool.binary(&m.name)
        ),
    ]);
    let _ = write!(
        out,
        "\n# The tool placed in its folder's .placed/, the folder the tool host reads: the executable\n\
         # the release's bin/ holds, its manifest, its manual and the schemas beside it.\n{}",
        deploy_step(
            &format!("place-{}-tool", tool.name),
            "before-flip",
            &place_command
        )
    );
    let register_command = script_command(&[
        "set -eu",
        &format!(
            "bmo pocket put {dir}/.placed --placed --service {} --allow {}",
            tool.service(),
            tool.capability()
        ),
    ]);
    let _ = write!(
        out,
        "\n# The placed tool registered with the tool host under this service's name, carrying the\n\
         # capability its entries ask for.\n{}",
        deploy_step(
            &format!("register-{}-tool", tool.name),
            "after-health",
            &register_command
        )
    );
    out
}
