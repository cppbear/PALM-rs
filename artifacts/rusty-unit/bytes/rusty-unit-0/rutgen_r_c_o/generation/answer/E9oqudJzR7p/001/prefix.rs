// Answer 0

#[test]
fn test_original_capacity_from_repr_one() {
    let repr = 1;
    original_capacity_from_repr(repr);
}

#[test]
fn test_original_capacity_from_repr_two() {
    let repr = 2;
    original_capacity_from_repr(repr);
}

#[test]
fn test_original_capacity_from_repr_max() {
    let repr = 17;
    original_capacity_from_repr(repr);
}

#[test]
fn test_original_capacity_from_repr_edge_case() {
    let repr = 10;
    original_capacity_from_repr(repr);
}

