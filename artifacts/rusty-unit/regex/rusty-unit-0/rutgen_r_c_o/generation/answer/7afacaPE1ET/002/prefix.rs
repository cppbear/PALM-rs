// Answer 0

#[test]
fn test_flag_state_case_insensitive() {
    let span = Span { start: Position(0), end: Position(10) };
    let items = vec![
        FlagsItem { span, kind: FlagsItemKind::Flag(Flag::CaseInsensitive) },
    ];
    let flags = Flags { span, items };
    flags.flag_state(Flag::CaseInsensitive);
}

#[test]
fn test_flag_state_multiline() {
    let span = Span { start: Position(0), end: Position(15) };
    let items = vec![
        FlagsItem { span, kind: FlagsItemKind::Flag(Flag::MultiLine) },
    ];
    let flags = Flags { span, items };
    flags.flag_state(Flag::MultiLine);
}

#[test]
fn test_flag_state_dot_matches_new_line() {
    let span = Span { start: Position(0), end: Position(20) };
    let items = vec![
        FlagsItem { span, kind: FlagsItemKind::Flag(Flag::DotMatchesNewLine) },
    ];
    let flags = Flags { span, items };
    flags.flag_state(Flag::DotMatchesNewLine);
}

#[test]
fn test_flag_state_swap_greed() {
    let span = Span { start: Position(5), end: Position(20) };
    let items = vec![
        FlagsItem { span, kind: FlagsItemKind::Flag(Flag::SwapGreed) },
    ];
    let flags = Flags { span, items };
    flags.flag_state(Flag::SwapGreed);
}

#[test]
fn test_flag_state_unicode() {
    let span = Span { start: Position(0), end: Position(8) };
    let items = vec![
        FlagsItem { span, kind: FlagsItemKind::Flag(Flag::Unicode) },
    ];
    let flags = Flags { span, items };
    flags.flag_state(Flag::Unicode);
}

#[test]
fn test_flag_state_ignore_whitespace() {
    let span = Span { start: Position(3), end: Position(12) };
    let items = vec![
        FlagsItem { span, kind: FlagsItemKind::Flag(Flag::IgnoreWhitespace) },
    ];
    let flags = Flags { span, items };
    flags.flag_state(Flag::IgnoreWhitespace);
}

#[test]
fn test_flag_state_negation_effect() {
    let span = Span { start: Position(1), end: Position(10) };
    let items = vec![
        FlagsItem { span: Span { start: Position(1), end: Position(2) }, kind: FlagsItemKind::Negation },
        FlagsItem { span, kind: FlagsItemKind::Flag(Flag::CaseInsensitive) },
    ];
    let flags = Flags { span, items };
    flags.flag_state(Flag::CaseInsensitive);
}

#[test]
fn test_flag_state_multiple_items() {
    let span1 = Span { start: Position(0), end: Position(5) };
    let span2 = Span { start: Position(5), end: Position(10) };
    let items = vec![
        FlagsItem { span: span1, kind: FlagsItemKind::Negation },
        FlagsItem { span: span2, kind: FlagsItemKind::Flag(Flag::MultiLine) },
    ];
    let flags = Flags { span: Span { start: Position(0), end: Position(10) }, items };
    flags.flag_state(Flag::MultiLine);
}

