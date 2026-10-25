use crate::yaml::GITHUB;
use std::collections::BTreeMap;

use crate::yaml::{
    Action, Axis, AxisName, Comment, Concurrency, Env, Expr, HEADER, Input,
    Job, Level, Matrix, Permissions, Push, Scope, Step, Text, Trigger, Uses,
    Value, With, Workflow, render, write,
};

type Built<T> = Result<T, &'static str>;

/// The golden target: this repository's own hand-written CI workflow.
/// The hand-written ci.yml this repository had before actlith generated
/// it (nix T10): the golden shape the model and builders must reproduce.
const CI: &str = include_str!("../../../tests/fixtures/yaml/ci-hand.yml");
const BUILD: &str = include_str!("../../../tests/fixtures/yaml/build.yml");
const BUMP: &str = include_str!("../../../tests/fixtures/yaml/update-pins.yml");

const SKIP_FORK: &str = "github.event_name == 'pull_request' && \
    github.event.pull_request.head.repo.full_name != github.repository";

const PUSH_WHY: &str = "Push what this job builds, so the next run \
    downloads it. The flake\nmakes the fleet tools follow nixpkgs-lock; \
    when that pin is newer\nthan theirs, their own caches never match and \
    every runner builds\nthem from source. Fork PRs get no secret and still \
    read the cache.";

const CI_WHY: &str = "Hand-written until `actlith gen` can render the gate \
    kind, then generated\n(T10, V21). Orchestration only: the gate is \
    hk.pkl, run through the same\ndev shell a laptop uses, so CI and a local \
    `hk check` cannot disagree\nabout a tool version. One job per CI system \
    (SPEC.md C).";

const NIX_CONFIG: &str = "extra-substituters = https://pr0d1r2.cachix.org\n\
    extra-trusted-public-keys = pr0d1r2.cachix.org-1:\
    NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM=\n";

fn expr(inner: &str) -> Built<Expr> {
    Expr::new(&GITHUB, inner).ok_or("bad expression")
}

fn comments(text: &str) -> Vec<Comment> {
    vec![Comment::new(text)]
}

fn said(text: &str, value: Value) -> Input {
    Input {
        comments: comments(text),
        value,
    }
}

fn lit(s: &str) -> Value {
    Value::Text(Text::lit(s))
}

fn uses(reference: &str, version: &str, with: With) -> Step {
    step(Action::Uses(Uses {
        reference: reference.to_owned(),
        version: Some(Comment::new(version)),
        with,
    }))
}

fn step(action: Action) -> Step {
    Step {
        comments: Vec::new(),
        name: None,
        id: None,
        condition: None,
        action,
        env: Env::new(),
    }
}

fn run(command: &str) -> Step {
    step(Action::Run(command.to_owned()))
}

fn job(runs_on: Text, steps: Vec<Step>) -> Job {
    Job {
        comments: Vec::new(),
        name: None,
        needs: Vec::new(),
        condition: None,
        runs_on: runs_on.into(),
        permissions: None,
        matrix: None,
        timeout_minutes: 30,
        env: Env::new(),
        steps,
    }
}

fn permissions(scopes: &[(Scope, Level)]) -> Permissions {
    Permissions(scopes.iter().copied().collect())
}

fn checkout() -> Step {
    let why = "Full history: tdd-order reads the push range, and a later \
               semver\nstep compares with the last release tag.";
    let with = With::default()
        .set("persist-credentials", Input::new(Value::Bool(false)))
        .set("fetch-depth", said(why, Value::Int(0)));
    uses(
        "actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1",
        "v7.0.1",
        with,
    )
}

fn install_nix() -> Step {
    let config = Input::new(lit(NIX_CONFIG));
    uses(
        "cachix/install-nix-action@630ae543ea3a38a9a4166f03376c02c50f408342",
        "v31.11.0",
        With::default().set("extra_nix_config", config),
    )
}

fn cachix_with() -> Built<With> {
    let token = Value::Text(Text::expr(expr("secrets.CACHIX_AUTH_TOKEN")?));
    let skip = Value::Text(Text::expr(expr(SKIP_FORK)?));
    let why = "Only what was built here; sources and tarballs stay out of \
               the cache.";
    let filter = said(why, lit("(-source$|nixpkgs\\.tar\\.gz$)"));
    Ok(With::default()
        .set("name", Input::new(lit("pr0d1r2")))
        .set("authToken", Input::new(token))
        .set("skipPush", Input::new(skip))
        .set("pushFilter", filter))
}

fn cachix() -> Built<Step> {
    let mut s = uses(
        "cachix/cachix-action@5f2d7c5294214f71b873db4b969586b980625e71",
        "v17",
        cachix_with()?,
    );
    s.comments = comments(PUSH_WHY);
    Ok(s)
}

/// The runners of `ci.yml`'s matrix, in the order its comment names
/// their systems.
const OS: [&str; 4] = [
    "ubuntu-latest",
    "ubuntu-24.04-arm",
    "macos-latest",
    "macos-15-intel",
];

fn os_matrix() -> Built<Matrix> {
    let axis = Axis {
        comments: comments(
            "x86_64-linux, aarch64-linux, aarch64-darwin, x86_64-darwin",
        ),
        values: OS.map(String::from).to_vec(),
    };
    Ok(Matrix {
        fail_fast: false,
        axes: BTreeMap::from([(AxisName::new("os").ok_or("axis")?, axis)]),
        include: Vec::new(),
    })
}

