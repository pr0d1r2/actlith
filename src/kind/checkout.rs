//! The checkout step (V3): credentials are dropped unless the job pushes.

use super::error::KindError;
use super::step::uses;
use crate::pin::PinTable;
use crate::yaml::{Comment, Input, Step, Value, With};

/// How a job checks out the repository.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Checkout {
    /// The job pushes, so it keeps the credentials checkout stores.
    pub(crate) pushes: bool,
    /// `fetch-depth`; `None` leaves checkout's default (one commit).
    pub(crate) fetch_depth: Option<u32>,
    /// Why the depth is needed, written above it.
    pub(crate) why_depth: Option<Comment>,
}

/// The `actions/checkout` step, pinned.
pub(crate) fn checkout(
    pins: &PinTable,
    how: &Checkout,
) -> Result<Step, KindError> {
    let keep = Input::new(Value::Bool(how.pushes));
    let mut with = With::default().set("persist-credentials", keep);
    if let Some(depth) = how.fetch_depth {
        let input = Input {
            comments: how.why_depth.iter().cloned().collect(),
            value: Value::Int(u64::from(depth)),
        };
        with = with.set("fetch-depth", input);
    }
    uses(pins, "actions/checkout", with)
}

#[cfg(test)]
mod tests;
