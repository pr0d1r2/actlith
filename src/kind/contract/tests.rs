//! The checked row, its args and cache are pipelith's `kind` types
//! (T65, V49, `src/core:V51`).

use std::any::TypeId;

use crate::core::kind as neutral;
use crate::kind::{Args, Cache, Row};

#[test]
fn the_checked_row_is_pipelith_row() {
    assert_eq!(TypeId::of::<Row>(), TypeId::of::<neutral::Row>());
}

#[test]
fn the_args_are_pipelith_args() {
    assert_eq!(TypeId::of::<Args>(), TypeId::of::<neutral::Args>());
}

#[test]
fn the_cache_is_pipelith_cache() {
    assert_eq!(TypeId::of::<Cache>(), TypeId::of::<neutral::Cache>());
}