fn gate_job() -> Built<Job> {
    let gate = run("nix develop --command hk check --all --check");
    let steps = vec![checkout(), install_nix(), cachix()?, gate];
    Ok(Job {
        name: Some("gate".to_owned()),
        matrix: Some(os_matrix()?),
        ..job(Text::expr(expr("matrix.os")?), steps)
    })
}

fn gate_on() -> Trigger {
    Trigger {
        push: Some(Push {
            branches: vec!["main".to_owned()],
        }),
        pull_request: true,
        schedule: Vec::new(),
        workflow_dispatch: true,
    }
}

fn concurrency(id: &str, cancel: bool) -> Built<Option<Concurrency>> {
    Ok(Some(Concurrency {
        group: Text::lit(&format!("{id}-")).then_expr(expr("github.ref")?),
        cancel_in_progress: cancel,
    }))
}

fn workflow(name: &str, on: Trigger, jobs: Vec<(&str, Job)>) -> Workflow {
    Workflow {
        comments: Vec::new(),
        name: name.to_owned(),
        on,
        concurrency: None,
        permissions: permissions(&[(Scope::Contents, Level::Read)]),
        env: Env::new(),
        jobs: jobs.into_iter().map(|(k, j)| (k.to_owned(), j)).collect(),
    }
}

fn ci() -> Built<Workflow> {
    Ok(Workflow {
        comments: comments(CI_WHY),
        concurrency: concurrency("ci", true)?,
        ..workflow("ci", gate_on(), vec![("gate", gate_job()?)])
    })
}

#[test]
fn ci_yml_is_reproduced_byte_for_byte() -> Built<()> {
    assert_eq!(write(&ci()?), CI);
    Ok(())
}

#[test]
fn render_prepends_the_generated_header() -> Built<()> {
    let out = render(&ci()?);
    assert_eq!(out, format!("{HEADER}\n{CI}"));
    assert!(out.starts_with("# generated by actlith gen from .actlith;"));
    Ok(())
}

#[test]
fn rendering_is_deterministic() -> Built<()> {
    let workflow = ci()?;
    assert_eq!(render(&workflow), render(&workflow.clone()));
    Ok(())
}

fn bump_job() -> Job {
    let keep = Input::new(Value::Bool(true));
    let with = With::default().set("persist-credentials", keep);
    let checkout = uses("actions/checkout@abc", "v7", with);
    let script = run("scripts/actlith/update-pins.sh");
    Job {
        permissions: Some(permissions(&[
            (Scope::PullRequests, Level::Write),
            (Scope::Contents, Level::Write),
        ])),
        ..job(Text::lit("ubuntu-latest"), vec![checkout, script])
    }
}

fn bump() -> Built<Workflow> {
    let on = Trigger {
        schedule: vec!["50 6 * * *".to_owned()],
        workflow_dispatch: true,
        ..Trigger::default()
    };
    Ok(Workflow {
        concurrency: concurrency("update-pins", false)?,
        ..workflow("update-pins", on, vec![("update-pins", bump_job())])
    })
}

#[test]
fn scheduled_kind_shape_with_job_permissions() -> Built<()> {
    assert_eq!(render(&bump()?), BUMP);
    Ok(())
}

fn pair(os: &str, system: &str) -> BTreeMap<String, String> {
    BTreeMap::from([
        ("os".to_owned(), os.to_owned()),
        ("system".to_owned(), system.to_owned()),
    ])
}

fn include_matrix() -> Matrix {
    Matrix {
        fail_fast: false,
        axes: BTreeMap::new(),
        include: vec![
            pair("ubuntu-latest", "x86_64-linux"),
            pair("macos-latest", "aarch64-darwin"),
        ],
    }
}

fn build_job() -> Built<Job> {
    let system = Text::expr(expr("matrix.system")?);
    let matrix = include_matrix();
    let build = run("nix build --print-build-logs .#default");
    Ok(Job {
        matrix: Some(matrix),
        timeout_minutes: 10,
        env: Env::from([("SYSTEM".to_owned(), system)]),
        ..job(Text::expr(expr("matrix.os")?), vec![build])
    })
}

fn verify_step() -> Built<Step> {
    let mut verify = run("scripts/actlith/verify-cache.sh");
    verify.name = Some("verify cache".to_owned());
    verify.id = Some("verify".to_owned());
    verify.condition = Some(expr("success()")?);
    verify.env = Env::from([("CACHE".to_owned(), Text::lit("pr0d1r2"))]);
    Ok(verify)
}

fn verify_job() -> Built<Job> {
    let main = "github.event_name == 'push' && github.ref == 'refs/heads/main'";
    Ok(Job {
        comments: comments("a cachix push failure does not fail its job"),
        name: Some("verify-cache".to_owned()),
        needs: vec!["build".to_owned()],
        condition: Some(expr(main)?),
        timeout_minutes: 10,
        ..job(Text::lit("ubuntu-latest"), vec![verify_step()?])
    })
}

fn build() -> Built<Workflow> {
    let jobs = vec![("verify-cache", verify_job()?), ("build", build_job()?)];
    Ok(Workflow {
        permissions: Permissions::default(),
        env: Env::from([("NIX_CONFIG".to_owned(), Text::lit("a = b"))]),
        ..workflow("build", gate_on(), jobs)
    })
}

#[test]
fn needs_if_include_env_and_empty_permissions_shape() -> Built<()> {
    assert_eq!(render(&build()?), BUILD);
    Ok(())
}
