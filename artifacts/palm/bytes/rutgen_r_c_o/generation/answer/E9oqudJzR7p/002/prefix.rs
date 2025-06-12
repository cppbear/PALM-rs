// Answer 0

#[test]
fn test_original_capacity_from_repr_zero() {
    let repr = 0;
    original_capacity_from_repr(repr);
}

#[test]
fn test_original_capacity_from_repr_min() {
    let repr = 10;
    original_capacity_from_repr(repr);
}

#[test]
fn test_original_capacity_from_repr_min_plus_one() {
    let repr = 11;
    original_capacity_from_repr(repr);
}

#[test]
fn test_original_capacity_from_repr_max_minus_one() {
    let repr = 16;
    original_capacity_from_repr(repr);
}

#[test]
fn test_original_capacity_from_repr_max() {
    let repr = 17;
    original_capacity_from_repr(repr);
}

