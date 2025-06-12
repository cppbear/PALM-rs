// Answer 0

#[test]
fn test_from_bracketed_with_empty_set() {
    struct MockClassBracketed {
        kind: ast::ClassKind,
    }

    let ast = MockClassBracketed { kind: ast::ClassKind::EmptySet };
    let result = from_bracketed(&ast);
    assert_eq!(result, ClassInduct::empty());
}

#[test]
fn test_from_bracketed_with_single_char() {
    struct MockClassBracketed {
        kind: ast::ClassKind,
    }

    let ast = MockClassBracketed { kind: ast::ClassKind::Single('a') };
    let result = from_bracketed(&ast);
    assert_eq!(result, ClassInduct::from_set(&ast.kind));
}

#[test]
fn test_from_bracketed_with_range() {
    struct MockClassBracketed {
        kind: ast::ClassKind,
    }

    let ast = MockClassBracketed { kind: ast::ClassKind::Range('a', 'z') };
    let result = from_bracketed(&ast);
    assert_eq!(result, ClassInduct::from_set(&ast.kind));
}

#[test]
fn test_from_bracketed_with_multiple_characters() {
    struct MockClassBracketed {
        kind: ast::ClassKind,
    }

    let ast = MockClassBracketed { kind: ast::ClassKind::Multiple(vec!['a', 'b', 'c']) };
    let result = from_bracketed(&ast);
    assert_eq!(result, ClassInduct::from_set(&ast.kind));
}

#[should_panic]
#[test]
fn test_from_bracketed_with_invalid_input() {
    struct MockClassBracketed {
        kind: ast::ClassKind,
    }

    let ast = MockClassBracketed { kind: ast::ClassKind::Invalid };
    from_bracketed(&ast);
}

