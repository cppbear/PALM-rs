// Answer 0

#[test]
fn test_gen_mod_u128_small_range() {
    let mut rng = Rng(0);
    let result = rng.gen_mod_u128(10);
    assert!(result < 10);
}

#[test]
fn test_gen_mod_u128_large_range() {
    let mut rng = Rng(0);
    let result = rng.gen_mod_u128(1_000_000);
    assert!(result < 1_000_000);
}

#[test]
fn test_gen_mod_u128_zero_range() {
    let mut rng = Rng(0);
    let result = rng.gen_mod_u128(0);
    assert_eq!(result, 0);
}

#[should_panic(expected = "empty range")]
fn test_gen_mod_u128_panic_empty_range() {
    let mut rng = Rng(0);
    rng.gen_mod_u128(u128::MAX);
}

