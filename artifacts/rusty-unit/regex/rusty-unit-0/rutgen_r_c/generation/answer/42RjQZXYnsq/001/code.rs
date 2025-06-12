// Answer 0

#[test]
fn test_is_negation_not_negation() {
    let flag_item = FlagsItemKind::Flag(Flag::CaseInsensitive);
    assert_eq!(flag_item.is_negation(), false);
}

#[test]
fn test_is_negation_not_negation_multi_line() {
    let flag_item = FlagsItemKind::Flag(Flag::MultiLine);
    assert_eq!(flag_item.is_negation(), false);
}

#[test]
fn test_is_negation_not_negation_dot_matches_new_line() {
    let flag_item = FlagsItemKind::Flag(Flag::DotMatchesNewLine);
    assert_eq!(flag_item.is_negation(), false);
}

#[test]
fn test_is_negation_not_negation_swap_greed() {
    let flag_item = FlagsItemKind::Flag(Flag::SwapGreed);
    assert_eq!(flag_item.is_negation(), false);
}

#[test]
fn test_is_negation_not_negation_unicode() {
    let flag_item = FlagsItemKind::Flag(Flag::Unicode);
    assert_eq!(flag_item.is_negation(), false);
}

#[test]
fn test_is_negation_not_negation_ignore_whitespace() {
    let flag_item = FlagsItemKind::Flag(Flag::IgnoreWhitespace);
    assert_eq!(flag_item.is_negation(), false);
}

