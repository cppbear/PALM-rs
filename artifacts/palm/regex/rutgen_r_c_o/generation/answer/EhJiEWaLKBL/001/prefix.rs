// Answer 0

#[test]
fn test_state_flags_default() {
    let flags = StateFlags::default();
    let mut formatter = fmt::Formatter::new();
    flags.fmt(&mut formatter);
}

#[test]
fn test_state_flags_match_only() {
    let mut flags = StateFlags(0b00000001);
    let mut formatter = fmt::Formatter::new();
    flags.fmt(&mut formatter);
}

#[test]
fn test_state_flags_word_only() {
    let mut flags = StateFlags(0b00000010);
    let mut formatter = fmt::Formatter::new();
    flags.fmt(&mut formatter);
}

#[test]
fn test_state_flags_empty_only() {
    let mut flags = StateFlags(0b00000100);
    let mut formatter = fmt::Formatter::new();
    flags.fmt(&mut formatter);
}

#[test]
fn test_state_flags_match_and_word() {
    let mut flags = StateFlags(0b00000011);
    let mut formatter = fmt::Formatter::new();
    flags.fmt(&mut formatter);
}

#[test]
fn test_state_flags_match_and_empty() {
    let mut flags = StateFlags(0b00000101);
    let mut formatter = fmt::Formatter::new();
    flags.fmt(&mut formatter);
}

#[test]
fn test_state_flags_word_and_empty() {
    let mut flags = StateFlags(0b00000110);
    let mut formatter = fmt::Formatter::new();
    flags.fmt(&mut formatter);
}

#[test]
fn test_state_flags_all() {
    let mut flags = StateFlags(0b00000111);
    let mut formatter = fmt::Formatter::new();
    flags.fmt(&mut formatter);
}

#[test]
fn test_state_flags_none() {
    let mut flags = StateFlags(0b00000000);
    let mut formatter = fmt::Formatter::new();
    flags.fmt(&mut formatter);
}

