//! The report as a JSON document (V33): `schema_version` first, then
//! `workflows`, `hand_written`, `bots`. A field rename or removal bumps
//! [`SCHEMA_VERSION`].

use super::json::Json;
use super::model::Report;

/// The JSON schema version (V33).
pub(crate) const SCHEMA_VERSION: u32 = 0;

/// The document of `report`.
pub(crate) fn document(_report: &Report) -> Json {
    Json::Null
}

#[cfg(test)]
mod tests;
