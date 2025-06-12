// Answer 0

#[derive(Debug)]
struct Rng(u64);

impl Rng {
    pub fn with_seed(seed: u64) -> Self {
        Rng(seed)
    }
}

#[test]
fn test_rng_with_seed_zero() {
    let rng = Rng::with_seed(0);
    assert_eq!(format!("{:?}", rng), "Rng(0)");
}

#[test]
fn test_rng_with_seed_small_value() {
    let rng = Rng::with_seed(1);
    assert_eq!(format!("{:?}", rng), "Rng(1)");
}

#[test]
fn test_rng_with_seed_large_value() {
    let rng = Rng::with_seed(18446744073709551615); // Maximum value for u64
    assert_eq!(format!("{:?}", rng), "Rng(18446744073709551615)");
}

#[should_panic]
fn test_rng_with_seed_negative_value() {
    // No negative values can be passed to u64
    // Testing with panicking assertions is not applicable here,
    // since the function's parameter type prevents this.
}

