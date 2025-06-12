// Answer 0

#[derive(Debug)]
struct Xoshiro256PlusPlus {
    s: [u64; 4],
}

impl Xoshiro256PlusPlus {
    fn seed_from_u64(mut state: u64) -> Self {
        const PHI: u64 = 0x9e3779b97f4a7c15;
        let mut s = [0; 4];
        for i in s.iter_mut() {
            state = state.wrapping_add(PHI);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            z = z ^ (z >> 31);
            *i = z;
        }
        debug_assert_ne!(s, [0; 4]);
        Xoshiro256PlusPlus { s }
    }
}

#[test]
fn test_seed_from_u64_non_zero_state() {
    let seed: u64 = 123456789; // arbitrary non-zero seed
    let rng = Xoshiro256PlusPlus::seed_from_u64(seed);
    assert_ne!(rng.s, [0; 4]); // state should not be all zeros
}

#[test]
fn test_seed_from_u64_different_state() {
    let seed1: u64 = 987654321; // different non-zero seed
    let seed2: u64 = 234567890; // another different non-zero seed
    let rng1 = Xoshiro256PlusPlus::seed_from_u64(seed1);
    let rng2 = Xoshiro256PlusPlus::seed_from_u64(seed2);
    assert_ne!(rng1.s, rng2.s); // different seeds should yield different states
}

#[test]
fn test_seed_from_u64_edge_cases() {
    let seeds = [u64::MAX, 0xFFFFFFFFFFFFFFFF]; // maximum possible seed values
    for &seed in &seeds {
        let rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        assert_ne!(rng.s, [0; 4]); // state should not be all zeros
    }
}

