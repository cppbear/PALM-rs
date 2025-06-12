// Answer 0

#[test]
fn test_seed_from_u64_zero() {
    let state = 0u64;
    let rng = SmallRng::seed_from_u64(state);
}

#[test]
fn test_seed_from_u64_one() {
    let state = 1u64;
    let rng = SmallRng::seed_from_u64(state);
}

#[test]
fn test_seed_from_u64_max() {
    let state = 18_446_744_073_709_551_615u64;
    let rng = SmallRng::seed_from_u64(state);
}

#[test]
fn test_seed_from_u64_random_middle() {
    let state = 9_223_372_036_854_775_807u64;
    let rng = SmallRng::seed_from_u64(state);
}

#[test]
fn test_seed_from_u64_large_prime() {
    let state = 3_999_999_999_999_999_999u64;
    let rng = SmallRng::seed_from_u64(state);
}

