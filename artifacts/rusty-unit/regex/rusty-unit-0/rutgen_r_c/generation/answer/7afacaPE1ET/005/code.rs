// Answer 0

#[test]
fn test_flag_state_none_no_items() {
    let flags = Flags {
        span: Span {
            start: Position(0),
            end: Position(0),
        },
        items: Vec::new(),
    };
    
    assert_eq!(flags.flag_state(Flag::CaseInsensitive), None);
}

#[test]
fn test_flag_state_none_negation_only() {
    let flags = Flags {
        span: Span {
            start: Position(0),
            end: Position(0),
        },
        items: vec![
            FlagsItem {
                span: Span {
                    start: Position(0),
                    end: Position(1),
                },
                kind: FlagsItemKind::Negation,
            },
        ],
    };
    
    assert_eq!(flags.flag_state(Flag::CaseInsensitive), None);
}

#[test]
fn test_flag_state_none_negative_and_positive() {
    let flags = Flags {
        span: Span {
            start: Position(0),
            end: Position(0),
        },
        items: vec![
            FlagsItem {
                span: Span {
                    start: Position(0),
                    end: Position(1),
                },
                kind: FlagsItemKind::Negation,
            },
            FlagsItem {
                span: Span {
                    start: Position(1),
                    end: Position(2),
                },
                kind: FlagsItemKind::Flag(Flag::CaseInsensitive),
            },
        ],
    };
    
    assert_eq!(flags.flag_state(Flag::CaseInsensitive), Some(false));
}

#[test]
fn test_flag_state_none_negative_flag_other_than_target() {
    let flags = Flags {
        span: Span {
            start: Position(0),
            end: Position(0),
        },
        items: vec![
            FlagsItem {
                span: Span {
                    start: Position(0),
                    end: Position(1),
                },
                kind: FlagsItemKind::Negation,
            },
            FlagsItem {
                span: Span {
                    start: Position(1),
                    end: Position(2),
                },
                kind: FlagsItemKind::Flag(Flag::MultiLine),
            },
        ],
    };
    
    assert_eq!(flags.flag_state(Flag::CaseInsensitive), None);
}

