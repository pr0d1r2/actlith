//! Each v1 bot names the GitHub App it assumes; any other bot none.

use super::{NOTE, app};

#[test]
fn each_v1_bot_names_its_app() {
    assert_eq!(app("coderabbit"), Some("CodeRabbit"));
    assert_eq!(app("greptile"), Some("Greptile"));
    assert_eq!(app("socket"), Some("Socket"));
}

#[test]
fn another_bot_names_none() {
    assert_eq!(app("bugbot"), None);
}

#[test]
fn the_note_points_at_the_free_tier() {
    assert_eq!(NOTE, "install the app; free tier per docs/SERVICES.md");
}
