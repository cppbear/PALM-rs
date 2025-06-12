// Answer 0

#[test]
fn test_clear() {
    let mut literals = Literals {
        lits: vec![Literal::Unicode('a'), Literal::Byte(1)],
        limit_size: 10,
        limit_class: 5,
    };
    assert_eq!(literals.lits.len(), 2);
    
    literals.clear();
    assert_eq!(literals.lits.len(), 0);
}

#[test]
fn test_clear_empty() {
    let mut literals = Literals {
        lits: vec![],
        limit_size: 0,
        limit_class: 0,
    };
    assert_eq!(literals.lits.len(), 0);
    
    literals.clear();
    assert_eq!(literals.lits.len(), 0);
}

