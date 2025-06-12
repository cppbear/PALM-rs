// Answer 0

#[test]
fn test_cross_add_empty_bytes() {
    let mut literals = Literals::empty();
    literals.cross_add(&[]);
}

#[test]
fn test_cross_add_empty_bytes_after_initialization() {
    let mut literals = Literals::empty();
    literals.set_limit_size(5);
    literals.cross_add(&[]);
}

