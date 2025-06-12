// Answer 0

#[test]
fn test_pattern() {
    struct MockParser;
    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            static RESULT: Parser = Parser {
                ast: ast::parse::Parser {},
                hir: hir::translate::Translator {},
            };
            &RESULT
        }
    }

    let mock_parser = MockParser;
    let pattern_value = "abc";
    let parser_i = ParserI::new(mock_parser, pattern_value);
    assert_eq!(parser_i.pattern(), pattern_value);
}

