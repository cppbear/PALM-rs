// Answer 0

#[test]
fn test_fmt_assertion_start_line() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let assertion = ast::Assertion {
        span: Span { /* span details */ },
        kind: ast::AssertionKind::StartLine,
    };
    writer.fmt_assertion(&assertion).unwrap();
    assert_eq!(output, "^");
}

#[test]
fn test_fmt_assertion_end_line() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let assertion = ast::Assertion {
        span: Span { /* span details */ },
        kind: ast::AssertionKind::EndLine,
    };
    writer.fmt_assertion(&assertion).unwrap();
    assert_eq!(output, "$");
}

#[test]
fn test_fmt_assertion_start_text() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let assertion = ast::Assertion {
        span: Span { /* span details */ },
        kind: ast::AssertionKind::StartText,
    };
    writer.fmt_assertion(&assertion).unwrap();
    assert_eq!(output, r"\A");
}

#[test]
fn test_fmt_assertion_end_text() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let assertion = ast::Assertion {
        span: Span { /* span details */ },
        kind: ast::AssertionKind::EndText,
    };
    writer.fmt_assertion(&assertion).unwrap();
    assert_eq!(output, r"\z");
}

#[test]
fn test_fmt_assertion_word_boundary() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let assertion = ast::Assertion {
        span: Span { /* span details */ },
        kind: ast::AssertionKind::WordBoundary,
    };
    writer.fmt_assertion(&assertion).unwrap();
    assert_eq!(output, r"\b");
}

#[test]
fn test_fmt_assertion_not_word_boundary() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let assertion = ast::Assertion {
        span: Span { /* span details */ },
        kind: ast::AssertionKind::NotWordBoundary,
    };
    writer.fmt_assertion(&assertion).unwrap();
    assert_eq!(output, r"\B");
}

