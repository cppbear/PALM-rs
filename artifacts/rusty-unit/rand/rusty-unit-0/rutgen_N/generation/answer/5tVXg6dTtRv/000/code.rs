// Answer 0

#[test]
fn test_rng_initialization() {
    use rand::{rngs::ThreadRng, prelude::*};

    // Initialize the ThreadRng using the rng function
    let mut rng: ThreadRng = rng();

    // Perform a basic random operation to check initialization
    let value: u32 = rng.gen();
    assert!(value >= 0);
}

#[test]
fn test_rng_random_bool() {
    use rand::{rngs::ThreadRng, prelude::*};

    // Initialize the ThreadRng
    let mut rng: ThreadRng = rng();

    // Test generating a random boolean
    let random_bool: bool = rng.gen();
    assert!(random_bool == true || random_bool == false);
}

#[test]
fn test_rng_random_range() {
    use rand::{rngs::ThreadRng, prelude::*};

    // Initialize the ThreadRng
    let mut rng: ThreadRng = rng();

    // Test generating a random number in a specific range
    let random_number: u32 = rng.gen_range(1..=6);
    assert!(random_number >= 1 && random_number <= 6);
}

