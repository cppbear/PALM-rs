// Answer 0

#[test]
fn test_add_literal_exceeds_limit_size() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 1000,
        limit_class: 10,
    };
    let lit = Literal::Unicode('a');
    literals.lits = vec![lit.clone(); 999]; // 999 bytes already in the set
    assert_eq!(literals.num_bytes(), 999);
    assert_eq!(lit.len(), 1);
    let result = literals.add(lit);
}

#[test]
fn test_add_literal_exceeds_limit_size_edge_case() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 500,
        limit_class: 10,
    };
    let lit = Literal::Byte(255);
    literals.lits = vec![lit.clone(); 499]; // 499 bytes already in the set
    assert_eq!(literals.num_bytes(), 499);
    assert_eq!(lit.len(), 1);
    let result = literals.add(lit);
}

#[test]
fn test_add_literal_at_limit_size() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 1000,
        limit_class: 10,
    };
    let lit = Literal::Unicode('b');
    literals.lits = vec![lit.clone(); 1000]; // Exactly at limit size
    assert_eq!(literals.num_bytes(), 1000);
    assert_eq!(lit.len(), 1);
    let result = literals.add(Literal::Byte(1));
}

