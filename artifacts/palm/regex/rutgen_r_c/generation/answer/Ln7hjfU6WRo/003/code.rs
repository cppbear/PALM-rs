// Answer 0

#[test]
fn test_longest_common_prefix_non_empty() {
    // Arrange
    let literal1 = Literal::Unicode('a');
    let literal2 = Literal::Unicode('a');
    let literal3 = Literal::Unicode('a');
    let literals = Literals {
        lits: vec![literal1.clone(), literal2.clone(), literal3.clone()],
        limit_size: 10,
        limit_class: 5,
    };

    // Act
    let result = literals.longest_common_prefix();

    // Assert
    assert_eq!(result, b"a");
}

#[test]
fn test_longest_common_prefix_with_different_literals() {
    // Arrange
    let literal1 = Literal::Unicode('a');
    let literal2 = Literal::Unicode('a');
    let literal3 = Literal::Unicode('b');
    let literals = Literals {
        lits: vec![literal1.clone(), literal2.clone(), literal3.clone()],
        limit_size: 10,
        limit_class: 5,
    };

    // Act
    let result = literals.longest_common_prefix();

    // Assert
    assert_eq!(result, b"a");
}

#[test]
fn test_longest_common_prefix_empty() {
    // Arrange
    let literal = Literal::Unicode('a');
    let literals = Literals {
        lits: vec![literal],
        limit_size: 10,
        limit_class: 5,
    };

    // Act
    let result = literals.longest_common_prefix();

    // Assert
    assert_eq!(result, b"a");
}

#[test]
fn test_longest_common_prefix_empty_lits() {
    // Arrange
    let literals = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 5,
    };

    // Act
    let result = literals.longest_common_prefix();

    // Assert
    assert!(result.is_empty());
}

#[test]
fn test_longest_common_prefix_partial_match() {
    // Arrange
    let literal1 = Literal::Unicode('a');
    let literal2 = Literal::Unicode('a');
    let literal3 = Literal::Unicode('a');
    let literal4 = Literal::Unicode('b');
    let literals = Literals {
        lits: vec![literal1, literal2, literal3, literal4],
        limit_size: 10,
        limit_class: 5,
    };

    // Act
    let result = literals.longest_common_prefix();

    // Assert
    assert_eq!(result, b"a");
}

