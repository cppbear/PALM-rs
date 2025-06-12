// Answer 0

#[test]
fn test_add_byte_class_success() {
    // Arrange
    let mut literals = Literals {
        lits: vec![Literal::empty()],
        limit_size: 100,
        limit_class: 10,
    };
    let class_bytes = ClassBytes::new(vec![
        ClassBytesRange { start: 1, end: 3 }
    ]);

    // Act
    let result = literals.add_byte_class(&class_bytes);

    // Assert
    assert!(result);
    assert_eq!(literals.lits.len(), 3);
    assert_eq!(literals.lits[0], Literal::new(vec![1]));
    assert_eq!(literals.lits[1], Literal::new(vec![2]));
    assert_eq!(literals.lits[2], Literal::new(vec![3]));
}

#[test]
fn test_add_byte_class_exceeds_limit() {
    // Arrange
    let mut literals = Literals {
        lits: vec![Literal::empty()],
        limit_size: 5,
        limit_class: 1,
    };
    let class_bytes = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 255 }
    ]);

    // Act
    let result = literals.add_byte_class(&class_bytes);

    // Assert
    assert!(!result);
    assert_eq!(literals.lits.len(), 1);
}

#[test]
fn test_add_byte_class_empty_base() {
    // Arrange
    let mut literals = Literals {
        lits: vec![],
        limit_size: 100,
        limit_class: 10,
    };
    let class_bytes = ClassBytes::new(vec![
        ClassBytesRange { start: 1, end: 2 }
    ]);

    // Act
    let result = literals.add_byte_class(&class_bytes);

    // Assert
    assert!(result);
    assert_eq!(literals.lits.len(), 2);
    assert_eq!(literals.lits[0], Literal::new(vec![1]));
    assert_eq!(literals.lits[1], Literal::new(vec![2]));
}

#[test]
fn test_add_byte_class_no_range_in_class() {
    // Arrange
    let mut literals = Literals {
        lits: vec![Literal::new(vec![1])],
        limit_size: 100,
        limit_class: 10,
    };
    let class_bytes = ClassBytes::new(vec![]);

    // Act
    let result = literals.add_byte_class(&class_bytes);

    // Assert
    assert!(result);
    assert_eq!(literals.lits.len(), 1);
    assert_eq!(literals.lits[0], Literal::new(vec![1]));
}

#[test]
fn test_add_byte_class_class_exceeds_limit() {
    // Arrange
    let mut literals = Literals {
        lits: vec![Literal::new(vec![1])],
        limit_size: 100,
        limit_class: 1,
    };
    let class_bytes = ClassBytes::new(vec![
        ClassBytesRange { start: 2, end: 4 }
    ]);

    // Act
    let result = literals.add_byte_class(&class_bytes);

    // Assert
    assert!(!result);
    assert_eq!(literals.lits.len(), 1);
}

