// Answer 0

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Position(usize);

#[test]
fn test_flag_state_case_insensitive() {
    let span = Span { start: Position(0), end: Position(5) };
    let case_insensitive_item = FlagsItem {
        span,
        kind: FlagsItemKind::Flag(Flag::CaseInsensitive),
    };
    let mut flags = Flags {
        span,
        items: vec![case_insensitive_item],
    };

    assert_eq!(flags.flag_state(Flag::CaseInsensitive), Some(true));
}

#[test]
fn test_flag_state_case_insensitive_negated() {
    let span = Span { start: Position(0), end: Position(5) };
    let negation_item = FlagsItem {
        span,
        kind: FlagsItemKind::Negation,
    };
    let case_insensitive_item = FlagsItem {
        span,
        kind: FlagsItemKind::Flag(Flag::CaseInsensitive),
    };
    let mut flags = Flags {
        span,
        items: vec![negation_item, case_insensitive_item],
    };

    assert_eq!(flags.flag_state(Flag::CaseInsensitive), Some(false));
}

#[test]
fn test_flag_state_multi_line() {
    let span = Span { start: Position(6), end: Position(11) };
    let multi_line_item = FlagsItem {
        span,
        kind: FlagsItemKind::Flag(Flag::MultiLine),
    };
    let mut flags = Flags {
        span,
        items: vec![multi_line_item],
    };

    assert_eq!(flags.flag_state(Flag::MultiLine), Some(true));
}

#[test]
fn test_flag_state_multi_line_negated() {
    let span = Span { start: Position(6), end: Position(11) };
    let negation_item = FlagsItem {
        span,
        kind: FlagsItemKind::Negation,
    };
    let multi_line_item = FlagsItem {
        span,
        kind: FlagsItemKind::Flag(Flag::MultiLine),
    };
    let mut flags = Flags {
        span,
        items: vec![negation_item, multi_line_item],
    };

    assert_eq!(flags.flag_state(Flag::MultiLine), Some(false));
}

#[test]
fn test_flag_state_flags_not_present() {
    let span = Span { start: Position(0), end: Position(5) };
    let mut flags = Flags {
        span,
        items: vec![],
    };

    assert_eq!(flags.flag_state(Flag::CaseInsensitive), None);
}

#[test]
fn test_flag_state_negation_without_flag() {
    let span = Span { start: Position(0), end: Position(5) };
    let negation_item = FlagsItem {
        span,
        kind: FlagsItemKind::Negation,
    };
    let mut flags = Flags {
        span,
        items: vec![negation_item],
    };

    assert_eq!(flags.flag_state(Flag::CaseInsensitive), None);
}

