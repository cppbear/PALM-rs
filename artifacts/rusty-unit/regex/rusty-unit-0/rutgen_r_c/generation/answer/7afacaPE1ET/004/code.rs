// Answer 0

#[test]
fn test_flag_state_flag_not_present() {
    let span = Span { start: Position(0), end: Position(10) };
    let flags = Flags {
        span,
        items: vec![],
    };
    
    let result = flags.flag_state(Flag::CaseInsensitive);
    assert_eq!(result, None);
}

#[test]
fn test_flag_state_flag_negated() {
    let span1 = Span { start: Position(0), end: Position(10) };
    let span2 = Span { start: Position(11), end: Position(15) };
    let item1 = FlagsItem {
        span: span1,
        kind: FlagsItemKind::Negation,
    };
    let item2 = FlagsItem {
        span: span2,
        kind: FlagsItemKind::Flag(Flag::CaseInsensitive),
    };
    let flags = Flags {
        span: span1, 
        items: vec![item1, item2],
    };
    
    let result = flags.flag_state(Flag::CaseInsensitive);
    assert_eq!(result, Some(false));
}

#[test]
fn test_flag_state_flag_present_not_negated() {
    let span1 = Span { start: Position(0), end: Position(10) };
    let span2 = Span { start: Position(11), end: Position(15) };
    let item1 = FlagsItem {
        span: span1,
        kind: FlagsItemKind::Flag(Flag::CaseInsensitive),
    };
    let item2 = FlagsItem {
        span: span2,
        kind: FlagsItemKind::Negation,
    };
    let flags = Flags {
        span: span1,
        items: vec![item1, item2],
    };
    
    let result = flags.flag_state(Flag::CaseInsensitive);
    assert_eq!(result, Some(true));
}

