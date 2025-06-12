// Answer 0

#[test]
fn test_cross_add_empty_lits() {
    let limit_size = 5;
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size,
        limit_class: 0,
    };
    let bytes: &[u8] = &[1, 2, 3];
    literals.cross_add(bytes);
}

#[test]
fn test_cross_add_with_prefix_fit() {
    let limit_size = 5;
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size,
        limit_class: 0,
    };
    let bytes: &[u8] = &[1, 2, 3, 4];
    literals.cross_add(bytes);
}

#[test]
fn test_cross_add_limit_exceeds() {
    let limit_size = 3;
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size,
        limit_class: 0,
    };
    let bytes: &[u8] = &[1, 2, 3, 4, 5];
    literals.cross_add(bytes);
}

#[test]
fn test_cross_add_single_byte_fits() {
    let limit_size = 4;
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size,
        limit_class: 0,
    };
    let bytes: &[u8] = &[1];
    literals.cross_add(bytes);
}

#[test]
fn test_cross_add_exact_limit() {
    let limit_size = 3;
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size,
        limit_class: 0,
    };
    let bytes: &[u8] = &[1, 2];
    literals.cross_add(bytes);
}

