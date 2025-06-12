// Answer 0

#[test]
fn test_cross_product_success() {
    struct MockHir;
    
    let mut literals_set = Literals {
        lits: vec![
            Literal::Unicode('a'),
            Literal::Byte(1),
        ],
        limit_size: 20,
        limit_class: 5,
    };
    
    let new_literals = Literals {
        lits: vec![
            Literal::Unicode('b'),
            Literal::Byte(2),
        ],
        limit_size: 20,
        limit_class: 5,
    };
    
    // Making 'self' complete
    literals_set.lits[0].cut = true; // Making first Unicode 'cut'
    literals_set.lits[1].cut = true; // Making Byte 'cut'
    
    let result = literals_set.cross_product(&new_literals);
    
    assert!(result);
    assert_eq!(literals_set.literals().len(), 4); // 2 * 2 from cross product
}

#[test]
fn test_cross_product_exceed_limit() {
    struct MockHir;

    let mut literals_set = Literals {
        lits: vec![
            Literal::Unicode('x'),
            Literal::Byte(255),
        ],
        limit_size: 5,
        limit_class: 5,
    };

    let new_literals = Literals {
        lits: vec![
            Literal::Unicode('y'),
            Literal::Byte(3),
        ],
        limit_size: 10,
        limit_class: 5,
    };

    // Making 'self' complete
    literals_set.lits[0].cut = true; // Making first Unicode 'cut'
    
    let result = literals_set.cross_product(&new_literals);
    
    assert!(!result); // Should return false as it exceeds limit
}

#[test]
fn test_cross_product_empty_lits() {
    struct MockHir;
    
    let mut literals_set = Literals {
        lits: vec![
            Literal::Unicode('c'),
            Literal::Byte(100),
        ],
        limit_size: 10,
        limit_class: 5,
    };

    let new_literals = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 5,
    };

    let result = literals_set.cross_product(&new_literals);
    
    assert!(result); // Should return true as new_literals is empty
    assert_eq!(literals_set.literals().len(), 2); // No change
}

