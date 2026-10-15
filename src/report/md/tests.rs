//! The Markdown format: a table per section, `None.` for an empty one; a
//! `|` in a cell is escaped and a backtick widens the code span. The full
//! page is the golden in `output`.

use super::{code, md};
use crate::report::model::{Manual, Report};

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
