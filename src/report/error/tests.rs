//! Every refusal prints the error it carries.

use crate::kind::KindError;
use crate::registry::RegistryError;
use crate::report::ReportError;

#[test]
fn a_row_error_prints_its_line_and_fix() {
    let error = ReportError::Row(RegistryError::new(4, "bad", "good"));
    assert_eq!(error.to_string(), ".actlith:4: bad\n  fix: good");
}

#[test]
fn a_kind_error_prints_as_itself() {
    let kind = KindError::row(5, "what", "fix");
    assert_eq!(
        ReportError::from(kind.clone()).to_string(),
        kind.to_string()
    );
}
