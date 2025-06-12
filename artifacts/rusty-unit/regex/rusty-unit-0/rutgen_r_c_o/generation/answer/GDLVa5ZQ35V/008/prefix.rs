// Answer 0

#[test]
fn test_visit_post_assertion_start_line() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let assertion = Assertion { span: Span { start: 0, end: 10 }, kind: AssertionKind::StartLine };
    let ast = Ast::Assertion(assertion);
    writer.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_assertion_end_line() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let assertion = Assertion { span: Span { start: 20, end: 30 }, kind: AssertionKind::EndLine };
    let ast = Ast::Assertion(assertion);
    writer.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_assertion_start_text() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let assertion = Assertion { span: Span { start: 40, end: 50 }, kind: AssertionKind::StartText };
    let ast = Ast::Assertion(assertion);
    writer.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_assertion_end_text() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let assertion = Assertion { span: Span { start: 60, end: 70 }, kind: AssertionKind::EndText };
    let ast = Ast::Assertion(assertion);
    writer.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_assertion_word_boundary() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let assertion = Assertion { span: Span { start: 80, end: 90 }, kind: AssertionKind::WordBoundary };
    let ast = Ast::Assertion(assertion);
    writer.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_assertion_not_word_boundary() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let assertion = Assertion { span: Span { start: 90, end: 100 }, kind: AssertionKind::NotWordBoundary };
    let ast = Ast::Assertion(assertion);
    writer.visit_post(&ast).unwrap();
}

