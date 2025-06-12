// Answer 0

#[test]
fn test_parse_set_class_open_empty_class() {
    let input = "[^]";
    let mut parser = Parser::new(input.as_bytes());
    let result = parser.parse_set_class_open();
    assert!(result.is_ok());
    let (set, union) = result.unwrap();
    assert!(set.negated);
    assert!(union.items.len() == 1);
    if let ast::ClassSetItem::Literal(ref lit) = union.items[0] {
        assert_eq!(lit.c, ']');
    }
}

#[test]
fn test_parse_set_class_open_with_negation_and_hyphen() {
    let input = "[^-]";
    let mut parser = Parser::new(input.as_bytes());
    let result = parser.parse_set_class_open();
    assert!(result.is_ok());
    let (set, union) = result.unwrap();
    assert!(set.negated);
    assert!(union.items.len() == 1);
    if let ast::ClassSetItem::Literal(ref lit) = union.items[0] {
        assert_eq!(lit.c, '-');
    }
}

#[test]
fn test_parse_set_class_open_with_multiple_hyphens() {
    let input = "[-][-][-]";
    let mut parser = Parser::new(input.as_bytes());
    let result = parser.parse_set_class_open();
    assert!(result.is_ok());
    let (_, union) = result.unwrap();
    assert!(union.items.len() == 3);
    let expected_hyphens = union.items.iter().filter(|item| {
        if let ast::ClassSetItem::Literal(ref lit) = **item {
            lit.c == '-'
        } else {
            false
        }
    }).count();
    assert_eq!(expected_hyphens, 3);
}

#[test]
fn test_parse_set_class_open_unclosed_class() {
    let input = "[";
    let mut parser = Parser::new(input.as_bytes());
    let result = parser.parse_set_class_open();
    assert!(result.is_err());
    // Ensure the error kind is ClassUnclosed.
    assert_eq!(result.unwrap_err().kind(), ast::ErrorKind::ClassUnclosed);
}

#[test]
fn test_parse_set_class_open_only_closing_bracket() {
    let input = "[]";
    let mut parser = Parser::new(input.as_bytes());
    let result = parser.parse_set_class_open();
    assert!(result.is_ok());
    let (_, union) = result.unwrap();
    assert!(union.items.len() == 1);
    if let ast::ClassSetItem::Literal(ref lit) = union.items[0] {
        assert_eq!(lit.c, ']');
    }
}

