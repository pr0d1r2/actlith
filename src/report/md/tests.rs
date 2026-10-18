//! The Markdown format: a table per section, `None.` for an empty one; a
//! `|` in a cell is escaped and a backtick widens the code span. The full
//! page is the golden in `output`.

use super::{code, md};
use crate::report::model::{Hosted, Manual, Report, Workflow};

#[test]
fn an_empty_report_says_none_under_each_heading() {
    assert_eq!(
        md(&Report::default()),
        "# actlith report\n\n## Workflows\n\nNone.\n\n## Actions\n\nNone.\n\n\
         ## Secrets\n\nNone.\n\n## Hand-written\n\nNone.\n\n## Bots\n\n\
         None.\n"
    );
}

#[test]
fn a_pipe_in_a_cell_is_escaped() {
    let report = Report {
        hand_written: vec![Manual {
            path: "a|b.yml".to_owned(),
            reason: "x | y".to_owned(),
        }],
        ..Report::default()
    };
    assert!(md(&report).contains("| `a\\|b.yml` | x \\| y |\n"));
}

#[test]
fn a_backtick_widens_the_code_span() {
    assert_eq!(code("a"), "`a`");
    assert_eq!(code("a`b"), "`` a`b ``");
}

/// A gate on `aarch64-darwin` with Xcode 26.4 pinned.
fn pinned() -> Workflow {
    let system = "aarch64-darwin".to_owned();
    Workflow {
        id: "ci".to_owned(),
        kind: "gate",
        systems: vec![Hosted {
            system,
            runner: "macos-latest",
        }],
        cache: None,
        actions: Vec::new(),
        secrets: Vec::new(),
        xcode: Some("26.4".to_owned()),
    }
}

#[test]
fn a_pinned_xcode_follows_the_runner() {
    let report = Report {
        workflows: vec![pinned()],
        ..Report::default()
    };
    let want = "| `ci` | `gate` | `aarch64-darwin` on `macos-latest` (Xcode \
                `26.4`) | - |\n";
    assert!(md(&report).contains(want));
}
