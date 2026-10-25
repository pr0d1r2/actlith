use super::{DEFAULT_SYSTEMS, System, SystemError, parse_system, runner};

const ALL: [(&str, System); 4] = [
    ("x86_64-linux", System::X86_64Linux),
    ("aarch64-linux", System::Aarch64Linux),
    ("aarch64-darwin", System::Aarch64Darwin),
    ("x86_64-darwin", System::X86_64Darwin),
];

#[test]
fn parses_every_known_name() {
    for (name, system) in ALL {
        assert_eq!(parse_system(name), Ok(system), "{name}");
    }
}

#[test]
fn display_round_trips_through_parse() {
    for (name, system) in ALL {
        assert_eq!(system.to_string(), name);
    }
}

#[test]
fn unknown_name_is_refused_by_name() {
    let err = parse_system("riscv64-linux");
    assert_eq!(err, Err(SystemError::Unknown("riscv64-linux".to_owned())));
}

#[test]
fn parse_is_exact() {
    for name in ["X86_64-linux", " x86_64-linux", "x86_64-linux ", ""] {
        assert!(parse_system(name).is_err(), "{name:?}");
    }
}

#[test]
fn unknown_message_names_it_and_the_known_ones() {
    let msg = SystemError::Unknown("riscv64-linux".to_owned()).to_string();
    assert_eq!(
        msg,
        "unknown system `riscv64-linux`; known: x86_64-linux, \
         aarch64-linux, aarch64-darwin, x86_64-darwin"
    );
}

#[test]
fn every_system_maps_to_its_runner() {
    assert_eq!(runner(System::X86_64Linux), "ubuntu-latest");
    assert_eq!(runner(System::Aarch64Linux), "ubuntu-24.04-arm");
    assert_eq!(runner(System::Aarch64Darwin), "macos-latest");
    assert_eq!(runner(System::X86_64Darwin), "macos-15-intel");
}

#[test]
fn default_systems_are_all_four() {
    let all: Vec<System> = ALL.iter().map(|&(_, s)| s).collect();
    assert_eq!(DEFAULT_SYSTEMS.as_slice(), all.as_slice());
}

#[test]
fn every_default_system_has_its_own_runner() {
    let labels: Vec<&str> = DEFAULT_SYSTEMS.into_iter().map(runner).collect();
    let mut unique = labels.clone();
    unique.dedup();
    assert_eq!(unique, labels, "one runner per system");
}

#[test]
fn error_is_a_std_error() {
    let err: &dyn std::error::Error =
        &SystemError::Unknown("riscv64-linux".to_owned());
    assert!(err.source().is_none());
}

#[test]
fn system_is_pipelith_system() {
    use std::any::TypeId;
    let ours = TypeId::of::<System>();
    assert_eq!(ours, TypeId::of::<crate::core::system::System>());
}
