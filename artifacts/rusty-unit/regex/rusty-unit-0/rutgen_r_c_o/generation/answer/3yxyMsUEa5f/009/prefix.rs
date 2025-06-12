// Answer 0

#[test]
fn test_cross_product_empty_self_exceeds_limit() {
    let mut self_literals = Literals {
        lits: Vec::new(),
        limit_size: 0,
        limit_class: 1,
    };

    let lits = Literals {
        lits: vec![Literal::Unicode('a')],
        limit_size: 1,
        limit_class: 1,
    };

    let result = self_literals.cross_product(&lits);
}

#[test]
fn test_cross_product_empty_self_with_non_empty_bytes_exceeds_limit() {
    let mut self_literals = Literals {
        lits: Vec::new(),
        limit_size: 0,
        limit_class: 1,
    };

    let lits = Literals {
        lits: vec![Literal::Byte(1)],
        limit_size: 1,
        limit_class: 1,
    };

    let result = self_literals.cross_product(&lits);
}

#[test]
fn test_cross_product_empty_self_with_multiple_literals_exceeds_limit() {
    let mut self_literals = Literals {
        lits: Vec::new(),
        limit_size: 0,
        limit_class: 1,
    };

    let lits = Literals {
        lits: vec![Literal::Byte(1), Literal::Unicode('b')],
        limit_size: 2,
        limit_class: 1,
    };

    let result = self_literals.cross_product(&lits);
}

