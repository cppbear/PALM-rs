// Answer 0

#[test]
fn test_next_u64_initial_state_zero() {
    let mut rng = Mcg128Xsl64 { state: 0 };
    let _ = rng.next_u64();
}

#[test]
fn test_next_u64_initial_state_one() {
    let mut rng = Mcg128Xsl64 { state: 1 };
    let _ = rng.next_u64();
}

#[test]
fn test_next_u64_initial_state_max() {
    let mut rng = Mcg128Xsl64 { state: u128::MAX };
    let _ = rng.next_u64();
}

#[test]
fn test_next_u64_initial_state_mid() {
    let mut rng = Mcg128Xsl64 { state: u128::MAX / 2 };
    let _ = rng.next_u64();
}

#[test]
fn test_next_u64_initial_state_random() {
    let mut rng = Mcg128Xsl64 { state: 123456789012345678901234567890123456 };
    let _ = rng.next_u64();
}

