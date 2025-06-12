// Answer 0

#[test]
fn test_visit_post_literal() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };

    let ast = Ast::Literal(Literal {
        span: Span::default(),
        kind: LiteralKind::Verbatim,
        c: 'a',
    });

    let result = writer.visit_post(&ast);
    assert_eq!(result, Ok(()));
    assert_eq!(output, "a");
}

#[test]
fn test_visit_post_unicode_class() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };

    let ast = Ast::Class(Class::Unicode(ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ClassUnicodeKind::OneLetter('x'),
    }));

    let result = writer.visit_post(&ast);
    assert_eq!(result, Ok(()));
    assert_eq!(output, r"\px");
}

#[test]
fn test_visit_post_bracketed_class() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };

    let ast = Ast::Class(Class::Bracketed(ClassBracketed {
        span: Span::default(),
        negated: true,
        kind: ClassSet::Normal,
    }));

    let result = writer.visit_post(&ast);
    assert_eq!(result, Ok(()));
    assert_eq!(output, "]");
}

