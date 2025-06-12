// Answer 0

#[test]
fn test_cross_add_with_non_empty_bytes_and_full_capacity() {
    let mut literals = Literals {
        lits: vec![
            Literal::new(vec![b'a']),
            Literal::new(vec![b'b']),
        ],
        limit_size: 4,
        limit_class: 0,
    };
    
    assert_eq!(literals.cross_add(&[b'c', b'd']), false);
}

#[test]
fn test_cross_add_with_full_capacity_and_minimum_bytes() {
    let mut literals = Literals {
        lits: vec![
            Literal::new(vec![b'x']),
            Literal::new(vec![b'y']),
        ],
        limit_size: 2,
        limit_class: 0,
    };

    assert_eq!(literals.cross_add(&[b'z']), false);
}

#[test]
fn test_cross_add_with_bytes_just_fitting() {
    let mut literals = Literals {
        lits: vec![
            Literal::new(vec![b'1']),
            Literal::new(vec![b'2']),
        ],
        limit_size: 4,
        limit_class: 0,
    };

    assert_eq!(literals.cross_add(&[b'3']), false);
}

#[test]
fn test_cross_add_with_bytes_exceeding_limit_size() {
    let mut literals = Literals {
        lits: vec![
            Literal::new(vec![b'a']),
            Literal::new(vec![b'b']),
        ],
        limit_size: 3,
        limit_class: 0,
    };

    assert_eq!(literals.cross_add(&[b'c', b'd']), false);
}

