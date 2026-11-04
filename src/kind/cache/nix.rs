//! The install-nix step, trusting the row's cache as a substituter
//! (V60): literal values as they are; a secret value or a read netrc
//! (`netrc` V66) only where the run gets it, so a fork PR installs nix
//! without the cache, never with an empty substituter.

use super::cachix::SKIP_FORK_PR;
use super::netrc::{netrc_file, read_netrc};
use super::secrets::secret_expr;
use crate::core::cache::{BinaryCache, CacheValue};
use crate::kind::{KindError, TRUSTED, uses};
use crate::pin::PinTable;
use crate::yaml::{Comment, Expr, GITHUB, Input, Step, Text, Value, With};

const ACTION: &str = "cachix/install-nix-action";

const SECRET_WHY: &str = "The cache's url or key is a secret, which a \
    fork PR does not get: this\nrun trusts the cache, a fork PR's runs \
    without it.";

const NETRC_WHY: &str = "The cache is read with a netrc secret, which a \
    fork PR does not get:\nthis run trusts the cache, a fork PR's runs \
    without it.";

const FORK_WHY: &str =
    "A fork PR: no cache secret, so no cache rather than an empty one.";

/// The `cachix/install-nix-action` step, pinned; with a cache, its
/// `extra-substituters` and `extra-trusted-public-keys`. A cache with a
/// secret value gets two steps: trusting it unless the run is a fork PR,
/// and without it on a fork PR; one with a read netrc gets the step
/// writing it first, and `netrc-file` in the trusting one. `line` is the
/// row's, for that step's errors.
pub(crate) fn install_nix(
    line: usize,
    pins: &PinTable,
    cache: Option<&BinaryCache>,
) -> Result<Vec<Step>, KindError> {
    let Some(cache) = cache else {
        return Ok(vec![plain(pins)?]);
    };
    if let Some(config) = cache.nix_config() {
        return Ok(vec![trusting(pins, Text::lit(&config))?]);
    }
    let read = read_netrc(line, cache)?;
    let why = if read.is_some() {
        NETRC_WHY
    } else {
        SECRET_WHY
    };
    Ok(read.into_iter().chain(guarded(pins, cache, why)?).collect())
}

/// install-nix trusting `cache` unless the run is a fork PR, saying
/// `why`, then without it on a fork PR.
fn guarded(
    pins: &PinTable,
    cache: &BinaryCache,
    why: &str,
) -> Result<[Step; 2], KindError> {
    let mut trusted = trusting(pins, netrc_file(config(cache), cache))?;
    trusted.condition = Expr::new(&GITHUB, TRUSTED);
    trusted.comments = vec![Comment::new(why)];
    let mut fork = plain(pins)?;
    fork.condition = Expr::new(&GITHUB, SKIP_FORK_PR);
    fork.comments = vec![Comment::new(FORK_WHY)];
    Ok([trusted, fork])
}

fn plain(pins: &PinTable) -> Result<Step, KindError> {
    uses(pins, ACTION, With::default())
}

fn trusting(pins: &PinTable, config: Text) -> Result<Step, KindError> {
    let config = Input::new(Value::Text(config));
    uses(
        pins,
        ACTION,
        With::default().set("extra_nix_config", config),
    )
}

/// One `setting = value` line per nix setting, a secret value as its
/// `${{ secrets.<name> }}`.
fn config(cache: &BinaryCache) -> Text {
    let line = |text: Text, (setting, value): (&str, &CacheValue)| {
        let text = text.then_lit(&format!("{setting} = "));
        value_of(text, value).then_lit("\n")
    };
    cache.nix_settings().into_iter().fold(Text::default(), line)
}

fn value_of(text: Text, value: &CacheValue) -> Text {
    match secret_expr(value) {
        Some(e) => text.then_expr(e),
        None => text.then_lit(value.literal().unwrap_or_default()),
    }
}

#[cfg(test)]
mod tests;
