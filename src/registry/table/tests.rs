use super::{ALL, Table};

#[test]
fn every_section_is_found_by_its_name() {
    for t in ALL {
        assert_eq!(Table::named(t.name()), Some(t));
    }
    assert_eq!(Table::named("checks"), None);
}

#[test]
fn headers_follow_the_spec() {
    let headers: Vec<&str> = ALL.iter().map(|t| t.header()).collect();
    assert_eq!(
        headers,
        [
            "id|kind|systems|cache|args",
            "action|version|sha",
            "path|reason",
            "bot|args",
        ]
    );
}

#[test]
fn the_names_hint_lists_every_section() {
    assert_eq!(
        Table::names(),
        "## workflows, ## actions, ## hand-written, ## bots"
    );
}
