// Answer 0

#[test]
fn test_class_exceeds_limits_when_size_equals_limit_class_and_has_literals() {
    // Setup
    let limit_size = 10;
    let limit_class = 5;
    let mut literals = Literals {
        lits: vec![Literal::Unicode('a'), Literal::Unicode('b')],
        limit_size,
        limit_class,
    };

    // Creating a situation where size == limit_class
    let size = literals.limit_class();

    // Adding enough literals to exceed limit_size
    literals.lits.push(Literal::Unicode('c')); // Adding another literal to avoid is_empty()

    // Testing
    assert!(literals.class_exceeds_limits(size));
}

#[test]
fn test_class_exceeds_limits_when_size_equals_limit_class_with_cut_literals() {
    // Setup
    let limit_size = 10;
    let limit_class = 5;
    let mut literals = Literals {
        lits: vec![Literal::Unicode('a'), Literal::Unicode('b')],
        limit_size,
        limit_class,
    };

    // Creating a situation where size == limit_class
    let size = literals.limit_class();

    // Adding cut literal to ensure count remains reasonable
    literals.lits.push(Literal::Byte(0xFF)); // This literal won't be counted due to being cut

    // Testing
    assert!(literals.class_exceeds_limits(size)); // Expected to return true still since cut is included
}

#[test]
fn test_class_exceeds_limits_with_max_literals() {
    // Setup
    let limit_size = 10;
    let limit_class = 3;
    let mut literals = Literals {
        lits: vec![
            Literal::Unicode('a'), 
            Literal::Unicode('b'), 
            Literal::Unicode('c')
        ],
        limit_size,
        limit_class,
    };

    // Creating a situation where size == limit_class
    let size = literals.limit_class();

    // This should cause a failure since number of literals is too many
    assert!(literals.class_exceeds_limits(size));
}

