//! The report as a JSON document (V33): `schema_version` first, then
//! `workflows`, `hand_written`, `bots`. A field rename or removal bumps
//! [`SCHEMA_VERSION`].

use super::json::Json;
use super::model::{App, Hosted, Manual, Need, Report, Workflow};
use crate::pin::Uses;

/// The JSON schema version (V33).
pub(crate) const SCHEMA_VERSION: u32 = 1;

/// The document of `report`.
pub(crate) fn document(report: &Report) -> Json {
    Json::Object(vec![
        ("schema_version", Json::Number(SCHEMA_VERSION)),
        ("workflows", array(&report.workflows, workflow)),
        ("hand_written", array(&report.hand_written, manual)),
        ("bots", array(&report.bots, app)),
    ])
}

fn array<T>(items: &[T], each: fn(&T) -> Json) -> Json {
    Json::Array(items.iter().map(each).collect())
}

fn s(text: &str) -> Json {
    Json::String(text.to_owned())
}

fn workflow(w: &Workflow) -> Json {
    let cache = w.cache.as_deref().map_or(Json::Null, s);
    Json::Object(vec![
        ("id", s(&w.id)),
        ("kind", s(w.kind)),
        ("systems", array(&w.systems, hosted)),
        ("cache", cache),
        ("actions", array(&w.actions, uses)),
        ("secrets", array(&w.secrets, need)),
    ])
}

fn hosted(h: &Hosted) -> Json {
    Json::Object(vec![("system", s(&h.system)), ("runner", s(h.runner))])
}

fn uses(u: &Uses) -> Json {
    Json::Object(vec![
        ("action", s(&u.action)),
        ("version", s(&u.version)),
        ("sha", s(&u.sha)),
    ])
}

fn need(n: &Need) -> Json {
    Json::Object(vec![("name", s(&n.name)), ("purpose", s(n.purpose))])
}

fn manual(m: &Manual) -> Json {
    Json::Object(vec![("path", s(&m.path)), ("reason", s(&m.reason))])
}

fn app(a: &App) -> Json {
    Json::Object(vec![
        ("bot", s(&a.bot)),
        ("app", s(a.name)),
        ("note", s(a.note)),
    ])
}

#[cfg(test)]
mod tests;
