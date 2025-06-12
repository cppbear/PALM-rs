// Answer 0

#[test]
fn test_add_literal_within_limit() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 10,
        limit_class: 5,
    };
    let literal = Literal::Unicode('a');
    assert_eq!(literals.add(literal), true);
    assert_eq!(literals.lits.len(), 1);
}

#[test]
fn test_add_literal_exceeds_limit() {
    let mut literals = Literals {
        lits: vec![Literal::Unicode('a'), Literal::Unicode('b')],
        limit_size: 2, // Total size limit is 2 bytes
        limit_class: 5,
    };
    let literal = Literal::Unicode('c');
    assert_eq!(literals.add(literal), false);
    assert_eq!(literals.lits.len(), 2); // Should not change
}

#[test]
fn test_add_multiple_literals_within_limit() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 3,
        limit_class: 5,
    };
    let lit1 = Literal::Unicode('a');
    let lit2 = Literal::Unicode('b');
    assert_eq!(literals.add(lit1), true);
    assert_eq!(literals.add(lit2), true);
    assert_eq!(literals.lits.len(), 2);
}

#[test]
fn test_add_character_byte() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 4,
        limit_class: 5,
    };
    let literal = Literal::Byte(255);
    assert_eq!(literals.add(literal), true);
    assert_eq!(literals.lits.len(), 1);
}

