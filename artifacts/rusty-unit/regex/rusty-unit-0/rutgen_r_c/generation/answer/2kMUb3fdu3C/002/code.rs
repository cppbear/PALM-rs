// Answer 0

#[test]
fn test_add_literal_with_exact_limit_size() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 5,
        limit_class: 10,
    };

    let literal = Literal::Unicode('a');
    literals.add_char(literal); // Assuming a method to add character literals

    // Check that the number of bytes is exactly the limit size
    let result = literals.add(Literal::Unicode('b')); // This should still fit within the limit
    assert!(result);
    assert_eq!(literals.num_bytes(), 5);
}

#[test]
fn test_add_literal_exceeding_limit_size() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 5,
        limit_class: 10,
    };

    literals.add(Literal::Unicode('a')); // 1 byte
    literals.add(Literal::Unicode('b')); // 1 byte
    literals.add(Literal::Unicode('c')); // 1 byte
    literals.add(Literal::Unicode('d')); // 1 byte

    // Adding this literal would exceed the limit_size
    let result = literals.add(Literal::Unicode('e')); // 1 byte, total would be 5
    assert!(result); // this should still return true

    let result_exceed = literals.add(Literal::Unicode('f')); // Adding this would exceed limit size
    assert!(!result_exceed); // this should return false
}

#[test]
fn test_add_empty_literal() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 5,
        limit_class: 10,
    };

    let result = literals.add(Literal::Unicode(' ')); // Adding space character
    assert!(result);
    assert_eq!(literals.num_bytes(), 1); // Should be one byte now
}

