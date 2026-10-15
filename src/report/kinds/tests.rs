//! The v1 kinds, found by name; any other name is not one.

use super::{lookup, names};
use crate::kind::Kind;

#[test]
fn every_v1_kind_is_found_by_its_name() {
    for name in ["gate", "build", "update-pins", "upstream"] {
        assert_eq!(lookup(name).map(Kind::name), Some(name));
    }
}

#[test]
fn another_name_is_no_kind() {
    assert!(lookup("deploy").is_none());
}

#[test]
fn the_names_are_listed_for_a_fix_hint() {
    assert_eq!(names(), "`gate`, `build`, `update-pins`, `upstream`");
}
