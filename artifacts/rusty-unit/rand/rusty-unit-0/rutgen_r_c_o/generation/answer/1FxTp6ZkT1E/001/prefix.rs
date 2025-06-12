// Answer 0

#[test]
fn test_next_u64_min_values() {
    let mut rng = Lcg64Xsh32 { state: 0, increment: 0 };
    rng.next_u64();
}

#[test]
fn test_next_u64_max_state() {
    let mut rng = Lcg64Xsh32 { state: u64::MAX, increment: 0 };
    rng.next_u64();
}

#[test]
fn test_next_u64_max_increment() {
    let mut rng = Lcg64Xsh32 { state: 0, increment: u64::MAX };
    rng.next_u64();
}

#[test]
fn test_next_u64_large_values() {
    let mut rng = Lcg64Xsh32 { state: u64::MAX - 1, increment: u64::MAX - 1 };
    rng.next_u64();
}

#[test]
fn test_next_u64_random_values() {
    let mut rng = Lcg64Xsh32 { state: 123456789, increment: 987654321 };
    rng.next_u64();
}

#[test]
fn test_next_u64_state_inc_same() {
    let mut rng = Lcg64Xsh32 { state: 42, increment: 42 };
    rng.next_u64();
}

#[test]
fn test_next_u64_state_inc_odd() {
    let mut rng = Lcg64Xsh32 { state: 33, increment: 17 };
    rng.next_u64();
}

#[test]
fn test_next_u64_state_inc_even() {
    let mut rng = Lcg64Xsh32 { state: 44, increment: 34 };
    rng.next_u64();
}

