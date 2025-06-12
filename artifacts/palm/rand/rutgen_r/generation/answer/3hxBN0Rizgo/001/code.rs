// Answer 0

#[derive(Debug)]
struct Rng {
    seed: [u8; 4], // assuming a fixed size seed for the example
}

impl Rng {
    fn from_seed(seed: [u8; 4]) -> Self {
        Rng { seed }
    }
}

struct SmallRng(Rng);

impl SmallRng {
    const Seed: [u8; 4] = [0; 4]; // Just an example seed size

    fn from_seed(seed: Self::Seed) -> Self {
        const LEN: usize = core::mem::size_of::<Self::Seed>();
        let seed = (&seed[..LEN]).try_into().unwrap();
        SmallRng(Rng::from_seed(seed))
    }
}

#[test]
fn test_small_rng_from_seed_valid() {
    let seed: [u8; 4] = [1, 2, 3, 4]; // valid seed
    let rng = SmallRng::from_seed(seed);
    assert_eq!(rng.0.seed, [1, 2, 3, 4]);
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_small_rng_from_seed_panic_out_of_bounds() {
    let seed: [u8; 2] = [1, 2]; // seed is too small
    let rng = SmallRng::from_seed(seed);
}

#[should_panic(expected = "array try_into failed")]
#[test]
fn test_small_rng_from_seed_panic_try_into() {
    let seed: [u8; 5] = [1, 2, 3, 4, 5]; // seed is too large
    let rng = SmallRng::from_seed(seed);
}

