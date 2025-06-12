// Answer 0

#[test]
fn test_usable_capacity_positive() {
    let cap = 100;
    let result = usable_capacity(cap);
    assert_eq!(result, 75);
}

#[test]
fn test_usable_capacity_zero() {
    let cap = 0;
    let result = usable_capacity(cap);
    assert_eq!(result, 0);
}

#[test]
fn test_usable_capacity_small() {
    let cap = 1;
    let result = usable_capacity(cap);
    assert_eq!(result, 0);
}

#[test]
fn test_usable_capacity_four() {
    let cap = 4;
    let result = usable_capacity(cap);
    assert_eq!(result, 3);
}

#[test]
fn test_usable_capacity_large() {
    let cap = 1000;
    let result = usable_capacity(cap);
    assert_eq!(result, 750);
}

#[test]
fn test_usable_capacity_edge_case() {
    let cap = usize::MAX;
    let result = usable_capacity(cap);
    assert_eq!(result, usize::MAX - usize::MAX / 4);
}

