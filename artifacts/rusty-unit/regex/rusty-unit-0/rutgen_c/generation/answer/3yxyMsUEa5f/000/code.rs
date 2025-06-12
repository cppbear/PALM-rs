// Answer 0

#[test]
fn test_cross_product_with_empty_literal_set() {
    let mut literals_a = Literals::empty();
    let literals_b = Literals::empty();
    assert_eq!(literals_a.cross_product(&literals_b), true);
}

#[test]
fn test_cross_product_with_non_empty_literal_set() {
    let mut literals_a = Literals {
        lits: vec![Literal::new(vec![b'a'])],
        limit_size: 10,
        limit_class: 1,
    };
    let literals_b = Literals {
        lits: vec![Literal::new(vec![b'b'])],
        limit_size: 10,
        limit_class: 1,
    };
    assert_eq!(literals_a.cross_product(&literals_b), true);
    assert_eq!(literals_a.literals().len(), 1);
}

#[test]
fn test_cross_product_exceeding_limit() {
    let mut literals_a = Literals {
        lits: vec![Literal::new(vec![b'a'])],
        limit_size: 1,
        limit_class: 1,
    };
    let literals_b = Literals {
        lits: vec![Literal::new(vec![b'b'])],
        limit_size: 1,
        limit_class: 1,
    };
    assert_eq!(literals_a.cross_product(&literals_b), false);
}

#[test]
fn test_cross_product_creates_new_literals() {
    let mut literals_a = Literals {
        lits: vec![Literal::new(vec![b'a']), Literal::new(vec![b'c'])],
        limit_size: 10,
        limit_class: 1,
    };
    let literals_b = Literals {
        lits: vec![Literal::new(vec![b'b'])],
        limit_size: 10,
        limit_class: 1,
    };
    assert_eq!(literals_a.cross_product(&literals_b), true);
    assert_eq!(literals_a.literals().len(), 2);
}

#[test]
fn test_cross_product_with_cut_literals() {
    let mut literals_a = Literals {
        lits: vec![Literal::new(vec![b'a'])],
        limit_size: 10,
        limit_class: 1,
    };
    literals_a.lits[0].cut = true; // Mark as cut
    let literals_b = Literals {
        lits: vec![Literal::new(vec![b'b']), Literal::new(vec![b'c'])],
        limit_size: 10,
        limit_class: 1,
    };
    assert_eq!(literals_a.cross_product(&literals_b), true);
    assert_eq!(literals_a.literals().len(), 2);
}

