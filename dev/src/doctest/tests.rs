const PKLITH: &str = include_str!("../../../.pklith");

#[test]
fn doctest_runs_for_every_workspace_member() {
    assert!(PKLITH.contains(
        "doctest|test|cargo|**/*.rs, Cargo.toml|cargo test --workspace --doc -q|-|-|a doc example failed"
    ));
}
