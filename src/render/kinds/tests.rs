//! The closed kind list (C) and the scripts each kind needs (V31).

use super::{entries, lookup, names};
use crate::script::scripts;

#[test]
fn every_v1_kind_is_found_by_name() {
    for name in ["gate", "build", "update-pins", "upstream"] {
        let found = lookup(name).map(|e| e.kind.name());
        assert_eq!(found, Some(name));
    }
}

#[test]
fn any_other_name_is_no_kind() {
    for name in ["", "Gate", "lint", "update_pins"] {
        assert!(lookup(name).is_none(), "{name}");
    }
}

#[test]
fn the_names_list_every_kind_in_order() {
    assert_eq!(names(), "`gate`, `build`, `update-pins`, `upstream`");
}

#[test]
fn each_kind_needs_its_scripts() {
    let needs = |name: &str| lookup(name).map(|e| e.scripts.to_vec());
    assert_eq!(needs("gate"), Some(vec![]));
    assert_eq!(
        needs("build"),
        Some(vec!["scripts/actlith/verify-cache.sh"])
    );
    assert_eq!(
        needs("update-pins"),
        Some(vec![
            "scripts/actlith/update-pins.sh",
            "scripts/actlith/open-pr.sh"
        ])
    );
    assert_eq!(needs("upstream"), Some(vec!["scripts/actlith/open-pr.sh"]));
}

#[test]
fn every_needed_script_is_seeded() {
    for entry in entries() {
        for path in entry.scripts {
            assert!(scripts().iter().any(|(p, _)| p == path), "{path}");
        }
    }
}
