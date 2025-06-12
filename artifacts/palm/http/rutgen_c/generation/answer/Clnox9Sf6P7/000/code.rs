// Answer 0

#[test]
fn test_usable_capacity_with_large_value() {
    let cap: usize = 1024;
    let result = usable_capacity(cap);
    assert_eq!(result, 768);
}

#[test]
fn test_usable_capacity_with_small_value() {
    let cap: usize = 8;
    let result = usable_capacity(cap);
    assert_eq!(result, 6);
}

#[test]
fn test_usable_capacity_with_zero() {
    let cap: usize = 0;
    let result = usable_capacity(cap);
    assert_eq!(result, 0);
}

#[test]
fn test_usable_capacity_with_one() {
    let cap: usize = 1;
    let result = usable_capacity(cap);
    assert_eq!(result, 1);
}

#[test]
fn test_usable_capacity_with_three() {
    let cap: usize = 3;
    let result = usable_capacity(cap);
    assert_eq!(result, 2);
}

