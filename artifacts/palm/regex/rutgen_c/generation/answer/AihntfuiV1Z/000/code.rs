// Answer 0

#[test]
fn test_num_bytes_empty() {
    let literals = Literals {
        lits: Vec::new(),
        limit_size: 0,
        limit_class: 0,
    };
    assert_eq!(literals.num_bytes(), 0);
}

#[test]
fn test_num_bytes_single_byte() {
    let literals = Literals {
        lits: vec![Literal::Byte(1)],
        limit_size: 1,
        limit_class: 1,
    };
    assert_eq!(literals.num_bytes(), 1);
}

#[test]
fn test_num_bytes_multiple_bytes() {
    let literals = Literals {
        lits: vec![Literal::Byte(1), Literal::Byte(2), Literal::Byte(3)],
        limit_size: 3,
        limit_class: 1,
    };
    assert_eq!(literals.num_bytes(), 3);
}

#[test]
fn test_num_bytes_single_unicode() {
    let literals = Literals {
        lits: vec![Literal::Unicode('a')],
        limit_size: 1,
        limit_class: 1,
    };
    assert_eq!(literals.num_bytes(), 1);
}

#[test]
fn test_num_bytes_mixed_literals() {
    let literals = Literals {
        lits: vec![Literal::Unicode('a'), Literal::Byte(1)],
        limit_size: 2,
        limit_class: 1,
    };
    assert_eq!(literals.num_bytes(), 2);
}

