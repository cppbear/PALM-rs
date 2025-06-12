// Answer 0

#[test]
fn test_usable_capacity_zero() {
    let cap = 0;
    usable_capacity(cap);
}

#[test]
fn test_usable_capacity_min() {
    let cap = 1;
    usable_capacity(cap);
}

#[test]
fn test_usable_capacity_small() {
    let cap = 4;
    usable_capacity(cap);
}

#[test]
fn test_usable_capacity_medium() {
    let cap = 100;
    usable_capacity(cap);
}

#[test]
fn test_usable_capacity_large() {
    let cap = 16384;
    usable_capacity(cap);
}

#[test]
fn test_usable_capacity_boundary() {
    let cap = 16383;
    usable_capacity(cap);
}

