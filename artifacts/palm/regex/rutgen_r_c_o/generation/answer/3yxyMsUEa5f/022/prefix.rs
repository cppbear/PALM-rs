// Answer 0

#[test]
fn test_cross_product_success() {
    let mut self_literals = Literals {
        lits: vec![
            Literal::Byte(1), Literal::Byte(2), Literal::Byte(3), 
            Literal::Byte(4), Literal::Byte(5), Literal::Byte(6), 
            Literal::Byte(7), Literal::Byte(8), Literal::Byte(9), 
            Literal::Byte(10)
        ],
        limit_size: 100,
        limit_class: 10,
    };
    
    let cross_literals = Literals {
        lits: vec![
            Literal::Byte(11), Literal::Byte(12), Literal::Byte(13), 
            Literal::Byte(14), Literal::Byte(15), Literal::Byte(16), 
            Literal::Byte(17), Literal::Byte(18), Literal::Byte(19), 
            Literal::Byte(20)
        ],
        limit_size: 100,
        limit_class: 10,
    };

    self_literals.set_limit_size(100);
    self_literals.set_limit_class(10);

    self_literals.cross_product(&cross_literals);
}

#[test]
fn test_cross_product_edge_case() {
    let mut self_literals = Literals {
        lits: vec![
            Literal::Byte(1), Literal::Byte(2), Literal::Byte(3), 
            Literal::Byte(4), Literal::Byte(5), 
        ],
        limit_size: 20,
        limit_class: 10,
    };

    let cross_literals = Literals {
        lits: vec![
            Literal::Byte(6), Literal::Byte(7), Literal::Byte(8), 
            Literal::Byte(9), Literal::Byte(10), 
            Literal::Byte(11), Literal::Byte(12), 
            Literal::Byte(13), Literal::Byte(14), Literal::Byte(15), 
        ],
        limit_size: 100,
        limit_class: 10,
    };

    self_literals.set_limit_size(20);
    self_literals.set_limit_class(10);

    assert!(self_literals.cross_product(&cross_literals));
}

#[test]
fn test_cross_product_with_max_size() {
    let mut self_literals = Literals {
        lits: vec![
            Literal::Byte(1), Literal::Byte(2), Literal::Byte(3), 
            Literal::Byte(4), Literal::Byte(5), 
            Literal::Byte(6), Literal::Byte(7), Literal::Byte(8), 
            Literal::Byte(9), Literal::Byte(10),
        ],
        limit_size: 50,
        limit_class: 10,
    };

    let cross_literals = Literals {
        lits: vec![
            Literal::Byte(11), Literal::Byte(12), Literal::Byte(13), 
            Literal::Byte(14), 
        ],
        limit_size: 50,
        limit_class: 10,
    };

    self_literals.set_limit_size(50);
    self_literals.set_limit_class(10);
    
    self_literals.cross_product(&cross_literals);
}

