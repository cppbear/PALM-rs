// Answer 0

#[test]
fn test_clear_non_empty_literals() {
    let mut literals = Literals {
        lits: vec![Literal::Unicode('a'), Literal::Byte(1)],
        limit_size: 10,
        limit_class: 5,
    };
    literals.clear();
    assert_eq!(literals.lits.len(), 0);
}

#[test]
fn test_clear_empty_literals() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 10,
        limit_class: 5,
    };
    literals.clear();
    assert_eq!(literals.lits.len(), 0);
}

#[test]
fn test_clear_after_adding_elements() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 10,
        limit_class: 5,
    };
    literals.add(Literal::Unicode('b'));
    literals.add(Literal::Byte(2));
    literals.clear();
    assert_eq!(literals.lits.len(), 0);
}

