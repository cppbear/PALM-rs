// Answer 0

#[test]
fn test_next_u32_min_state_increment() {
    let mut rng = Lcg128CmDxsm64 { state: 0, increment: 0 };
    let result = rng.next_u32();
}

#[test]
fn test_next_u32_min_state_max_increment() {
    let mut rng = Lcg128CmDxsm64 { state: 0, increment: u128::MAX };
    let result = rng.next_u32();
}

#[test]
fn test_next_u32_max_state_min_increment() {
    let mut rng = Lcg128CmDxsm64 { state: u128::MAX, increment: 0 };
    let result = rng.next_u32();
}

#[test]
fn test_next_u32_max_state_max_increment() {
    let mut rng = Lcg128CmDxsm64 { state: u128::MAX, increment: u128::MAX };
    let result = rng.next_u32();
}

#[test]
fn test_next_u32_mid_state_increment() {
    let mut rng = Lcg128CmDxsm64 { state: 1 << 63, increment: 1 << 63 };
    let result = rng.next_u32();
}

#[test]
fn test_next_u32_random_state_increment() {
    let mut rng = Lcg128CmDxsm64 { state: 1234567891011121314, increment: 9876543210123456789 };
    let result = rng.next_u32();
}

