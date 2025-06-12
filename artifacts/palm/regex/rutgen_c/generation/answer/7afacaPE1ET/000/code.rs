// Answer 0

#[test]
fn test_flag_state_case_insensitive() {
    let span = Span { start: Position(0), end: Position(1) };
    let item = FlagsItem { span, kind: FlagsItemKind::Flag(Flag::CaseInsensitive) };
    let mut flags = Flags { span, items: vec![item] };
    assert_eq!(flags.flag_state(Flag::CaseInsensitive), Some(true));
}

#[test]
fn test_flag_state_case_insensitive_negated() {
    let span = Span { start: Position(0), end: Position(1) };
    let negation_item = FlagsItem { span, kind: FlagsItemKind::Negation };
    let flag_item = FlagsItem { span: span, kind: FlagsItemKind::Flag(Flag::CaseInsensitive) };
    let mut flags = Flags { span, items: vec![negation_item, flag_item] };
    assert_eq!(flags.flag_state(Flag::CaseInsensitive), Some(false));
}

#[test]
fn test_flag_state_unicode() {
    let span = Span { start: Position(0), end: Position(1) };
    let item = FlagsItem { span, kind: FlagsItemKind::Flag(Flag::Unicode) };
    let mut flags = Flags { span, items: vec![item] };
    assert_eq!(flags.flag_state(Flag::Unicode), Some(true));
}

#[test]
fn test_flag_state_not_found() {
    let span = Span { start: Position(0), end: Position(1) };
    let item = FlagsItem { span, kind: FlagsItemKind::Flag(Flag::MultiLine) };
    let mut flags = Flags { span, items: vec![item] };
    assert_eq!(flags.flag_state(Flag::CaseInsensitive), None);
}

#[test]
fn test_flag_state_multiple_flags_and_negations() {
    let span = Span { start: Position(0), end: Position(1) };
    let negation_item = FlagsItem { span, kind: FlagsItemKind::Negation };
    let flag_item1 = FlagsItem { span, kind: FlagsItemKind::Flag(Flag::CaseInsensitive) };
    let flag_item2 = FlagsItem { span, kind: FlagsItemKind::Flag(Flag::Unicode) };
    let mut flags = Flags { span, items: vec![negation_item, flag_item1, flag_item2] };
    assert_eq!(flags.flag_state(Flag::Unicode), Some(false));
}

