// Answer 0

#[test]
fn test_fmt_flags_case_insensitive() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let flags = Flags {
        span: Span::new(0, 1),
        items: vec![
            FlagsItem {
                span: Span::new(0, 1),
                kind: FlagsItemKind::Flag(Flag::CaseInsensitive),
            },
        ],
    };
    writer.fmt_flags(&flags).unwrap();
}

#[test]
fn test_fmt_flags_multi_line() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let flags = Flags {
        span: Span::new(0, 1),
        items: vec![
            FlagsItem {
                span: Span::new(0, 1),
                kind: FlagsItemKind::Flag(Flag::MultiLine),
            },
        ],
    };
    writer.fmt_flags(&flags).unwrap();
}

#[test]
fn test_fmt_flags_dot_matches_new_line() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let flags = Flags {
        span: Span::new(0, 1),
        items: vec![
            FlagsItem {
                span: Span::new(0, 1),
                kind: FlagsItemKind::Flag(Flag::DotMatchesNewLine),
            },
        ],
    };
    writer.fmt_flags(&flags).unwrap();
}

#[test]
fn test_fmt_flags_swap_greed() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let flags = Flags {
        span: Span::new(0, 1),
        items: vec![
            FlagsItem {
                span: Span::new(0, 1),
                kind: FlagsItemKind::Flag(Flag::SwapGreed),
            },
        ],
    };
    writer.fmt_flags(&flags).unwrap();
}

#[test]
fn test_fmt_flags_unicode() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let flags = Flags {
        span: Span::new(0, 1),
        items: vec![
            FlagsItem {
                span: Span::new(0, 1),
                kind: FlagsItemKind::Flag(Flag::Unicode),
            },
        ],
    };
    writer.fmt_flags(&flags).unwrap();
}

#[test]
fn test_fmt_flags_ignore_whitespace() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let flags = Flags {
        span: Span::new(0, 1),
        items: vec![
            FlagsItem {
                span: Span::new(0, 1),
                kind: FlagsItemKind::Flag(Flag::IgnoreWhitespace),
            },
        ],
    };
    writer.fmt_flags(&flags).unwrap();
}

#[test]
fn test_fmt_flags_combined() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let flags = Flags {
        span: Span::new(0, 1),
        items: vec![
            FlagsItem {
                span: Span::new(0, 1),
                kind: FlagsItemKind::Flag(Flag::CaseInsensitive),
            },
            FlagsItem {
                span: Span::new(1, 1),
                kind: FlagsItemKind::Flag(Flag::Unicode),
            },
        ],
    };
    writer.fmt_flags(&flags).unwrap();
}

