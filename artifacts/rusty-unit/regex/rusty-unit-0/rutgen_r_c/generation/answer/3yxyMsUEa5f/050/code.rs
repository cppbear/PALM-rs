// Answer 0

#[test]
fn test_cross_product_success() {
    // Create the first Literals instance with some literals, not complete
    let mut self_literals = Literals {
        lits: vec![
            Literal::Unicode('a'),
            Literal::Unicode('b'),
        ],
        limit_size: 10,
        limit_class: 5,
    };

    // Create the second Literals instance with literals
    let lits = Literals {
        lits: vec![
            Literal::Unicode('x'),
            Literal::Unicode('y'),
        ],
        limit_size: 10,
        limit_class: 5,
    };

    // Invoke the method
    let result = self_literals.cross_product(&lits);

    // Validate expectations
    assert!(result);
    assert_eq!(self_literals.literals().len(), 4); // Expect 4 combinations: ax, ay, bx, by
}

#[test]
fn test_cross_product_exceeds_limit() {
    // Create the first Literals instance with literals, not complete
    let mut self_literals = Literals {
        lits: vec![
            Literal::Unicode('a'),
            Literal::Unicode('b'),
        ],
        limit_size: 5, // Limit is small
        limit_class: 2,
    };

    // Create the second Literals instance with larger literals
    let lits = Literals {
        lits: vec![
            Literal::Unicode('x'), // Length will push limit over
            Literal::Unicode('y'),
        ],
        limit_size: 10,
        limit_class: 5,
    };

    // Invoke the method
    let result = self_literals.cross_product(&lits);

    // Validate expectations
    assert!(!result);
    assert_eq!(self_literals.literals().len(), 2); // No new combinations added due to limit
}

#[test]
fn test_cross_product_with_empty_expr() {
    // Create the first Literals instance with some literals
    let mut self_literals = Literals {
        lits: vec![
            Literal::Unicode('a'),
            Literal::Unicode('b'),
        ],
        limit_size: 10,
        limit_class: 5,
    };

    // Create an empty second Literals instance
    let lits = Literals {
        lits: Vec::new(),
        limit_size: 10,
        limit_class: 5,
    };

    // Invoke the method
    let result = self_literals.cross_product(&lits);

    // Validate expectations
    assert!(result);
    assert_eq!(self_literals.literals().len(), 2); // No change in size
}

#[test]
fn test_cross_product_with_only_cuts() {
    // Create the first Literals instance with cut literals
    let mut self_literals = Literals {
        lits: vec![
            Literal::Unicode('a'), // Complete literal
            Literal::Unicode('b'), // Complete literal
        ],
        limit_size: 10,
        limit_class: 5,
    };

    // Create the second Literals instance with cut literals
    let lits = Literals {
        lits: vec![
            Literal::Unicode('x'), // Length will considered 
            Literal::Unicode('y'), // Length will considered 
        ],
        limit_size: 10,
        limit_class: 5,
    };

    // Invoke the method
    let result = self_literals.cross_product(&lits);

    // Validate expectations
    assert!(result);
    assert_eq!(self_literals.literals().len(), 4); // Expect new literals are added
}

