//! `app declare`: the generator's words, answers and exit codes, in the checkout it runs in
//! or on a named application the register carries.

mod support;

#[test]
fn declare_writes_then_reports_in_step() {
    let world = support::World::new("declare-write");
    let checkout = support::named_application(&world, "qa-applet");

    let written = support::run(&["app", "declare"], &world, &checkout);
    assert_eq!(written.code, Some(0), "{}", written.stderr);
    assert_eq!(
        written.stdout.trim(),
        "bmo app declare: wrote the declarations of qa-applet"
    );

    let checked = support::run(&["app", "declare", "--check"], &world, &checkout);
    assert_eq!(checked.code, Some(0), "{}", checked.stderr);
    assert_eq!(checked.stdout, "");
}

#[test]
fn declare_check_names_every_reading_off() {
    let world = support::World::new("declare-off");
    let checkout = support::named_application(&world, "qa-applet");
    support::run(&["app", "declare"], &world, &checkout);

    let reading = checkout.join("testbed.toml");
    let mut off = std::fs::read_to_string(&reading).expect("the reading stands");
    off.push_str("# hand-edited, which the manifest no longer stands for\n");
    std::fs::write(&reading, off).expect("the reading is spoiled");
    std::fs::remove_file(checkout.join("install.toml")).expect("the reading is taken away");

    let checked = support::run(&["app", "declare", "--check"], &world, &checkout);
    assert_eq!(checked.code, Some(1));
    assert!(
        checked.stdout.contains("out of date: testbed.toml"),
        "{}",
        checked.stdout
    );
    assert!(
        checked.stdout.contains("missing: install.toml"),
        "{}",
        checked.stdout
    );
}

#[test]
fn declare_refuses_a_manifest_that_does_not_read() {
    let world = support::World::new("declare-unread");

    let nowhere = support::run(&["app", "declare"], &world, &world.root);
    assert_eq!(nowhere.code, Some(1));
    assert!(
        nowhere.stderr.contains("cannot read bmo-app.toml"),
        "{}",
        nowhere.stderr
    );

    let spoiled = support::application(&world, "qa-spoiled", "app = 1\nbogus = 2\n");
    let spoiled = support::run(&["app", "declare"], &world, &spoiled);
    assert_eq!(spoiled.code, Some(1));
    assert!(spoiled.stderr.contains("bogus"), "{}", spoiled.stderr);
}

#[test]
fn declare_finds_the_application_the_register_names() {
    let world = support::World::new("declare-named");
    let checkout = support::named_application(&world, "qa-applet");
    support::register(&world, &[checkout.display().to_string()]);

    let written = support::run(&["app", "declare", "qa-applet"], &world, &world.root);
    assert_eq!(written.code, Some(0), "{}", written.stderr);
    assert!(
        checkout.join("testbed.toml").is_file(),
        "the readings went beside the manifest"
    );

    let absent = support::run(&["app", "declare", "qa-phantom"], &world, &world.root);
    assert_eq!(absent.code, Some(2));
    assert!(absent.stderr.contains("qa-phantom"), "{}", absent.stderr);
}

#[test]
fn declare_refuses_a_register_that_does_not_read() {
    let world = support::World::new("declare-register-spoiled");
    std::fs::write(&world.register, "checkouts = [ \"unterminated\n")
        .expect("the spoiled register stands");

    let answered = support::run(&["app", "declare", "qa-applet"], &world, &world.root);
    assert_eq!(answered.code, Some(2));
    assert!(
        answered.stderr.contains("does not read"),
        "{}",
        answered.stderr
    );
}
