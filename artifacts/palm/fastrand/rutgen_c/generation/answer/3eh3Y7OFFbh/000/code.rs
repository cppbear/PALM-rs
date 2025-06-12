// Answer 0

#[test]
fn test_gen_u128() {
    let mut rng = Rng(42);
    let result = rng.gen_u128();
    let expected_high = (rng.gen_u64() << 64) | rng.gen_u64();
    assert_eq!(result, expected_high);
}

#[test]
fn test_gen_u128_randomness() {
    let mut rng1 = Rng(1);
    let mut rng2 = Rng(1);
    let result1 = rng1.gen_u128();
    let result2 = rng2.gen_u128();
    assert_ne!(result1, result2);
}

#[test]
fn test_gen_u128_boundary_case() {
    let mut rng = Rng(u64::MAX);
    let result = rng.gen_u128();
    let high_part = (u128::from(u64::MAX) << 64) | u128::from(u64::MAX);
    assert_eq!(result, high_part);
}

