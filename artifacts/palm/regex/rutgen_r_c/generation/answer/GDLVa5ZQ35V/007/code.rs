// Answer 0

#[test]
fn test_visit_post_empty() {
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let ast = Ast::Empty(Span::new(0, 0));
    let result = writer.visit_post(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_visit_post_unicode_class() {
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let unicode_class = ClassUnicode {
        span: Span::new(0, 1),
        negated: false,
        kind: ClassUnicodeKind::OneLetter('a'),
    };
    let ast = Ast::Class(Class::Unicode(unicode_class));
    let result = writer.visit_post(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.wtr, r"\pa");
}

#[test]
fn test_visit_post_bracketed_class() {
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let bracketed_class = ClassBracketed {
        span: Span::new(0, 3),
        negated: false,
        kind: ClassSet::Normal,
    };
    let ast = Ast::Class(Class::Bracketed(bracketed_class));
    let result = writer.visit_post(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.wtr, "]");
}

#[test]
fn test_visit_post_perl_class() {
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let perl_class = ClassPerl {
        span: Span::new(0, 1),
        kind: ClassPerlKind::Digit,
        negated: false,
    };
    let ast = Ast::Class(Class::Perl(perl_class));
    let result = writer.visit_post(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.wtr, r"\d");
}

