// Answer 0

#[test]
fn test_state_flags_is_match_true() {
    let flags = StateFlags(1);
    assert!(flags.is_match());
}

#[test]
fn test_state_flags_is_match_false() {
    let flags = StateFlags(0);
    assert!(!flags.is_match());
}

#[test]
fn test_state_flags_is_match_edge_case() {
    let flags = StateFlags(2);
    assert!(!flags.is_match());
}

#[test]
fn test_state_flags_is_match_another_edge_case() {
    let flags = StateFlags(255);
    assert!(flags.is_match());
}

