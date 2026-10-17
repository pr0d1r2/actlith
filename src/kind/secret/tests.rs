//! Secret names and their expression (T17, V14).

use crate::kind::Secret;

#[test]
fn the_cachix_secret_is_the_documented_one() {
    let secret = Secret::cachix();
    let name = secret.as_ref().map(Secret::name);
    assert_eq!(name, Some("CACHIX_AUTH_TOKEN"));
    let purpose = secret.as_ref().map(Secret::purpose);
    assert_eq!(purpose, Some("cachix push"));
    let expr = secret.map(|s| s.expr().text().to_owned());
    assert_eq!(expr.as_deref(), Some("${{ secrets.CACHIX_AUTH_TOKEN }}"));
}

#[test]
fn a_well_formed_name_is_taken() {
    for name in ["GITHUB_TOKEN", "_X", "bot_token2"] {
        assert!(Secret::new(name, "pr").is_some(), "{name}");
    }
}

#[test]
fn a_malformed_name_is_refused() {
    for name in ["", "2X", "A-B", "A }} B", "A.B", "A B"] {
        assert_eq!(Secret::new(name, "pr"), None, "{name}");
    }
}
