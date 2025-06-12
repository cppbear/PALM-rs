// Answer 0

#[test]
fn test_from_seed() {
    struct Lcg64Xsh32 {
        state: u64,
        incr: u64,
    }

    impl Lcg64Xsh32 {
        pub fn from_state_incr(state: u64, incr: u64) -> Self {
            Lcg64Xsh32 { state, incr }
        }
    }

    struct Pcg64 {
        seed: [u8; 16],
    }

    impl Pcg64 {
        type Seed = [u8; 16];

        fn from_seed(seed: Self::Seed) -> Self {
            let mut seed_u64 = [0u64; 2];
            let mut cursor = std::io::Cursor::new(seed);
            let _ = cursor.read(&mut seed_u64).expect("Should read seed into u64 array");

            // The increment must be odd, hence we discard one bit:
            Lcg64Xsh32::from_state_incr(seed_u64[0], seed_u64[1] | 1);
            Pcg64 { seed }
        }
    }

    // Test with a normal seed
    let seed_normal: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let pcg_normal = Pcg64::from_seed(seed_normal);
    
    // Test with all zeros (edge case)
    let seed_zero: [u8; 16] = [0; 16];
    let pcg_zero = Pcg64::from_seed(seed_zero);
    
    // Test with maximum value (edge case)
    let seed_max: [u8; 16] = [255; 16];
    let pcg_max = Pcg64::from_seed(seed_max);

    // Test with a seed that has odd increment implicitly (using odd last byte)
    let seed_odd: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17];
    let pcg_odd = Pcg64::from_seed(seed_odd);
}

