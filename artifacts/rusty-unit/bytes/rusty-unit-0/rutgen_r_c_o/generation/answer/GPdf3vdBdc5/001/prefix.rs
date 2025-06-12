// Answer 0

#[test]
fn test_original_capacity_to_repr_zero() {
    original_capacity_to_repr(0);
}

#[test]
fn test_original_capacity_to_repr_min_capacity() {
    original_capacity_to_repr(10);
}

#[test]
fn test_original_capacity_to_repr_max_capacity() {
    original_capacity_to_repr(131071);
}

#[test]
fn test_original_capacity_to_repr_mid_capacity() {
    original_capacity_to_repr(65536);
}

#[test]
fn test_original_capacity_to_repr_near_max_capacity() {
    original_capacity_to_repr(131000);
}

#[test]
fn test_original_capacity_to_repr_small_increment() {
    original_capacity_to_repr(11);
}

#[test]
fn test_original_capacity_to_repr_large_increment() {
    original_capacity_to_repr(123456);
}

#[test]
fn test_original_capacity_to_repr_max_original_capacity() {
    original_capacity_to_repr(131070);
}

#[test]
fn test_original_capacity_to_repr_capacity_at_boundary() {
    original_capacity_to_repr(32768);
}

