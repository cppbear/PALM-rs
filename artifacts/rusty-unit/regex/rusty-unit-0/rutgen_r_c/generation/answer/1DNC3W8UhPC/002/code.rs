// Answer 0

#[test]
fn test_is_empty_with_non_empty_lits() {
    let literal1 = Literal::Unicode('a');
    let literal2 = Literal::Byte(98);
    let non_empty_literals = Literals {
        lits: vec![literal1.clone(), literal2.clone()],
        limit_size: 0,
        limit_class: 0,
    };

    assert!(!non_empty_literals.is_empty());
}

#[test]
fn test_is_empty_with_non_empty_lits_all_empty() {
    let empty_literal = Literal::Unicode('\0');
    let non_empty_literals = Literals {
        lits: vec![empty_literal.clone(), empty_literal.clone()],
        limit_size: 0,
        limit_class: 0,
    };

    assert!(non_empty_literals.is_empty());
}

#[test]
fn test_is_empty_with_single_non_empty_literal() {
    let literal = Literal::Unicode('b');
    let single_literal = Literals {
        lits: vec![literal],
        limit_size: 0,
        limit_class: 0,
    };

    assert!(!single_literal.is_empty());
}

#[test]
fn test_is_empty_with_mixed_literals() {
    let literal1 = Literal::Unicode('c');
    let literal2 = Literal::Byte(0);
    let mixed_literals = Literals {
        lits: vec![literal1, literal2],
        limit_size: 0,
        limit_class: 0,
    };

    assert!(!mixed_literals.is_empty());
}

