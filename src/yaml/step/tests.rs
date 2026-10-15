use super::{Action, Input, Step, Uses, With};
use crate::yaml::node::{Entry, Node, document};
use crate::yaml::{Comment, Env, Text, Value};

fn run(cmd: &str) -> Step {
    Step {
        comments: vec![Comment::new("first")],
        name: None,
        id: None,
        condition: None,
        action: Action::Run(cmd.to_owned()),
        env: Env::new(),
    }
}

fn out(step: &Step) -> String {
    let steps = Node::Seq(vec![step.entries()]);
    document(&[], &[Entry::fixed("steps", steps)])
}

#[test]
fn run_step_carries_comment_above_its_dash() {
    let want = "---\nsteps:\n  # first\n  - run: make\n";
    assert_eq!(out(&run("make")), want);
}

#[test]
fn with_set_replaces_in_place_keeps_order() {
    let one = |n| Input::new(Value::Int(n));
    let with = With::default()
        .set("b", one(1))
        .set("a", one(2))
        .set("b", one(3));
    let mut step = run("x");
    step.action = Action::Uses(Uses {
        reference: "o/r@s".to_owned(),
        version: None,
        with,
    });
    step.env = Env::from([("K".to_owned(), Text::lit("v"))]);
    let want = "  - uses: o/r@s\n    with:\n      b: 3\n      a: 2\n    \
                env:\n      K: v\n";
    assert!(out(&step).ends_with(want));
}

#[test]
fn uses_without_inputs_has_no_with() {
    let mut step = run("x");
    step.comments = Vec::new();
    step.action = Action::Uses(Uses {
        reference: "o/r@s".to_owned(),
        version: Some(Comment::new("v1")),
        with: With::default(),
    });
    assert_eq!(out(&step), "---\nsteps:\n  - uses: o/r@s  # v1\n");
}
