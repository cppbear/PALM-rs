// Answer 0

#[test]
fn test_union_with_empty_lits() {
    #[derive(Clone, Eq, PartialEq)]
    struct DummyHir;

    let mut literals1 = Literals {
        lits: vec![Literal::empty()],
        limit_size: 10,
        limit_class: 5,
    };

    let literals2 = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 5,
    };

    assert!(literals1.union(literals2));
    assert_eq!(literals1.lits.len(), 2); // One empty literal should be added
}

#[test]
fn test_union_maximum_size() {
    #[derive(Clone, Eq, PartialEq)]
    struct DummyHir;

    let mut literals1 = Literals {
        lits: vec![
            Literal::new(vec![b'a']),
            Literal::new(vec![b'b']),
        ],
        limit_size: 5,
        limit_class: 5,
    };

    let literals2 = Literals {
        lits: vec![
            Literal::new(vec![b'c']),
        ],
        limit_size: 5,
        limit_class: 5,
    };

    assert!(literals1.union(literals2));
    assert_eq!(literals1.lits.len(), 3); // Should include all literals from literals2
}

#[test]
fn test_union_exceeds_limit() {
    #[derive(Clone, Eq, PartialEq)]
    struct DummyHir;

    let mut literals1 = Literals {
        lits: vec![
            Literal::new(vec![b'a']),
            Literal::new(vec![b'b']),
        ],
        limit_size: 3,
        limit_class: 5,
    };

    let literals2 = Literals {
        lits: vec![
            Literal::new(vec![b'c']),
        ],
        limit_size: 5,
        limit_class: 5,
    };

    assert!(!literals1.union(literals2));
    assert_eq!(literals1.lits.len(), 2); // Should remain unchanged
}

