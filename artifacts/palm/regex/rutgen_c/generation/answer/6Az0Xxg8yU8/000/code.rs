// Answer 0

#[test]
fn test_union_success() {
    let mut literals1 = Literals {
        lits: vec![Literal::Unicode('a')],
        limit_size: 5,
        limit_class: 1,
    };
    let literals2 = Literals {
        lits: vec![Literal::Unicode('b')],
        limit_size: 5,
        limit_class: 1,
    };

    assert_eq!(literals1.union(literals2), true);
    assert_eq!(literals1.lits.len(), 2);
}

#[test]
fn test_union_exceeds_limit() {
    let mut literals1 = Literals {
        lits: vec![Literal::Unicode('a'), Literal::Unicode('b')],
        limit_size: 3,
        limit_class: 1,
    };
    let literals2 = Literals {
        lits: vec![Literal::Unicode('c')],
        limit_size: 5,
        limit_class: 1,
    };

    assert_eq!(literals1.union(literals2), false);
    assert_eq!(literals1.lits.len(), 2);
}

#[test]
fn test_union_with_empty_literals() {
    let mut literals1 = Literals {
        lits: vec![Literal::Unicode('a')],
        limit_size: 5,
        limit_class: 1,
    };
    let literals2 = Literals {
        lits: vec![],
        limit_size: 5,
        limit_class: 1,
    };

    assert_eq!(literals1.union(literals2), true);
    assert_eq!(literals1.lits.len(), 2); // Should now include one empty literal
} 

#[test]
fn test_union_empty_to_non_empty() {
    let mut literals1 = Literals {
        lits: vec![],
        limit_size: 5,
        limit_class: 1,
    };
    let literals2 = Literals {
        lits: vec![Literal::Unicode('a')],
        limit_size: 5,
        limit_class: 1,
    };

    assert_eq!(literals1.union(literals2), true);
    assert_eq!(literals1.lits.len(), 1);
}

