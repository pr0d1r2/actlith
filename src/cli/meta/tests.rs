//! `completions` and `man` come from the argv table.

use clap_complete::Shell;

use super::{completions, man};

#[test]
fn completions_name_the_command_and_its_verbs() {
    let zsh = completions(Shell::Zsh);
    assert_eq!(zsh.code, 0);
    assert!(zsh.out.contains("#compdef actlith"));
    assert!(zsh.out.contains("detect"));
    let bash = completions(Shell::Bash);
    assert!(bash.out.contains("actlith"));
}

#[test]
fn man_is_roff_naming_every_verb() {
    let page = man();
    assert_eq!(page.code, 0);
    assert!(page.out.starts_with(".ie"));
    assert!(page.out.contains(".TH actlith"));
    assert!(page.out.contains("actlith\\-gen"));
}
