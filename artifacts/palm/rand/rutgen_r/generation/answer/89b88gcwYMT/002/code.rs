// Answer 0

#[test]
fn test_advance_with_positive_delta() {
    struct Pcg64 {
        state: u64,
        increment: u64,
    }

    const MULTIPLIER: u64 = 6364136223846793005;

    let mut rng = Pcg64 {
        state: 1,
        increment: 0,
    };

    rng.advance(1); // Simple test with a positive delta
    assert_eq!(rng.state, 6364136223846793006); // Expected value after one advancement
}

#[test]
fn test_advance_with_large_even_delta() {
    struct Pcg64 {
        state: u64,
        increment: u64,
    }

    const MULTIPLIER: u64 = 6364136223846793005;

    let mut rng = Pcg64 {
        state: 1,
        increment: 0,
    };

    rng.advance(2); // Test with an even delta
    assert_eq!(rng.state, 6364136223846793006.wrapping_mul(6364136223846793006).wrapping_add(6364136223846793005)); // Expected value
}

#[test]
fn test_advance_with_zero_delta() {
    struct Pcg64 {
        state: u64,
        increment: u64,
    }

    const MULTIPLIER: u64 = 6364136223846793005;

    let mut rng = Pcg64 {
        state: 1,
        increment: 0,
    };

    rng.advance(0); // Delta is zero, state should not change
    assert_eq!(rng.state, 1); // Expected state remains the same
}

