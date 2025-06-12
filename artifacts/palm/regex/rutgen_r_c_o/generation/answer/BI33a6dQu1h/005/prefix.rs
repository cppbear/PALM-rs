// Answer 0

#[test]
fn test_fmt_flags_single_swap_greed() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let ast = ast::Flags {
        span: Span::default(),
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(Box::new(ast::Flag::SwapGreed)),
            },
        ],
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let _ = writer.fmt_flags(&ast);
}

#[test]
fn test_fmt_flags_multiple_swap_greed() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let ast = ast::Flags {
        span: Span::default(),
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(Box::new(ast::Flag::SwapGreed)),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(Box::new(ast::Flag::SwapGreed)),
            },
        ],
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let _ = writer.fmt_flags(&ast);
}

#[test]
fn test_fmt_flags_no_items() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let ast = ast::Flags {
        span: Span::default(),
        items: vec![],
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let _ = writer.fmt_flags(&ast);
}

#[test]
fn test_fmt_flags_multiple_flags() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let ast = ast::Flags {
        span: Span::default(),
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(Box::new(ast::Flag::SwapGreed)),
            },
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(Box::new(ast::Flag::Unicode)),
            },
        ],
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let _ = writer.fmt_flags(&ast);
}

#[test]
#[should_panic]
fn test_fmt_flags_invalid_flag() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let ast = ast::Flags {
        span: Span::default(),
        items: vec![
            ast::FlagsItem {
                span: Span::default(),
                kind: ast::FlagsItemKind::Flag(Box::new(ast::Flag::IgnoreWhitespace)),
            },
        ],
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let _ = writer.fmt_flags(&ast);
}

