// Answer 0

#[test]
fn test_full_with_minimum_value() {
    let hash = 0;
    full(hash);
}

#[test]
fn test_full_with_middle_value() {
    let hash = 2_u64.pow(63);
    full(hash);
}

#[test]
fn test_full_with_maximum_value() {
    let hash = u64::MAX;
    full(hash);
}

#[test]
fn test_full_with_random_value() {
    let hash = 123456789123456789;
    full(hash);
}

#[test]
fn test_full_with_another_random_value() {
    let hash = 987654321987654321;
    full(hash);
}

