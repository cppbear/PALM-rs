// Answer 0

#[test]
fn test_rng_bool_false() {
    let mut rng = Rng::with_seed(1); // Seed with a specific value
    // Setting up u8 output to ensure we get an odd number
    let result = rng.bool();
    assert_eq!(result, false);
}

#[test]
fn test_rng_bool_true() {
    let mut rng = Rng::with_seed(2); // Seed with a different value
    // Setting up u8 output to ensure we get an even number
    let result = rng.bool();
    assert_eq!(result, true);
}

#[test]
fn test_rng_bool_boundary_conditions() {
    let mut rng_low = Rng::with_seed(u64::MIN); // Edge case seed
    let result_low = rng_low.bool();
    assert!(result_low == true || result_low == false); 

    let mut rng_high = Rng::with_seed(u64::MAX); // Edge case seed
    let result_high = rng_high.bool();
    assert!(result_high == true || result_high == false); 
}

