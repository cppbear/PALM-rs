// Answer 0

#[test]
fn test_into_class_set_item_perl_class() {
    #[derive(Clone)]
    struct DummyParser;

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &Parser {}
        }
    }

    let span = Span { start: 0, end: 1 };
    let perl_class = ClassPerl { span: span.clone(), kind: ClassPerlKind::Digit, negated: false };
    let primitive = Primitive::Perl(perl_class);
    let parser_instance = ParserI::new(DummyParser {}, "test_pattern");

    let result = primitive.into_class_set_item(&parser_instance);
}

#[test]
fn test_into_class_set_item_perl_class_negated() {
    #[derive(Clone)]
    struct DummyParser;

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &Parser {}
        }
    }

    let span = Span { start: 0, end: 1 };
    let perl_class = ClassPerl { span: span.clone(), kind: ClassPerlKind::Word, negated: true };
    let primitive = Primitive::Perl(perl_class);
    let parser_instance = ParserI::new(DummyParser {}, "test_pattern");

    let result = primitive.into_class_set_item(&parser_instance);
}

