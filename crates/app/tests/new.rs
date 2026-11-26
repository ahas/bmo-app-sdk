//! `app new`: the scaffold's own words handed on, the SDK found where the scaffold finds it,
//! the scaffold's exit code propagated, and the steps left said only once it ends well. The
//! scaffold itself is tested where it stands; these tests run a recording stub.

mod support;

#[test]
fn new_runs_the_scaffold_with_its_words_and_says_the_steps_left() {
    let world = support::World::new("new-words");
    let (sdk, record) = support::fake_sdk(&world, 0);

    let answered = support::run_with(
        &[
            "app",
            "new",
            "qa-widget",
            "--port",
            "7777",
            "--pages-port",
            "7778",
            "--public-url",
            "https://widget.example",
            "--description",
            "A widget, stood up for a test",
        ],
        &world,
        &world.root,
        &[("BMO_APP_SDK", sdk.clone())],
    );
    assert_eq!(answered.code, Some(0), "{}", answered.stderr);

    let handed = std::fs::read_to_string(&record).expect("the scaffold answered");
    assert_eq!(
        handed,
        format!(
            "--name\nqa-widget\n--port\n7777\n--pages-port\n7778\n--public-url\nhttps://widget.example\n--description\nA widget, stood up for a test\n{}\n",
            world.root.join("qa-widget").display()
        )
    );
    assert!(
        answered.stdout.contains("qa-widget stands at"),
        "{}",
        answered.stdout
    );
    assert!(
        answered.stdout.contains("1. Write the domain"),
        "{}",
        answered.stdout
    );
    assert!(
        answered.stdout.contains("6. Ask for the install"),
        "{}",
        answered.stdout
    );
}

#[test]
fn new_hands_the_scaffold_the_path_it_was_given() {
    let world = support::World::new("new-path");
    let (sdk, record) = support::fake_sdk(&world, 0);

    let answered = support::run_with(
        &[
            "app",
            "new",
            "qa-widget",
            "--port",
            "7777",
            "elsewhere/widget",
        ],
        &world,
        &world.root,
        &[("BMO_APP_SDK", sdk)],
    );
    assert_eq!(answered.code, Some(0), "{}", answered.stderr);

    let handed = std::fs::read_to_string(&record).expect("the scaffold answered");
    assert!(handed.ends_with("elsewhere/widget\n"), "{handed}");
}

#[test]
fn new_finds_the_sdk_among_the_register_checkouts() {
    let world = support::World::new("new-register");
    let (sdk, record) = support::fake_sdk(&world, 0);
    support::register(&world, &[sdk.display().to_string()]);

    let answered = support::run(
        &["app", "new", "qa-widget", "--port", "7777"],
        &world,
        &world.root,
    );
    assert_eq!(answered.code, Some(0), "{}", answered.stderr);
    assert!(
        std::fs::read_to_string(&record)
            .expect("the scaffold answered")
            .contains("qa-widget")
    );
}

#[test]
fn new_propagates_the_scaffold_refusal_without_the_steps() {
    let world = support::World::new("new-refused");
    let (sdk, _) = support::fake_sdk(&world, 2);

    let answered = support::run_with(
        &["app", "new", "qa-widget"],
        &world,
        &world.root,
        &[("BMO_APP_SDK", sdk)],
    );
    assert_eq!(answered.code, Some(2));
    assert!(
        !answered.stdout.contains("steps left"),
        "{}",
        answered.stdout
    );
}

#[test]
fn new_says_where_no_sdk_can_be_found() {
    let world = support::World::new("new-nowhere");

    let answered = support::run(
        &["app", "new", "qa-widget", "--port", "7777"],
        &world,
        &world.root,
    );
    assert_eq!(answered.code, Some(2));
    assert!(
        answered.stderr.contains("BMO_APP_SDK"),
        "{}",
        answered.stderr
    );
}
