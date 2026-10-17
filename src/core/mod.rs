//! `src/core`: the one node that names `pipelith` (src C, V51). It
//! re-exports pipelith's neutral modules, one per switch (`tests:T50`);
//! every other node reaches them through `crate::core`.

#![cfg_attr(
    not(test),
    expect(
        unused_imports,
        reason = "src/gen switches to files in the next commit"
    )
)]

pub(crate) use pipelith::{files, system};
