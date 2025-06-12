// Answer 0

#[test]
fn test_is_empty_with_no_literals() {
    let literals = Literals {
        lits: Vec::new(),
        limit_size: 0,
        limit_class: 0,
    };
    assert!(literals.is_empty());
}

#[test]
fn test_is_empty_with_one_empty_literal() {
    let literals = Literals {
        lits: vec![Literal::Unicode(' ')],
        limit_size: 0,
        limit_class: 0,
    };
    assert!(literals.is_empty());
}

#[test]
fn test_is_empty_with_multiple_empty_literals() {
    let literals = Literals {
        lits: vec![
            Literal::Unicode(' '),
            Literal::Unicode(' '),
        ],
        limit_size: 0,
        limit_class: 0,
    };
    assert!(literals.is_empty());
}

#[test]
fn test_is_empty_with_non_empty_literal() {
    let literals = Literals {
        lits: vec![Literal::Unicode('a')],
        limit_size: 0,
        limit_class: 0,
    };
    assert!(!literals.is_empty());
}

#[test]
fn test_is_empty_with_mixed_literals() {
    let literals = Literals {
        lits: vec![
            Literal::Unicode(' '),
            Literal::Unicode('a'),
        ],
        limit_size: 0,
        limit_class: 0,
    };
    assert!(!literals.is_empty());
}

