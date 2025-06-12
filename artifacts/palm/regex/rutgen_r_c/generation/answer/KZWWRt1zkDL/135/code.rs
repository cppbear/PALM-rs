// Answer 0

#[test]
fn test_parse_escape_octals_disabled() {
    struct DummyParser {
        parser: Parser,
        pattern: String,
        pos: Position,
    }

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &self.parser
        }
    }

    let span = Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 });
    let parser = Parser { octal: false, ..Default::default() };
    let dummy_parser = DummyParser { parser, pattern: "\\8".to_string(), pos: Position { offset: 0, line: 1, column: 1 } };
    let result = dummy_parser.parse_escape();
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, ErrorKind::UnsupportedBackreference);
}

#[test]
fn test_parse_escape_unicode() {
    struct DummyParser {
        parser: Parser,
        pattern: String,
        pos: Position,
    }

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &self.parser
        }
    }

    let span = Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 3 });
    let parser = Parser { octal: false, ..Default::default() };
    let dummy_parser = DummyParser { parser, pattern: "\\u0061".to_string(), pos: Position { offset: 0, line: 1, column: 1 } };
    let result = dummy_parser.parse_escape();

    assert!(result.is_ok());
    if let Ok(Primitive::Literal(ref lit)) = result {
        assert_eq!(lit.c, 'a');
    }
}

#[test]
fn test_parse_escape_invalid_escape() {
    struct DummyParser {
        parser: Parser,
        pattern: String,
        pos: Position,
    }

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &self.parser
        }
    }

    let span = Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 3 });
    let parser = Parser { octal: false, ..Default::default() };
    let dummy_parser = DummyParser { parser, pattern: "\\xg".to_string(), pos: Position { offset: 0, line: 1, column: 1 } };
    let result = dummy_parser.parse_escape();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, ErrorKind::EscapeUnrecognized);
}

#[test]
fn test_parse_escape_special_characters() {
    struct DummyParser {
        parser: Parser,
        pattern: String,
        pos: Position,
    }

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &self.parser
        }
    }

    let span = Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 2 });
    let parser = Parser { octal: false, ..Default::default() };
    let dummy_parser = DummyParser { parser, pattern: "\\t".to_string(), pos: Position { offset: 0, line: 1, column: 1 } };
    let result = dummy_parser.parse_escape();

    assert!(result.is_ok());
    if let Ok(Primitive::Literal(ref lit)) = result {
        assert_eq!(lit.kind, LiteralKind::Special(SpecialLiteralKind::Tab));
        assert_eq!(lit.c, '\t');
    }
}

#[test]
fn test_parse_escape_meta_character() {
    struct DummyParser {
        parser: Parser,
        pattern: String,
        pos: Position,
    }

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &self.parser
        }
    }

    let span = Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 2 });
    let parser = Parser { octal: false, ..Default::default() };
    let dummy_parser = DummyParser { parser, pattern: "\\*".to_string(), pos: Position { offset: 0, line: 1, column: 1 } };
    let result = dummy_parser.parse_escape();

    assert!(result.is_ok());
    if let Ok(Primitive::Literal(ref lit)) = result {
        assert_eq!(lit.kind, LiteralKind::Punctuation);
        assert_eq!(lit.c, '*');
    }
}

