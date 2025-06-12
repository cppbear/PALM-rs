// Answer 0

#[test]
fn test_visit_post_empty() {
    let translator = Translator::default();
    let mut visitor = TranslatorI::new(&translator, "");
    let ast = Ast::Empty(Span { start: 0, end: 0 });
    visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_literal() {
    let translator = Translator::default();
    let mut visitor = TranslatorI::new(&translator, "a");
    let literal = Literal {
        span: Span { start: 0, end: 1 },
        kind: LiteralKind::Unicode,
        c: 'a',
    };
    let ast = Ast::Literal(literal);
    visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_class_unicode() {
    let translator = Translator::default();
    let mut visitor = TranslatorI::new(&translator, "abc");
    let unicode_class = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'z' }]);
    let ast = Ast::Class(Class::Unicode(unicode_class));
    visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_class_bracketed() {
    let translator = Translator::default();
    let mut visitor = TranslatorI::new(&translator, "abc");
    let mut class_bytes = ClassBytes::new(vec![ClassBytesRange { start: 0, end: 255 }]);
    class_bytes.push(ClassBytesRange { start: 1, end: 1 });
    let ast = Ast::Class(Class::Bracketed(ast::ClassBracketed {
        span: Span { start: 0, end: 1 },
        negated: false,
        kind: ClassSet::Default,
    }));
    visitor.push(HirFrame::ClassBytes(class_bytes));
    visitor.visit_post(&ast).unwrap();
}

