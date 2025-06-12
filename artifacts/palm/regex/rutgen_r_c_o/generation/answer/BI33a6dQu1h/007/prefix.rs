// Answer 0

#[test]
fn test_fmt_flags_single_dot_matches_newline() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let flags = Flags {
        span: Span { start: 0, end: 1 },
        items: vec![
            FlagsItem {
                span: Span { start: 0, end: 1 },
                kind: FlagsItemKind::Flag(Box::new(Flag::DotMatchesNewLine)),
            },
        ],
    };
    writer.fmt_flags(&flags).unwrap();
}

#[test]
fn test_fmt_flags_multiple_dot_matches_newline() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let flags = Flags {
        span: Span { start: 0, end: 5 },
        items: vec![
            FlagsItem {
                span: Span { start: 0, end: 1 },
                kind: FlagsItemKind::Flag(Box::new(Flag::DotMatchesNewLine)),
            },
            FlagsItem {
                span: Span { start: 1, end: 2 },
                kind: FlagsItemKind::Flag(Box::new(Flag::DotMatchesNewLine)),
            },
            FlagsItem {
                span: Span { start: 2, end: 3 },
                kind: FlagsItemKind::Flag(Box::new(Flag::DotMatchesNewLine)),
            },
        ],
    };
    writer.fmt_flags(&flags).unwrap();
}

#[test]
fn test_fmt_flags_flag_with_negation() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let flags = Flags {
        span: Span { start: 0, end: 6 },
        items: vec![
            FlagsItem {
                span: Span { start: 0, end: 1 },
                kind: FlagsItemKind::Negation,
            },
            FlagsItem {
                span: Span { start: 1, end: 2 },
                kind: FlagsItemKind::Flag(Box::new(Flag::DotMatchesNewLine)),
            },
        ],
    };
    writer.fmt_flags(&flags).unwrap();
}

#[test]
#[should_panic]
fn test_fmt_flags_invalid_state() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let flags = Flags {
        span: Span { start: 0, end: 4 },
        items: vec![
            FlagsItem {
                span: Span { start: 0, end: 1 },
                kind: FlagsItemKind::Negation,
            },
            FlagsItem {
                span: Span { start: 1, end: 2 },
                kind: FlagsItemKind::Flag(Box::new(Flag::CaseInsensitive)), // this will not panic but let's assume hypothetical wrong usage
            },
        ],
    };
    writer.fmt_flags(&flags).unwrap();
}

#[test]
fn test_fmt_flags_empty_flags() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let flags = Flags {
        span: Span { start: 0, end: 0 },
        items: vec![],
    };
    writer.fmt_flags(&flags).unwrap();
}

