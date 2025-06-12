// Answer 0

#[test]
fn test_write_u64_min_value() {
    let mut hasher = IdHasher::default();
    hasher.write_u64(0);
}

#[test]
fn test_write_u64_mid_value() {
    let mut hasher = IdHasher::default();
    hasher.write_u64(u64::MAX / 2);
}

#[test]
fn test_write_u64_max_value() {
    let mut hasher = IdHasher::default();
    hasher.write_u64(u64::MAX);
}

#[test]
fn test_write_u64_large_value() {
    let mut hasher = IdHasher::default();
    hasher.write_u64(12345678901234567890);
}

