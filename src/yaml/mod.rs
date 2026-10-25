//! `src/yaml`: workflow model & own YAML writer; malformed YAML never
//! representable (V5). A workflow is built from typed parts, lowered to a
//! YAML tree in a fixed key order, and printed with fixed quoting; no
//! caller text is ever pasted into YAML text.
#![cfg_attr(
    not(test),
    expect(
        dead_code,
        unused_imports,
        reason = "used by src/kind and src/render in a later wave"
    )
)]

mod dialect;
mod job;
mod render;
mod step;
mod workflow;

pub(crate) use crate::core::yaml::{Comment, Expr, Text, Value};
pub(crate) use dialect::GITHUB;
pub(crate) use job::{Axis, AxisName, Job, Matrix, RunsOn};
pub(crate) use render::{HEADER, render, write};
pub(crate) use step::{Action, Input, Step, Uses, With};
pub(crate) use workflow::{
    Concurrency, Env, Level, Permissions, Push, Scope, Trigger, Workflow,
};
