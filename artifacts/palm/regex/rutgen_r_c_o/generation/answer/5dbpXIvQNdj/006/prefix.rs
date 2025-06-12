// Answer 0

#[test]
fn test_cross_add_normal_case() {
    let mut literals = Literals::empty();
    literals.set_limit_size(10);
    literals.cross_add(&[1, 2, 3, 4, 5]);
}

#[test]
fn test_cross_add_with_cut() {
    let mut literals = Literals::empty();
    literals.set_limit_size(10);
    literals.cross_add(&[1, 2]);
    literals.cross_add(&[3, 4, 5]);
}

#[test]
fn test_cross_add_at_limit() {
    let mut literals = Literals::empty();
    literals.set_limit_size(5);
    literals.cross_add(&[1, 2]);
    literals.cross_add(&[3, 4]);
}

#[test]
fn test_cross_add_large_bytes() {
    let mut literals = Literals::empty();
    literals.set_limit_size(15);
    literals.cross_add(&[1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_cross_add_small_limit() {
    let mut literals = Literals::empty();
    literals.set_limit_size(3);
    literals.cross_add(&[1, 2]);
    literals.cross_add(&[3]);
}

#[test]
fn test_cross_add_exceed_limit() {
    let mut literals = Literals::empty();
    literals.set_limit_size(5);
    literals.cross_add(&[1, 2]);
    let result = literals.cross_add(&[3, 4, 5, 6]);
}

