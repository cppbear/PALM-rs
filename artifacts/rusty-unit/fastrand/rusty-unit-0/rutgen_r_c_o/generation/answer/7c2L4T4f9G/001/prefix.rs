// Answer 0

#[test]
fn test_rng_bool_zero() {
    let mut rng = Rng::with_seed(42);
    let result = rng.bool();
}

#[test]
fn test_rng_bool_one() {
    let mut rng = Rng::with_seed(1);
    let result = rng.bool();
}

#[test]
fn test_rng_bool_mid() {
    let mut rng = Rng::with_seed(128);
    let result = rng.bool();
}

#[test]
fn test_rng_bool_high() {
    let mut rng = Rng::with_seed(255);
    let result = rng.bool();
}

#[test]
fn test_rng_bool_another_seed() {
    let mut rng = Rng::with_seed(100);
    let result = rng.bool();
}

#[test]
fn test_rng_bool_low_boundary() {
    let mut rng = Rng::with_seed(0);
    let result = rng.bool();
}

#[test]
fn test_rng_bool_large_seed() {
    let mut rng = Rng::with_seed(999999);
    let result = rng.bool();
}

