// Answer 0

#[test]
fn test_add_byte_class_success() {
    let mut literals = Literals {
        lits: vec![],
        limit_size: 100,
        limit_class: 10,
    };
    
    let mut class_bytes = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 3 }]);
    
    let result = literals.add_byte_class(&class_bytes);
    
    assert!(result);
    assert_eq!(literals.lits.len(), 3); // 1, 2, 3 added to the literals
}

#[test]
fn test_add_byte_class_exceeds_limit() {
    let mut literals = Literals {
        lits: vec![Literal::empty()],
        limit_size: 10,
        limit_class: 1, // This restricts the classes allowed
    };

    let mut class_bytes = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 10 }]); // Exceeds limit

    let result = literals.add_byte_class(&class_bytes);
    
    assert!(!result);
    assert_eq!(literals.lits.len(), 1); // No new literals should be added
}

#[test]
fn test_add_byte_class_empty_class() {
    let mut literals = Literals {
        lits: vec![],
        limit_size: 100,
        limit_class: 10,
    };
    
    let class_bytes = ClassBytes::empty(); // No ranges provided

    let result = literals.add_byte_class(&class_bytes);
    
    assert!(result);
    assert!(literals.is_empty()); // No literals should have been added
}

#[test]
fn test_add_byte_class_single_update() {
    let mut literals = Literals {
        lits: vec![Literal::new(vec![b'a'])],
        limit_size: 100,
        limit_class: 10,
    };
    
    let mut class_bytes = ClassBytes::new(vec![ClassBytesRange { start: 2, end: 2 }]); // Only adds 'b'

    let result = literals.add_byte_class(&class_bytes);
    
    assert!(result);
    assert_eq!(literals.lits.len(), 2); // 'ab' should be added
    assert_eq!(literals.lits[1].v, b"ab"); // Check the content of the newly added literal
}

