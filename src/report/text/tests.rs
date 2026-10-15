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
            runner: "ubuntu-latest",
        }],
        cache: None,
        actions: Vec::new(),
        secrets: Vec::new(),
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
