// Answer 0

#[test]
fn test_is_negation_case_insensitive() {
    let item = FlagsItemKind::Flag(Flag::CaseInsensitive);
    item.is_negation();
}

#[test]
fn test_is_negation_multi_line() {
    let item = FlagsItemKind::Flag(Flag::MultiLine);
    item.is_negation();
}

#[test]
fn test_is_negation_dot_matches_new_line() {
    let item = FlagsItemKind::Flag(Flag::DotMatchesNewLine);
    item.is_negation();
}

#[test]
fn test_is_negation_swap_greed() {
    let item = FlagsItemKind::Flag(Flag::SwapGreed);
    item.is_negation();
}

#[test]
fn test_is_negation_unicode() {
    let item = FlagsItemKind::Flag(Flag::Unicode);
    item.is_negation();
}

#[test]
fn test_is_negation_ignore_whitespace() {
    let item = FlagsItemKind::Flag(Flag::IgnoreWhitespace);
    item.is_negation();
}

