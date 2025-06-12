// Answer 0

#[test]
fn test_class_exceeds_limits_when_empty_and_at_limit_class() {
    let limit_size = 10;
    let limit_class = 5;
    let size = limit_class; // size should equal limit_class

    // Creating an empty Literals instance
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size,
        limit_class,
    };

    // As there are no literals, new_byte_count will be equal to size
    assert!(literals.class_exceeds_limits(size));
}

#[test]
fn test_class_exceeds_limits_when_empty_and_beyond_limit_size() {
    let limit_size = 5;
    let limit_class = 3;
    let size = limit_class; // size should equal limit_class

    // Creating an empty Literals instance
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size,
        limit_class,
    };

    // new_byte_count will be equal to size
    assert!(!literals.class_exceeds_limits(size)); // Should not exceed as both are equal and empty
}

