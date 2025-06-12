// Answer 0

#[test]
fn test_is_empty_with_empty_literals() {
    let lits = Literals::empty();
    lits.is_empty();
}

#[test]
fn test_is_empty_with_all_empty_literals() {
    let lits = Literals {
        lits: vec![
            Literal::Unicode('\0'),
            Literal::Byte(0x00),
        ],
        limit_size: 0,
        limit_class: 0,
    };
    lits.is_empty();
}

#[test]
fn test_is_empty_with_non_empty_literals() {
    let lits = Literals {
        lits: vec![
            Literal::Unicode('a'),
            Literal::Byte(0x00),
        ],
        limit_size: 0,
        limit_class: 0,
    };
    lits.is_empty();
}

#[test]
fn test_is_empty_with_single_empty_literal() {
    let lits = Literals {
        lits: vec![
            Literal::Unicode('\0'),
        ],
        limit_size: 0,
        limit_class: 0,
    };
    lits.is_empty();
}

#[test]
fn test_is_empty_with_single_byte_literal() {
    let lits = Literals {
        lits: vec![
            Literal::Byte(0x00),
        ],
        limit_size: 0,
        limit_class: 0,
    };
    lits.is_empty();
}

#[test]
fn test_is_empty_with_mixed_literals() {
    let lits = Literals {
        lits: vec![
            Literal::Unicode('a'),
            Literal::Byte(0x00),
            Literal::Unicode('\0'),
        ],
        limit_size: 0,
        limit_class: 0,
    };
    lits.is_empty();
}

