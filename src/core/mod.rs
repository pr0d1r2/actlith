//! `src/core`: the one node that names `pipelith` (src C, V51). It
//! re-exports pipelith's neutral modules, one per switch (`tests:T50`);
//! every other node reaches them through `crate::core`.

pub(crate) use pipelith::{files, kind, registry, system, yaml};
