//! What both bump kinds share (T22, T23, R5, `src/kind:V3`,
//! `src/kind:V14`).

use crate::bump::shared::{
    GITHUB_TOKEN, SYSTEMS, TOKEN, bump_job, plain, pushing_checkout, runs_on,
    token,
};
use crate::kind::{ArgSpec, Args, Row, Secret, TIMEOUT, parse_args};
use crate::pin::PinTable;
use crate::registry::parse;
use crate::system::System;
use crate::yaml::{Action, Level, Scope, Step, Text};

const SCHEMA: [ArgSpec; 3] = [
    ArgSpec {
        key: "word",
        required: false,
        default: None,
    },
    TOKEN,
    TIMEOUT,
];

fn args(cell: &str) -> Result<Args, String> {
    let text = format!(
        "format 1\n## workflows\nid|kind|systems|cache|args\nw|k|-|-|{cell}\n"
    );
    let registry = parse(&text).map_err(|e| e.to_string())?;
    let row = registry.workflows.first().ok_or("no row")?;
    parse_args(&SCHEMA, row).map_err(|e| e.to_string())
}

fn row(systems: &[System]) -> Result<Row, String> {
    Ok(Row {
        id: "w".to_owned(),
        line: 7,
        systems: systems.to_vec(),
        cache: None,
        args: args("-")?,
    })
}

fn with_keys(step: &Step) -> String {
    match &step.action {
        Action::Uses(u) => format!("{:?}", u.with),
        Action::Run(_) => String::new(),
    }
}

#[test]
fn the_default_token_is_github_token() -> Result<(), String> {
    let secret = token(&args("-")?).map_err(|e| e.to_string())?;
    assert_eq!(secret.name(), GITHUB_TOKEN);
    assert_eq!(secret.purpose(), "push the bump branch and open its PR");
    Ok(())
}

#[test]
fn a_named_token_is_the_secret_read() -> Result<(), String> {
    let secret = token(&args("token=APP_TOKEN")?).map_err(|e| e.to_string())?;
    assert_eq!(secret.expr().text(), "${{ secrets.APP_TOKEN }}");
    Ok(())
}

#[test]
fn a_token_that_is_no_secret_name_is_refused() -> Result<(), String> {
    let error = token(&args("token=A-B")?).err().map(|e| e.to_string());
    let error = error.unwrap_or_default();
    assert!(
        error.contains("token `A-B` is not a secret name"),
        "{error}"
    );
    Ok(())
}

#[test]
fn one_system_gives_its_runner() -> Result<(), String> {
    let label = runs_on(&row(&SYSTEMS)?).map_err(|e| e.to_string())?;
    assert_eq!(label, Text::lit("ubuntu-latest"));
    Ok(())
}

#[test]
fn none_or_two_systems_are_refused() -> Result<(), String> {
    let two = [System::X86_64Linux, System::Aarch64Linux];
    for (systems, count) in [(&two[..], 2), (&[][..], 0)] {
        let error = runs_on(&row(systems)?).err().map(|e| e.to_string());
        let error = error.unwrap_or_default();
        assert!(error.contains(&format!("{count} systems given")), "{error}");
    }
    Ok(())
}

#[test]
fn a_system_without_a_runner_is_refused() -> Result<(), String> {
    let error = runs_on(&row(&[System::X86_64Darwin])?).err();
    let error = error.map(|e| e.to_string()).unwrap_or_default();
    assert!(error.contains(".actlith:7"), "{error}");
    Ok(())
}

#[test]
fn the_checkout_keeps_credentials() -> Result<(), String> {
    let pins = PinTable::builtin();
    let secret = Secret::new(GITHUB_TOKEN, "t").ok_or("no secret")?;
    let step = pushing_checkout(&pins, &secret).map_err(|e| e.to_string())?;
    let with = with_keys(&step);
    assert!(with.contains("persist-credentials") && with.contains("true"));
    assert!(!with.contains("token"), "{with}");
    assert_eq!(step.comments.len(), 1);
    Ok(())
}

#[test]
fn a_named_token_checks_out_with_it() -> Result<(), String> {
    let pins = PinTable::builtin();
    let secret = Secret::new("APP_TOKEN", "t").ok_or("no secret")?;
    let step = pushing_checkout(&pins, &secret).map_err(|e| e.to_string())?;
    assert!(with_keys(&step).contains("secrets.APP_TOKEN"));
    Ok(())
}

#[test]
fn the_job_writes_contents_and_pull_requests_only() -> Result<(), String> {
    let job =
        bump_job(&row(&SYSTEMS)?, Vec::new()).map_err(|e| e.to_string())?;
    let scopes: Vec<_> = job.permissions.ok_or("none")?.0.into_iter().collect();
    let write = Level::Write;
    let want = [(Scope::Contents, write), (Scope::PullRequests, write)];
    assert_eq!(scopes, want);
    assert_eq!(job.timeout_minutes, 30);
    Ok(())
}

#[test]
fn a_plain_word_passes() -> Result<(), String> {
    let given = args("word=.#hk")?;
    assert_eq!(plain(&given, "word").map_err(|e| e.to_string())?, ".#hk");
    let path = args("word=scripts/bump-hk.sh")?;
    assert!(plain(&path, "word").is_ok());
    Ok(())
}

#[test]
fn an_empty_dashed_or_spaced_word_is_refused() -> Result<(), String> {
    for cell in ["-", "word=-rf", "word=a b", "word=$(x)"] {
        let given = args(cell)?;
        let error = plain(&given, "word").err().map(|e| e.to_string());
        let error = error.unwrap_or_default();
        assert!(error.contains("not a plain word"), "{cell}: {error}");
    }
    Ok(())
}
