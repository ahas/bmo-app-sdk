//! The protocol answer: the four nouns, each marked `home`, and nothing else.

mod support;

#[test]
fn nouns_answer_the_protocol() {
    let world = support::World::new("nouns");
    let answered = support::run(&["nouns"], &world, &world.root);

    assert_eq!(answered.code, Some(0), "{}", answered.stderr);
    assert_eq!(
        answered.stdout,
        "app new\thome\napp declare\thome\napp list\thome\napp check\thome\n"
    );
}

#[test]
fn a_word_no_verb_is_refused() {
    let world = support::World::new("nouns-refused");
    let answered = support::run(&["app", "forge"], &world, &world.root);

    assert_eq!(answered.code, Some(2));
}
