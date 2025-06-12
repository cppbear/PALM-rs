// Answer 0

#[test]
fn test_is_negation_true() {
    let negation_flag = FlagsItemKind::Negation;
    assert!(negation_flag.is_negation());
}

#[test]
fn test_is_negation_false_flag_case_insensitive() {
    let case_insensitive_flag = FlagsItemKind::Flag(Flag::CaseInsensitive);
    assert!(!case_insensitive_flag.is_negation());
}

#[test]
fn test_is_negation_false_flag_multi_line() {
    let multi_line_flag = FlagsItemKind::Flag(Flag::MultiLine);
    assert!(!multi_line_flag.is_negation());
}

#[test]
fn test_is_negation_false_flag_dot_matches_new_line() {
    let dot_matches_new_line_flag = FlagsItemKind::Flag(Flag::DotMatchesNewLine);
    assert!(!dot_matches_new_line_flag.is_negation());
}

#[test]
fn test_is_negation_false_flag_swap_greed() {
    let swap_greed_flag = FlagsItemKind::Flag(Flag::SwapGreed);
    assert!(!swap_greed_flag.is_negation());
}

#[test]
fn test_is_negation_false_flag_unicode() {
    let unicode_flag = FlagsItemKind::Flag(Flag::Unicode);
    assert!(!unicode_flag.is_negation());
}

#[test]
fn test_is_negation_false_flag_ignore_whitespace() {
    let ignore_whitespace_flag = FlagsItemKind::Flag(Flag::IgnoreWhitespace);
    assert!(!ignore_whitespace_flag.is_negation());
}

