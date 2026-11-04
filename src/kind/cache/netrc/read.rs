//! The read netrc of a private https cache (V66): a trusted-only step
//! runs the seeded `read-netrc.sh`, which checks the url is https (V65)
//! and writes the netrc for nix; the trusted install-nix step points
//! `netrc-file` at it. Its secret is the row's `read=`, never the push
//! one, and reaches the script through `env:` only (`src/kind:V3`).

use crate::core::cache::BinaryCache;
use crate::kind::{KindError, Secret, TRUSTED, https_env, run};
use crate::yaml::{Comment, Expr, GITHUB, Step, Text};

/// The seeded script that writes the read netrc (V66).
pub(crate) const READ_SCRIPT: &str = "scripts/actlith/read-netrc.sh";

/// What a read netrc secret is read for.
const READ: &str = "cache read (netrc)";

/// The file under the runner's temp dir the script writes, nix reads.
const FILE: &str = "/read-netrc";

const WHY: &str = "Write the netrc nix reads this cache with, mode 600, \
    after checking its\nurl is https. Fork PRs get no secret, so they run \
    without the cache.";

/// The secret holding `cache`'s read netrc; `None` without one.
pub(crate) fn read_secret(cache: &BinaryCache) -> Option<Secret> {
    let name = cache.read_netrc()?.secret()?;
    Secret::new(name, READ)
}

/// The step writing `cache`'s read netrc; `None` without one.
pub(crate) fn read_netrc(
    line: usize,
    cache: &BinaryCache,
) -> Result<Option<Step>, KindError> {
    let Some(netrc) = read_secret(cache) else {
        return Ok(None);
    };
    let command = format!("bash {READ_SCRIPT}");
    let mut step = run(line, &command, &[("NETRC", netrc.expr())])?;
    https_env(&mut step, cache, Some(&netrc));
    step.condition = Expr::new(&GITHUB, TRUSTED);
    step.comments = vec![Comment::new(WHY)];
    Ok(Some(step))
}

/// `text` with the `netrc-file` line for `cache`'s read netrc, if any.
pub(crate) fn netrc_file(text: Text, cache: &BinaryCache) -> Text {
    let temp = Expr::new(&GITHUB, "runner.temp");
    match temp.filter(|_| read_secret(cache).is_some()) {
        Some(temp) => text
            .then_lit("netrc-file = ")
            .then_expr(temp)
            .then_lit(&format!("{FILE}\n")),
        None => text,
    }
}

#[cfg(test)]
mod tests;
