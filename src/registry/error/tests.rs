use super::RegistryError;

#[test]
fn an_error_names_line_what_and_fix() {
    let e = RegistryError::new(3, "bad", "do better");
    assert_eq!(e, RegistryError::new(3, "bad", "do better"));
    assert_eq!(e.to_string(), ".actlith:3: bad\n  fix: do better");
}
