//! The https push (V61, V62): the seeded `push-https.sh` copies what the
//! job built to the cache over authenticated `PUT`. Its netrc comes from
//! the row's push secret and its url from the row, both through `env:`,
//! never spliced into `run:` (`src/kind:V3`).

use super::cachix::CachixPush;
use super::secrets::{push_secret, secret_expr};
use crate::core::cache::{BinaryCache, CacheKind};
use crate::kind::{KindError, Secret, TRUSTED, run};
use crate::yaml::{Comment, Expr, GITHUB, Step, Text};

/// The seeded script an https push runs (V62).
pub(crate) const PUSH_SCRIPT: &str = "scripts/actlith/push-https.sh";

/// The push runs on a push to `main` only.
const MAIN_PUSH: &str =
    "github.event_name == 'push' && github.ref == 'refs/heads/main'";

const FORK_WHY: &str = "Push what this job builds to the https cache, with \
    the netrc its push\nsecret holds. Fork PRs get no secret, so they skip \
    the push.";

const MAIN_WHY: &str = "Push only from main: what main builds is what \
    the cache must hold.";

/// The `push-https.sh <attr>` step for `cache` in `mode`; `None` unless
/// `cache` is an https cache with a push secret and `mode` pushes.
pub(crate) fn push_https(
    line: usize,
    cache: &BinaryCache,
    mode: CachixPush,
    attr: &str,
) -> Result<Option<Step>, KindError> {
    let Some(netrc) = netrc(cache, mode) else {
        return Ok(None);
    };
    let command = format!("bash {PUSH_SCRIPT} {attr}");
    let mut step = run(line, &command, &[("NETRC", netrc.expr())])?;
    https_env(&mut step, cache, Some(&netrc));
    let (when, why) = if mode == CachixPush::SkipForkPr {
        (TRUSTED, FORK_WHY)
    } else {
        (MAIN_PUSH, MAIN_WHY)
    };
    step.condition = Expr::new(&GITHUB, when);
    step.comments = vec![Comment::new(why)];
    Ok(Some(step))
}

/// Whether `cache` in `mode` gets the https push step, so the workflow
/// needs [`PUSH_SCRIPT`].
pub(crate) fn https_push(
    cache: Option<&BinaryCache>,
    mode: CachixPush,
) -> bool {
    cache.and_then(|c| netrc(c, mode)).is_some()
}

/// The netrc secret of an https `cache` that pushes in `mode`.
fn netrc(cache: &BinaryCache, mode: CachixPush) -> Option<Secret> {
    let https = cache.kind() == CacheKind::Https && mode.pushes();
    push_secret(cache).filter(|_| https)
}

/// The env an https script reads beside `NETRC`: `URL`, and with a
/// netrc the `CACHE` and `NETRC_SECRET` names its run-time https check
/// reports instead of the url (V65).
pub(crate) fn https_env(
    step: &mut Step,
    cache: &BinaryCache,
    netrc: Option<&Secret>,
) {
    step.env.insert("URL".to_owned(), url(cache));
    if let Some(netrc) = netrc {
        step.env.insert("CACHE".to_owned(), Text::lit(cache.name()));
        let name = Text::lit(netrc.name());
        step.env.insert("NETRC_SECRET".to_owned(), name);
    }
}

/// The url of `cache` as an env value: the literal, or its secret.
pub(crate) fn url(cache: &BinaryCache) -> Text {
    let value = cache.url();
    let literal = || Text::lit(value.literal().unwrap_or_default());
    secret_expr(value).map_or_else(literal, Text::expr)
}

#[cfg(test)]
mod tests;
