// Answer 0

#[test]
fn test_is_negation_with_negation() {
    let item = FlagsItemKind::Negation;
    assert!(item.is_negation());
}

#[test]
fn test_is_negation_with_flag_case_insensitive() {
    let item = FlagsItemKind::Flag(Flag::CaseInsensitive);
    assert!(!item.is_negation());
}

#[test]
fn test_is_negation_with_flag_multi_line() {
    let item = FlagsItemKind::Flag(Flag::MultiLine);
    assert!(!item.is_negation());
}

#[test]
fn test_is_negation_with_flag_dot_matches_new_line() {
    let item = FlagsItemKind::Flag(Flag::DotMatchesNewLine);
    assert!(!item.is_negation());
}

#[test]
fn test_is_negation_with_flag_swap_greed() {
    let item = FlagsItemKind::Flag(Flag::SwapGreed);
    assert!(!item.is_negation());
}

#[test]
fn test_is_negation_with_flag_unicode() {
    let item = FlagsItemKind::Flag(Flag::Unicode);
    assert!(!item.is_negation());
}

#[test]
fn test_is_negation_with_flag_ignore_whitespace() {
    let item = FlagsItemKind::Flag(Flag::IgnoreWhitespace);
    assert!(!item.is_negation());
}

