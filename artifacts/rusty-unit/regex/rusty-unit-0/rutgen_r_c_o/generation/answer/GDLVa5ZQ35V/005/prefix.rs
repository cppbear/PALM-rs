// Answer 0

#[test]
fn test_visit_post_empty() {
    let span = Span::new(0, 0);
    let ast = Ast::Empty(span);
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_literal() {
    let span = Span::new(0, 1);
    let ast = Ast::Literal(Literal { span, kind: LiteralKind::Verbatim, c: 'a' });
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_unicode_class_negated() {
    let span = Span::new(0, 1);
    let ast = Ast::Class(Class::Unicode(ClassUnicode { span, negated: true, kind: ClassUnicodeKind::OneLetter('a') }));
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_unicode_class_non_negated() {
    let span = Span::new(0, 1);
    let ast = Ast::Class(Class::Unicode(ClassUnicode { span, negated: false, kind: ClassUnicodeKind::Named('L') }));
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_class_bracketed() {
    let span = Span::new(0, 2);
    let ast = Ast::Class(Class::Bracketed(ClassBracketed { span, negated: true, kind: ClassSet::Normal }));
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_class_perl_digit_negated() {
    let span = Span::new(0, 1);
    let ast = Ast::Class(Class::Perl(ClassPerl { span, kind: ClassPerlKind::Digit, negated: true }));
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_class_perl_space_non_negated() {
    let span = Span::new(0, 1);
    let ast = Ast::Class(Class::Perl(ClassPerl { span, kind: ClassPerlKind::Space, negated: false }));
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_assertion_start_line() {
    let span = Span::new(0, 0);
    let ast = Ast::Assertion(Assertion { span, kind: AssertionKind::StartLine });
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    writer.visit_post(&ast);
}

