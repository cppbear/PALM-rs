// Answer 0

#[test]
fn test_union_success_non_empty() {
    struct DummyHir;

    let mut literals_a = Literals {
        lits: vec![
            Literal::Unicode('a'),
            Literal::Byte(1),
        ],
        limit_size: 10,
        limit_class: 1,
    };
    
    let literals_b = Literals {
        lits: vec![
            Literal::Unicode('b'),
            Literal::Byte(2),
        ],
        limit_size: 10,
        limit_class: 1,
    };

    let result = literals_a.union(literals_b);
    assert!(result);
    assert_eq!(literals_a.lits.len(), 4);
} 

#[test]
fn test_union_success_with_exact_limit() {
    struct DummyHir;

    let mut literals_a = Literals {
        lits: vec![
            Literal::Unicode('c'),
        ],
        limit_size: 3,
        limit_class: 1,
    };

    let literals_b = Literals {
        lits: vec![
            Literal::Byte(3),
            Literal::Byte(4),
        ],
        limit_size: 3,
        limit_class: 1,
    };

    let result = literals_a.union(literals_b);
    assert!(result);
    assert_eq!(literals_a.lits.len(), 3); 
} 

#[test]
fn test_union_success_empty_first() {
    struct DummyHir;

    let mut literals_a = Literals {
        lits: vec![],
        limit_size: 5,
        limit_class: 1,
    };

    let literals_b = Literals {
        lits: vec![
            Literal::Unicode('d'),
            Literal::Byte(5),
        ],
        limit_size: 5,
        limit_class: 1,
    };

    let result = literals_a.union(literals_b);
    assert!(result);
    assert_eq!(literals_a.lits.len(), 2);
}

