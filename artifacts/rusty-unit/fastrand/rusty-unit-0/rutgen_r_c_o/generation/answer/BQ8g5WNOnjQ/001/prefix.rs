// Answer 0

#[test]
fn test_seed_zero() {
    seed(0);
}

#[test]
fn test_seed_max_u64() {
    seed(18446744073709551615);
}

#[test]
fn test_seed_small_value() {
    seed(1);
}

#[test]
fn test_seed_large_value() {
    seed(123456789012345678);
}

#[test]
#[should_panic]
fn test_seed_negative_value() {
    seed(u64::MAX + 1);
}

