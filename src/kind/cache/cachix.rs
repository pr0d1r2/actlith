//! The cachix step (`src/kind:V14`, V61): a fork PR gets no secret, so
//! its push is skipped while the cache still serves reads; `pushFilter`
//! keeps sources and tarballs out of the cache. Only a cachix cache that
//! can push gets one; its token is the built-in `CACHIX_AUTH_TOKEN` or
//! the secret its `## caches` row names.

use super::secrets::push_secret;
use crate::core::cache::{BinaryCache, CacheKind};
use crate::kind::{KindError, Secret, uses};
use crate::pin::PinTable;
use crate::yaml::GITHUB;
use crate::yaml::{Comment, Expr, Input, Step, Text, Value, With};

/// Store paths never pushed: sources and the nixpkgs tarball.
pub(crate) const PUSH_FILTER: &str = "(-source$|nixpkgs\\.tar\\.gz$)";

/// The push is skipped on a pull request from a fork, which has no secret.
pub(crate) const SKIP_FORK_PR: &str = "github.event_name == 'pull_request' \
    && github.event.pull_request.head.repo.full_name != github.repository";

/// The push is skipped unless the run is a push to `main`.
pub(crate) const SKIP_UNLESS_MAIN: &str =
    "github.event_name != 'push' || github.ref != 'refs/heads/main'";

const PUSH_WHY: &str = "Push what this job builds, so the next run \
    downloads it. The flake\nmakes the fleet tools follow nixpkgs-lock; \
    when that pin is newer\nthan theirs, their own caches never match and \
    every runner builds\nthem from source. Fork PRs get no secret and still \
    read the cache.";

const MAIN_WHY: &str = "Push only from main: what main builds is what \
    the cache must hold.";

const FILTER_WHY: &str =
    "Only what was built here; sources and tarballs stay out of the cache.";

/// When the cachix step pushes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CachixPush {
    /// Every run but a fork PR (`gate`).
    SkipForkPr,
    /// Only a push to `main` (`build`).
    MainOnly,
    /// Authentication only; the caller pushes selected paths explicitly.
    ExplicitMain,
    /// Never: the job only reads the cache (`upstream`).
    ReadOnly,
}

impl CachixPush {
    /// Whether a job in this mode pushes at all.
    pub(crate) fn pushes(self) -> bool {
        self != Self::ReadOnly
    }

    fn skip(self) -> Option<(&'static str, &'static str)> {
        match self {
            Self::SkipForkPr => Some((SKIP_FORK_PR, PUSH_WHY)),
            Self::MainOnly | Self::ExplicitMain => {
                Some((SKIP_UNLESS_MAIN, MAIN_WHY))
            }
            Self::ReadOnly => None,
        }
    }
}

/// The `cachix/cachix-action` step for `cache`, pinned; `None` unless
/// `cache` is a cachix cache that can push (V61).
pub(crate) fn cachix(
    pins: &PinTable,
    cache: &BinaryCache,
    push: CachixPush,
) -> Result<Option<Step>, KindError> {
    let Some(token) = push_secret(cache) else {
        return Ok(None);
    };
    // RED stub (T79): every cache kind gets the cachix step.
    let _ = CacheKind::Cachix;
    step(pins, cache, push, &token).map(Some)
}

fn step(
    pins: &PinTable,
    cache: &BinaryCache,
    push: CachixPush,
    token: &Secret,
) -> Result<Step, KindError> {
    let name = Input::new(Value::Text(Text::lit(cache.name())));
    let with = With::default().set("name", name);
    let Some((skip, why)) = push.skip() else {
        let skip = Input::new(Value::Bool(true));
        return uses(pins, "cachix/cachix-action", with.set("skipPush", skip));
    };
    let mut step = uses(
        pins,
        "cachix/cachix-action",
        pushing(with, token, skip, push == CachixPush::ExplicitMain),
    )?;
    step.comments = vec![Comment::new(why)];
    Ok(step)
}

fn pushing(with: With, secret: &Secret, skip: &str, explicit: bool) -> With {
    let filter = Input {
        comments: vec![Comment::new(FILTER_WHY)],
        value: Value::Text(Text::lit(PUSH_FILTER)),
    };
    let skip_push = skip_input(skip, explicit);
    let with = with
        .set("authToken", Input::new(expr(Some(secret.expr()))))
        .set("skipPush", skip_push);
    if explicit {
        with
    } else {
        with.set("pushFilter", filter)
    }
}

fn skip_input(skip: &str, explicit: bool) -> Input {
    if explicit {
        Input::new(Value::Bool(true))
    } else {
        Input::new(expr(Expr::new(&GITHUB, skip)))
    }
}

fn expr(e: Option<Expr>) -> Value {
    Value::Text(e.into_iter().fold(Text::default(), Text::then_expr))
}

#[cfg(test)]
mod tests;
