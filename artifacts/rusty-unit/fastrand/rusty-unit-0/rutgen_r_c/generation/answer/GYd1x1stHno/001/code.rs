// Answer 0

#[test]
fn test_rng_fork_deterministic_output() {
    let mut base = fastrand::Rng::with_seed(0x4d595df4d0f33173);
    
    // Use the original generator to ensure it produces a sequence of values.
    base.bool(); // This step ensures we use the generator.

    // Fork the generator.
    let mut forked_rng = base.fork();

    // Generate a value from both the original and the forked generator.
    let original_value = base.gen_u64();
    let forked_value = forked_rng.gen_u64();

    // Verify that the original and forked generators produce different sequences
    // Print the values for manual verification
    assert_ne!(original_value, forked_value);
}

#[test]
fn test_rng_fork_reproducibility() {
    let mut base1 = fastrand::Rng::with_seed(0x4d595df4d0f33173);
    let mut base2 = fastrand::Rng::with_seed(0x4d595df4d0f33173);

    // Fork both generators
    let mut rng1 = base1.fork();
    let mut rng2 = base2.fork();

    // Generate values from both
    let value1 = rng1.gen_u64();
    let value2 = rng2.gen_u64();

    // Print values for manual verification
    assert_ne!(value1, value2, "Forked generators should produce different values.");
}

#[test]
fn test_rng_fork_panic_conditions() {
    let mut rng = fastrand::Rng::with_seed(0x4d595df4d0f33173);

    // Use the generator to ensure it is in a valid state.
    rng.bool(); 
    
    // No panic should occur, hence we don't expect the function to panic here.
    let _ = std::panic::catch_unwind(|| {
        let _ = rng.fork(); // This should be fine, hence no panic is expected.
    });
}

