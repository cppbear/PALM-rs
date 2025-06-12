// Answer 0

#[test]
fn test_seed_from_u64_valid() {
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

    let rng = Xoshiro256PlusPlus::seed_from_u64(123456789);
    assert_ne!(rng.s, [0; 4]);
}

#[test]
#[should_panic]
fn test_seed_from_u64_zero_state() {
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

    let rng = Xoshiro256PlusPlus::seed_from_u64(0);
    assert_eq!(rng.s, [0; 4]);
}

