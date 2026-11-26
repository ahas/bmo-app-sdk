//! `app list`: the applications the register carries, each with the checkout its manifest
//! names and whether its readings are in step. The register's shapes are read here, once:
//! strings, tables, paths beside the file and paths under the home.

mod support;

use std::fs;

#[test]
fn list_reports_each_application_once() {
    let world = support::World::new("list");
    let applet = support::named_application(&world, "qa-applet");
    let not_an_application = world.root.join("qa-service");
    fs::create_dir_all(&not_an_application).expect("the plain checkout stands");
    support::register(
        &world,
        &[
            applet.display().to_string(),
            not_an_application.display().to_string(),
        ],
    );
    support::run(&["app", "declare"], &world, &applet);

    let answered = support::run(&["app", "list"], &world, &world.root);
    assert_eq!(answered.code, Some(0), "{}", answered.stderr);
    assert_eq!(
        answered.stdout,
        format!("qa-applet  {}  in step\n", applet.display())
    );
}

#[test]
fn list_shows_the_checkout_the_manifest_names() {
    let world = support::World::new("list-checkout");
    let manifest = format!("{}\ncheckout = \"~/repos/qa-applet\"\n", support::MANIFEST);
    let checkout = support::application(&world, "qa-applet", &manifest);
    support::register(&world, &[checkout.display().to_string()]);

    let answered = support::run(&["app", "list"], &world, &world.root);
    assert_eq!(answered.code, Some(0), "{}", answered.stderr);
    assert!(
        answered.stdout.trim().starts_with(&format!(
            "qa-applet  {}/repos/qa-applet  out of step: ",
            world.home.display()
        )),
        "the manifest's checkout, not the register line: {}",
        answered.stdout
    );
    assert!(
        answered.stdout.contains("missing: testbed.toml"),
        "{}",
        answered.stdout
    );
}

#[test]
fn list_reads_the_register_shapes() {
    let world = support::World::new("list-shapes");
    let absolute = support::named_application(&world, "qa-absolute");
    let tabled = support::named_application(&world, "qa-tabled");
    support::named_application(&world, "qa-relative");
    let under_home = world.home.join("qa-tilde");
    fs::create_dir_all(&under_home).expect("the home checkout stands");
    fs::write(
        under_home.join(bmo_app_declare::MANIFEST),
        support::MANIFEST.replace("qa-applet", "qa-tilde"),
    )
    .expect("the manifest is written");

    fs::write(
        &world.register,
        format!(
            "checkouts = [\n  \"{}\",\n  {{ path = \"{}\" }},\n  \"qa-relative\",\n  \"~/qa-tilde\",\n]\n",
            absolute.display(),
            tabled.display(),
        ),
    )
    .expect("the register is written");

    let answered = support::run(&["app", "list"], &world, &world.root);
    assert_eq!(answered.code, Some(0), "{}", answered.stderr);
    for name in ["qa-absolute", "qa-tabled", "qa-relative", "qa-tilde"] {
        assert!(
            answered.stdout.contains(name),
            "{name} missed: {}",
            answered.stdout
        );
    }
    assert_eq!(answered.stdout.lines().count(), 4);
}

#[test]
fn list_reports_a_manifest_that_does_not_read() {
    let world = support::World::new("list-unread");
    let spoiled = support::application(&world, "qa-spoiled", "app = 1\nbogus = 2\n");
    support::register(&world, &[spoiled.display().to_string()]);

    let answered = support::run(&["app", "list"], &world, &world.root);
    assert_eq!(
        answered.code,
        Some(0),
        "the report does not fail on what it finds: {}",
        answered.stderr
    );
    assert!(
        answered.stdout.contains("qa-spoiled"),
        "{}",
        answered.stdout
    );
    assert!(answered.stdout.contains("bogus"), "{}", answered.stdout);
}

#[test]
fn list_says_where_no_applications_stand() {
    let world = support::World::new("list-empty");

    let answered = support::run(&["app", "list"], &world, &world.root);
    assert_eq!(answered.code, Some(0));
    assert_eq!(answered.stdout, "");
    assert!(
        answered.stderr.contains("carries no applications"),
        "{}",
        answered.stderr
    );
}

#[test]
fn list_refuses_a_register_that_does_not_read() {
    let world = support::World::new("list-spoiled");
    fs::write(&world.register, "checkouts = [ \"unterminated\n")
        .expect("the spoiled register stands");

    let answered = support::run(&["app", "list"], &world, &world.root);
    assert_eq!(answered.code, Some(2));
    assert!(
        answered.stderr.contains("does not read"),
        "{}",
        answered.stderr
    );
}
