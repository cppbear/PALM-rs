// Answer 0

#[test]
fn test_flag_state_not_found() {
    let start_position = Position(0);
    let end_position = Position(10);
    let span = Span { start: start_position, end: end_position };
    
    let item1 = FlagsItem {
        span: span.clone(),
        kind: FlagsItemKind::Flag(Flag::CaseInsensitive),
    };
    let item2 = FlagsItem {
        span: span.clone(),
        kind: FlagsItemKind::Flag(Flag::MultiLine),
    };
    let negation_item = FlagsItem {
        span: span.clone(),
        kind: FlagsItemKind::Negation,
    };
    
    let mut flags = Flags {
        span,
        items: vec![item1, negation_item, item2],
    };

    // Check for a flag that is present but will be negated
    assert_eq!(flags.flag_state(Flag::DotMatchesNewLine), None);
}

#[test]
fn test_flag_state_not_negated() {
    let start_position = Position(0);
    let end_position = Position(10);
    let span = Span { start: start_position, end: end_position };
    
    let item = FlagsItem {
        span: span.clone(),
        kind: FlagsItemKind::Flag(Flag::CaseInsensitive),
    };
    let item2 = FlagsItem {
        span: span.clone(),
        kind: FlagsItemKind::Flag(Flag::MultiLine),
    };
    
    let mut flags = Flags {
        span,
        items: vec![item, item2],
    };

    // Check for a flag that does not exist
    assert_eq!(flags.flag_state(Flag::DotMatchesNewLine), None);
}

#[test]
fn test_flag_state_negated() {
    let start_position = Position(0);
    let end_position = Position(10);
    let span = Span { start: start_position, end: end_position };
    
    let negation_item = FlagsItem {
        span: span.clone(),
        kind: FlagsItemKind::Negation,
    };
    let item = FlagsItem {
        span: span.clone(),
        kind: FlagsItemKind::Flag(Flag::CaseInsensitive),
    };
    
    let mut flags = Flags {
        span,
        items: vec![negation_item, item],
    };

    // Check for a flag that exists but is negated
    assert_eq!(flags.flag_state(Flag::CaseInsensitive), Some(false));
}

