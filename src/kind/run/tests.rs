//! `run:` holds one command and no expression; values go through `env:`
//! (T17, V3).

use crate::kind::{KindError, run};
use crate::yaml::GITHUB;
use crate::yaml::{Action, Expr, Text};

fn refused(command: &str, env: &[(&str, Expr)]) -> String {
    run(5, command, env)
        .err()
        .map(|e| e.to_string())
        .unwrap_or_default()
}

const FIX: &str = "\n  fix: write one command line; pass values through env, \
    longer logic through a script";

#[test]
fn a_command_runs_with_its_values_in_env() -> Result<(), KindError> {
    let reference = Expr::new(&GITHUB, "github.ref")
        .ok_or_else(|| KindError::row(0, "bad expression", "-"))?;
    let step = run(5, "scripts/x.sh \"$REF\"", &[("REF", reference.clone())])?;
    assert_eq!(step.action, Action::Run("scripts/x.sh \"$REF\"".to_owned()));
    let env: Vec<_> = step.env.into_iter().collect();
    assert_eq!(env, [("REF".to_owned(), Text::expr(reference))]);
    Ok(())
}

#[test]
fn an_expression_inside_run_is_refused() {
    assert_eq!(
        refused("echo ${{ github.head_ref }}", &[]),
        format!(
            ".actlith:5: command `echo ${{{{ github.head_ref }}}}` holds \
             `${{{{`, an expression inside `run:`{FIX}"
        )
    );
}

#[test]
fn a_multi_line_or_empty_command_is_refused() {
    assert_eq!(
        refused("a\nb", &[]),
        format!(".actlith:5: command `a\\nb` spans more than one line{FIX}")
    );
    assert_eq!(
        refused(" ", &[]),
        format!(".actlith:5: command ` ` is empty{FIX}")
    );
    assert!(refused("a\rb", &[]).contains("spans more than one line"));
}

#[test]
fn a_bad_env_name_is_refused() {
    for name in ["", "1A", "A-B", "A B"] {
        let value = Expr::new(&GITHUB, "github.ref")
            .into_iter()
            .map(|e| (name, e));
        let env: Vec<(&str, Expr)> = value.collect();
        let error = refused("true", &env);
        assert!(error.contains("is not a shell variable name"), "{name}");
    }
}
