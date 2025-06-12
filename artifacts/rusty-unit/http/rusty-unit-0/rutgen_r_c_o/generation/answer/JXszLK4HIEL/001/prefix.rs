// Answer 0

#[test]
fn test_finish_zero() {
    let hasher = IdHasher(0);
    hasher.finish();
}

#[test]
fn test_finish_one() {
    let hasher = IdHasher(1);
    hasher.finish();
}

#[test]
fn test_finish_max() {
    let hasher = IdHasher(u64::MAX);
    hasher.finish();
}

#[test]
fn test_finish_mid() {
    let hasher = IdHasher(2_147_483_647);
    hasher.finish();
}

#[test]
fn test_finish_large_value() {
    let hasher = IdHasher(1_234_567_890);
    hasher.finish();
}

