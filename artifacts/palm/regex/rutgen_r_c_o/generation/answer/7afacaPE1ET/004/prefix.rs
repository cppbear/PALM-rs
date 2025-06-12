// Answer 0

#[derive(Clone, Debug, Eq, PartialEq)]
struct Position(usize);

#[test]
fn test_flag_state_case_insensitive_with_negation() {
    let start = Position(0);
    let end = Position(10);
    let span = Span { start, end };
    
    let flags_item_negation = FlagsItem {
        span: span.clone(),
        kind: FlagsItemKind::Negation,
    };
    
    let flags_item_case_insensitive = FlagsItem {
        span,
        kind: FlagsItemKind::Flag(Flag::CaseInsensitive),
    };
    
    let mut flags = Flags {
        span,
        items: vec![flags_item_negation, flags_item_case_insensitive],
    };
    
    flags.flag_state(Flag::CaseInsensitive);
}

#[test]
fn test_flag_state_dot_matches_new_line_with_negation() {
    let start = Position(5);
    let end = Position(15);
    let span = Span { start, end };
    
    let flags_item_negation = FlagsItem {
        span: span.clone(),
        kind: FlagsItemKind::Negation,
    };
    
    let flags_item_dot_matches_new_line = FlagsItem {
        span,
        kind: FlagsItemKind::Flag(Flag::DotMatchesNewLine),
    };

    let mut flags = Flags {
        span,
        items: vec![flags_item_negation, flags_item_dot_matches_new_line],
    };
    
    flags.flag_state(Flag::DotMatchesNewLine);
}

#[test]
fn test_flag_state_case_insensitive_without_negation() {
    let start = Position(1);
    let end = Position(3);
    let span = Span { start, end };

    let flags_item_case_insensitive = FlagsItem {
        span,
        kind: FlagsItemKind::Flag(Flag::CaseInsensitive),
    };

    let mut flags = Flags {
        span,
        items: vec![flags_item_case_insensitive],
    };
    
    flags.flag_state(Flag::CaseInsensitive);
}

#[test]
fn test_flag_state_empty_flags() {
    let start = Position(0);
    let end = Position(0);
    let span = Span { start, end };

    let flags = Flags {
        span,
        items: vec![],
    };
    
    flags.flag_state(Flag::Unicode);
}

