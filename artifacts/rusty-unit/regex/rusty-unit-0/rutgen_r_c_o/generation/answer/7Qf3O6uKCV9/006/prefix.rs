// Answer 0

#[test]
fn test_fmt_assertion_start_line() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::StartLine,
    };
    let _ = writer.fmt_assertion(&ast);
}

#[test]
fn test_fmt_assertion_end_line() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::EndLine,
    };
    let _ = writer.fmt_assertion(&ast);
}

#[test]
fn test_fmt_assertion_start_text() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::StartText,
    };
    let _ = writer.fmt_assertion(&ast);
}

#[test]
fn test_fmt_assertion_end_text() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::EndText,
    };
    let _ = writer.fmt_assertion(&ast);
}

#[test]
fn test_fmt_assertion_word_boundary() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundary,
    };
    let _ = writer.fmt_assertion(&ast);
}

#[test]
fn test_fmt_assertion_not_word_boundary() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::NotWordBoundary,
    };
    let _ = writer.fmt_assertion(&ast);
}

