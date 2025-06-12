// Answer 0

#[test]
fn test_usable_capacity_full_capacity() {
    let cap = 100;
    let result = usable_capacity(cap);
    assert_eq!(result, 75);
}

#[test]
fn test_usable_capacity_zero_capacity() {
    let cap = 0;
    let result = usable_capacity(cap);
    assert_eq!(result, 0);
}

#[test]
fn test_usable_capacity_small_capacity() {
    let cap = 3;
    let result = usable_capacity(cap);
    assert_eq!(result, 2);
}

#[test]
fn test_usable_capacity_exact_divisible_capacity() {
    let cap = 8;
    let result = usable_capacity(cap);
    assert_eq!(result, 6);
}

#[test]
fn test_usable_capacity_large_capacity() {
    let cap = 1000;
    let result = usable_capacity(cap);
    assert_eq!(result, 750);
}

