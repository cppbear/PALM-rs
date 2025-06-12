// Answer 0

#[test]
fn test_fmt_flags_single_negation() {
    let mut buf = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut buf };
    let flags = Flags {
        span: Span::new(0, 1),
        items: vec![FlagsItem {
            span: Span::new(0, 1),
            kind: FlagsItemKind::Negation,
        }],
    };
    let _ = writer.fmt_flags(&flags);
}

#[test]
fn test_fmt_flags_single_case_insensitive_flag() {
    let mut buf = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut buf };
    let flags = Flags {
        span: Span::new(0, 1),
        items: vec![FlagsItem {
            span: Span::new(0, 1),
            kind: FlagsItemKind::Flag(Flag::CaseInsensitive),
        }],
    };
    let _ = writer.fmt_flags(&flags);
}

#[test]
fn test_fmt_flags_multiple_flags() {
    let mut buf = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut buf };
    let flags = Flags {
        span: Span::new(0, 6),
        items: vec![
            FlagsItem {
                span: Span::new(0, 1),
                kind: FlagsItemKind::Flag(Flag::CaseInsensitive),
            },
            FlagsItem {
                span: Span::new(1, 1),
                kind: FlagsItemKind::Flag(Flag::MultiLine),
            },
        ],
    };
    let _ = writer.fmt_flags(&flags);
}

#[test]
fn test_fmt_flags_combined_negation_and_flags() {
    let mut buf = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut buf };
    let flags = Flags {
        span: Span::new(0, 5),
        items: vec![
            FlagsItem {
                span: Span::new(0, 1),
                kind: FlagsItemKind::Negation,
            },
            FlagsItem {
                span: Span::new(1, 1),
                kind: FlagsItemKind::Flag(Flag::DotMatchesNewLine),
            },
        ],
    };
    let _ = writer.fmt_flags(&flags);
}

#[test]
fn test_fmt_flags_all_flags() {
    let mut buf = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut buf };
    let flags = Flags {
        span: Span::new(0, 6),
        items: vec![
            FlagsItem {
                span: Span::new(0, 1),
                kind: FlagsItemKind::Flag(Flag::CaseInsensitive),
            },
            FlagsItem {
                span: Span::new(1, 1),
                kind: FlagsItemKind::Flag(Flag::MultiLine),
            },
            FlagsItem {
                span: Span::new(2, 1),
                kind: FlagsItemKind::Flag(Flag::DotMatchesNewLine),
            },
            FlagsItem {
                span: Span::new(3, 1),
                kind: FlagsItemKind::Flag(Flag::SwapGreed),
            },
            FlagsItem {
                span: Span::new(4, 1),
                kind: FlagsItemKind::Flag(Flag::Unicode),
            },
            FlagsItem {
                span: Span::new(5, 1),
                kind: FlagsItemKind::Flag(Flag::IgnoreWhitespace),
            },
        ],
    };
    let _ = writer.fmt_flags(&flags);
}

