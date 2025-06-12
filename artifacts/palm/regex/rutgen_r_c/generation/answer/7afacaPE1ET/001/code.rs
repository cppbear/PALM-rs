// Answer 0

#[test]
fn test_flag_state_none_empty_items() {
    let flags = Flags {
        span: Span { start: Position(0), end: Position(1) },
        items: Vec::new(),
    };
    assert_eq!(flags.flag_state(Flag::CaseInsensitive), None);
}

#[test]
fn test_flag_state_none_no_flags() {
    let flags = Flags {
        span: Span { start: Position(0), end: Position(1) },
        items: vec![
            FlagsItem {
                span: Span { start: Position(0), end: Position(1) },
                kind: FlagsItemKind::Negation,
            },
        ],
    };
    assert_eq!(flags.flag_state(Flag::CaseInsensitive), None);
}

#[test]
fn test_flag_state_none_negation_with_other_flags() {
    let flags = Flags {
        span: Span { start: Position(0), end: Position(1) },
        items: vec![
            FlagsItem {
                span: Span { start: Position(0), end: Position(1) },
                kind: FlagsItemKind::Negation,
            },
            FlagsItem {
                span: Span { start: Position(1), end: Position(2) },
                kind: FlagsItemKind::Flag(Flag::MultiLine),
            },
        ],
    };
    assert_eq!(flags.flag_state(Flag::CaseInsensitive), None);
}

