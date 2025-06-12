// Answer 0

#[test]
fn test_next_u64_minimum_state_increment() {
    let mut rng = Lcg128CmDxsm64::new(0, 1);
    let _ = rng.next_u64();
}

#[test]
fn test_next_u64_maximum_state_increment() {
    let mut rng = Lcg128CmDxsm64::new(0, u128::MAX);
    let _ = rng.next_u64();
}

#[test]
fn test_next_u64_random_state_increment() {
    let mut rng = Lcg128CmDxsm64::new(12345678901234567890, 9876543210987654321);
    let _ = rng.next_u64();
}

#[test]
fn test_next_u64_large_state_small_increment() {
    let mut rng = Lcg128CmDxsm64::new(u128::MAX - 1, 1);
    let _ = rng.next_u64();
}

#[test]
fn test_next_u64_large_state_large_increment() {
    let mut rng = Lcg128CmDxsm64::new(u128::MAX - 100, u128::MAX - 10);
    let _ = rng.next_u64();
}

#[test]
fn test_next_u64_edge_case_state_increment() {
    let mut rng = Lcg128CmDxsm64::new(2u128.pow(127) - 1, 2u128.pow(127) - 1);
    let _ = rng.next_u64();
}

