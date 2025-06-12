// Answer 0

#[test]
fn test_parse_escape_with_valid_unicode_for_p() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let pattern = "\\p{Sc}";
    let parser = ParserI { parser: Parser { octal: true, ..Default::default() }, pattern };
    
    let result = parser.parse_escape();
    assert!(result.is_ok(), "Expected Ok result but got Err: {:?}", result);
    if let Ok(Primitive::Unicode(cls)) = result {
        assert_eq!(cls.negated, false);
    }
}

#[test]
fn test_parse_escape_with_valid_unicode_for_P() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let pattern = "\\P{Sc}";
    let parser = ParserI { parser: Parser { octal: true, ..Default::default() }, pattern };
    
    let result = parser.parse_escape();
    assert!(result.is_ok(), "Expected Ok result but got Err: {:?}", result);
    if let Ok(Primitive::Unicode(cls)) = result {
        assert_eq!(cls.negated, true);
    }
}

#[test]
fn test_parse_escape_with_perl_class_d() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let pattern = "\\d";
    let parser = ParserI { parser: Parser { octal: true, ..Default::default() }, pattern };
    
    let result = parser.parse_escape();
    assert!(result.is_ok(), "Expected Ok result but got Err: {:?}", result);
    if let Ok(Primitive::Perl(cls)) = result {
        assert_eq!(cls.kind, ast::ClassPerlKind::Digit);
        assert_eq!(cls.negated, false);
    }
}

#[test]
fn test_parse_escape_with_perl_class_D() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let pattern = "\\D";
    let parser = ParserI { parser: Parser { octal: true, ..Default::default() }, pattern };
    
    let result = parser.parse_escape();
    assert!(result.is_ok(), "Expected Ok result but got Err: {:?}", result);
    if let Ok(Primitive::Perl(cls)) = result {
        assert_eq!(cls.kind, ast::ClassPerlKind::Digit);
        assert_eq!(cls.negated, true);
    }
}

#[test]
fn test_parse_escape_with_special_character_n() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let pattern = "\\n";
    let parser = ParserI { parser: Parser { octal: true, ..Default::default() }, pattern };
    
    let result = parser.parse_escape();
    assert!(result.is_ok(), "Expected Ok result but got Err: {:?}", result);
    if let Ok(Primitive::Literal(lit)) = result {
        assert_eq!(lit.kind, ast::LiteralKind::Special(ast::SpecialLiteralKind::LineFeed));
        assert_eq!(lit.c, '\n');
    }
}

#[test]
fn test_parse_escape_with_meta_character() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let pattern = "\\*"; // A meta character that is escaped
    let parser = ParserI { parser: Parser { octal: true, ..Default::default() }, pattern };

    let result = parser.parse_escape();
    assert!(result.is_ok(), "Expected Ok result but got Err: {:?}", result);
    if let Ok(Primitive::Literal(lit)) = result {
        assert_eq!(lit.kind, ast::LiteralKind::Punctuation);
        assert_eq!(lit.c, '*');
    }
}

#[test]
#[should_panic]
fn test_parse_escape_with_unrecognized_escape() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let pattern = "\\z"; // An invalid escape sequence can cause a panic
    let parser = ParserI { parser: Parser { octal: true, ..Default::default() }, pattern };

    let _ = parser.parse_escape(); // This should panic
}

