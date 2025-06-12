// Answer 0

#[test]
fn test_class_exceeds_limits_with_empty_lits() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 1, // limit_size < limit_class + 1
        limit_class: 1, // size == limit_class
    };

    let size = literals.limit_class; // size <= limit_class
    literals.class_exceeds_limits(size);
}

#[test]
fn test_class_exceeds_limits_with_empty_lits_edge_case() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 0, // limit_size < limit_class + 1
        limit_class: 1, // size == limit_class
    };

    let size = literals.limit_class; // size <= limit_class
    literals.class_exceeds_limits(size);
}

