// Answer 0

#[test]
fn test_cut_empty_literals() {
    let mut literals = Literals::empty();
    literals.cut();
    assert!(literals.is_empty());
}

#[test]
fn test_cut_single_literal_not_cut() {
    let mut literals = Literals {
        lits: vec![Literal::new(vec![b'a'])],
        limit_size: 10,
        limit_class: 10,
    };
    assert!(!literals.lits[0].is_cut());
    literals.cut();
    assert!(literals.lits[0].is_cut());
}

#[test]
fn test_cut_multiple_literals() {
    let mut literals = Literals {
        lits: vec![Literal::new(vec![b'a']), Literal::new(vec![b'b'])],
        limit_size: 10,
        limit_class: 10,
    };
    assert!(!literals.lits[0].is_cut());
    assert!(!literals.lits[1].is_cut());
    literals.cut();
    assert!(literals.lits[0].is_cut());
    assert!(literals.lits[1].is_cut());
}

#[test]
fn test_cut_with_cut_literals() {
    let mut literals = Literals {
        lits: vec![Literal::new(vec![b'a']), Literal::new(vec![b'b'])],
        limit_size: 10,
        limit_class: 10,
    };
    literals.lits[0].cut(); // Cutting the first literal
    literals.cut();
    assert!(literals.lits[0].is_cut());
    assert!(literals.lits[1].is_cut());
}

