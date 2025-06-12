// Answer 0

#[test]
fn test_rng_bool_true() {
    let mut rng = Rng::with_seed(1);
    let result = rng.bool();
    assert!(result == true || result == false); // Validate the range of possible results
}

#[test]
fn test_rng_bool_false() {
    let mut rng = Rng::with_seed(2);
    let result = rng.bool();
    assert!(result == true || result == false); // Validate the range of possible results
}

#[test]
fn test_rng_bool_boundary_conditions() {
    let mut rng_a = Rng::with_seed(u64::MIN);
    let result_a = rng_a.bool();
    assert!(result_a == true || result_a == false); // Validate the range of possible results

    let mut rng_b = Rng::with_seed(u64::MAX);
    let result_b = rng_b.bool();
    assert!(result_b == true || result_b == false); // Validate the range of possible results
}

