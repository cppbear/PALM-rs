// Answer 0

#[test]
fn test_fmt_flags_single_flag_multiline() {
    let mut output = String::new();
    let flags = ast::Flags {
        span: Span::default(),
        items: vec![ast::FlagsItem {
            span: Span::default(),
            kind: ast::FlagsItemKind::Flag(Box::new(ast::Flag::MultiLine)),
        }],
    };
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let _ = writer.fmt_flags(&flags);
}

#[test]
fn test_fmt_flags_multiple_flags() {
    let mut output = String::new();
    let flags = ast::Flags {
        span: Span::default(),
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(Box::new(ast::Flag::MultiLine)),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(Box::new(ast::Flag::CaseInsensitive)),
            },
        ],
    };
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let _ = writer.fmt_flags(&flags);
}

#[test]
fn test_fmt_flags_negations_and_flags() {
    let mut output = String::new();
    let flags = ast::Flags {
        span: Span::default(),
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Negation,
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(Box::new(ast::Flag::MultiLine)),
            },
        ],
    };
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let _ = writer.fmt_flags(&flags);
}

#[test]
fn test_fmt_flags_complex_combination() {
    let mut output = String::new();
    let flags = ast::Flags {
        span: Span::default(),
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(Box::new(ast::Flag::CaseInsensitive)),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Negation,
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(Box::new(ast::Flag::MultiLine)),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(Box::new(ast::Flag::DotMatchesNewLine)),
            },
        ],
    };
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let _ = writer.fmt_flags(&flags);
}

