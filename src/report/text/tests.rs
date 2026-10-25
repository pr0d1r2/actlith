//! The text format: three sections, `-` for an empty one or no cache.
//! The full text is the golden in `output`.

use super::text;
use crate::report::model::{Hosted, Report, Workflow};

#[test]
fn an_empty_report_marks_each_section_empty() {
    assert_eq!(
        text(&Report::default()),
        "workflows\n  -\nhand-written\n  -\nbots\n  -\n"
    );
}

fn bare() -> Workflow {
    Workflow {
        id: "ci".to_owned(),
        kind: "gate",
        systems: vec![Hosted {
            system: "x86_64-linux".to_owned(),
            runner: "ubuntu-latest".to_owned(),
        }],
        cache: None,
        actions: Vec::new(),
        secrets: Vec::new(),
        xcode: None,
    }
}

#[test]
fn a_workflow_with_no_cache_shows_a_dash() {
    let report = Report {
        workflows: vec![bare()],
        ..Report::default()
    };
    assert!(text(&report).starts_with(
        "workflows\n  ci: kind gate, cache -\n    system x86_64-linux on \
         ubuntu-latest\nhand-written\n"
    ));
}

/// A gate on `aarch64-darwin` with Xcode 26.4 pinned.
fn pinned() -> Report {
    let system = "aarch64-darwin".to_owned();
    let systems = vec![Hosted {
        system,
        runner: "macos-latest".to_owned(),
    }];
    let xcode = Some("26.4".to_owned());
    let workflows = vec![Workflow {
        systems,
        xcode,
        ..bare()
    }];
    Report {
        workflows,
        ..Report::default()
    }
}

#[test]
fn a_pinned_xcode_sits_beside_the_runner() {
    let want = "\n    system aarch64-darwin on macos-latest, xcode 26.4\n";
    assert!(text(&pinned()).contains(want));
}
