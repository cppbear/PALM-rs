// Answer 0

#[test]
fn test_small_rng_from_seed() {
    use rand_core::{SeedableRng, RngCore};
    use rand::rngs::SmallRng;

    // Test with a valid seed
    let seed: [u8; 32] = [1; 32]; // Example seed
    let rng = SmallRng::from_seed(seed);
    assert_eq!(rng.0, Rng::from_seed(seed[..core::mem::size_of::<<Rng as SeedableRng>::Seed>()].try_into().unwrap()));
}

#[test]
fn test_small_rng_from_seed_zero() {
    use rand_core::{SeedableRng, RngCore};
    use rand::rngs::SmallRng;

    // Test with a seed of all zeros
    let seed: [u8; 32] = [0; 32];
    let rng = SmallRng::from_seed(seed);
    assert_eq!(rng.0, Rng::seed_from_u64(0));
}

#[should_panic]
fn test_small_rng_from_seed_invalid_length() {
    use rand_core::{SeedableRng, RngCore};
    use rand::rngs::SmallRng;

    // Test with an invalid seed length (this will panic)
    let seed: [u8; 31] = [1; 31]; // Invalid: not 32 bytes
    SmallRng::from_seed(seed);
}

