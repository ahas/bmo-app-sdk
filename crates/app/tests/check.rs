//! `app check`: the declaration check and the repository's own check, for one application or
//! all of them. The testbed stands in by a stub that answers at once; the real check is the
//! testbed's own to test.

mod support;

#[test]
fn check_passes_where_the_readings_stand_and_the_repository_agrees() {
    let world = support::World::new("check-pass");
    let checkout = support::named_application(&world, "qa-applet");
    support::register(&world, &[checkout.display().to_string()]);
    support::run(&["app", "declare"], &world, &checkout);
    support::stub_script(&world.bin.join("bmo-testbed"), "exit 0");

    let answered = support::run(&["app", "check"], &world, &world.root);
    assert_eq!(answered.code, Some(0), "{}", answered.stdout);
    assert!(
        answered.stdout.contains("  declarations: in step"),
        "{}",
        answered.stdout
    );
    assert!(
        answered
            .stdout
            .contains("the repository's own check passed"),
        "{}",
        answered.stdout
    );
}

#[test]
fn check_fails_where_the_repository_fails() {
    let world = support::World::new("check-repository-fails");
    let checkout = support::named_application(&world, "qa-applet");
    support::register(&world, &[checkout.display().to_string()]);
    support::run(&["app", "declare"], &world, &checkout);
    support::stub_script(&world.bin.join("bmo-testbed"), "exit 1");

    let answered = support::run(&["app", "check"], &world, &world.root);
    assert_eq!(answered.code, Some(1));
    assert!(
        answered.stdout.contains("failed (exit 1)"),
        "{}",
        answered.stdout
    );
}

#[test]
fn check_fails_where_the_readings_are_off_and_still_runs_the_repository() {
    let world = support::World::new("check-readings-off");
    let checkout = support::named_application(&world, "qa-applet");
    support::register(&world, &[checkout.display().to_string()]);
    support::stub_script(&world.bin.join("bmo-testbed"), "exit 0");

    let answered = support::run(&["app", "check"], &world, &world.root);
    assert_eq!(answered.code, Some(1));
    assert!(
        answered.stdout.contains("declarations: out of step"),
        "{}",
        answered.stdout
    );
    assert!(
        answered.stdout.contains("missing: testbed.toml"),
        "{}",
        answered.stdout
    );
    assert!(
        answered
            .stdout
            .contains("the repository's own check passed"),
        "every part runs before the answer: {}",
        answered.stdout
    );
}

#[test]
fn check_one_application_runs_on_it_alone() {
    let world = support::World::new("check-one");
    let applet = support::named_application(&world, "qa-applet");
    let other = support::named_application(&world, "qa-pear");
    support::register(
        &world,
        &[applet.display().to_string(), other.display().to_string()],
    );
    support::run(&["app", "declare"], &world, &applet);
    support::stub_script(&world.bin.join("bmo-testbed"), "exit 0");

    let answered = support::run(&["app", "check", "qa-applet"], &world, &world.root);
    assert_eq!(answered.code, Some(0), "{}", answered.stdout);
    assert!(answered.stdout.contains("qa-applet"), "{}", answered.stdout);
    assert!(!answered.stdout.contains("qa-pear"), "{}", answered.stdout);
}

#[test]
fn check_reports_a_manifest_that_does_not_read() {
    let world = support::World::new("check-unread");
    let spoiled = support::application(&world, "qa-spoiled", "app = 1\nbogus = 2\n");
    support::register(&world, &[spoiled.display().to_string()]);

    let answered = support::run(&["app", "check"], &world, &world.root);
    assert_eq!(answered.code, Some(1));
    assert!(
        answered.stdout.contains("the manifest does not read"),
        "{}",
        answered.stdout
    );
}

#[test]
fn check_says_where_there_is_nothing_to_run_on() {
    let world = support::World::new("check-nothing");

    let none = support::run(&["app", "check"], &world, &world.root);
    assert_eq!(none.code, Some(2));
    assert!(none.stderr.contains("no applications"), "{}", none.stderr);

    let absent = support::run(&["app", "check", "qa-phantom"], &world, &world.root);
    assert_eq!(absent.code, Some(2));
    assert!(absent.stderr.contains("qa-phantom"), "{}", absent.stderr);
}

#[test]
fn check_says_where_the_testbed_cannot_be_reached() {
    let world = support::World::new("check-no-testbed");
    let checkout = support::named_application(&world, "qa-applet");
    support::register(&world, &[checkout.display().to_string()]);
    support::run(&["app", "declare"], &world, &checkout);

    let answered = support::run(&["app", "check"], &world, &world.root);
    assert_eq!(answered.code, Some(1));
    assert!(
        answered.stdout.contains("could not be started"),
        "{}",
        answered.stdout
    );
}
