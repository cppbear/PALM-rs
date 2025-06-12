// Answer 0

#[test]
fn test_from_seed_valid_input() {
    struct Pcg128Cm;
    
    impl Pcg128Cm {
        type Seed = [u8; 32]; // Assuming a 255-bit seed fits within 32 bytes
    
        fn from_seed(seed: Self::Seed) -> Self {
            let mut seed_u64 = [0u64; 4];
            le::read_u64_into(&seed, &mut seed_u64);
            let state = u128::from(seed_u64[0]) | (u128::from(seed_u64[1]) << 64);
            let incr = u128::from(seed_u64[2]) | (u128::from(seed_u64[3]) << 64);
            Self::from_state_incr(state, incr | 1)
        }
        
        fn from_state_incr(state: u128, incr: u128) -> Self {
            // Dummy implementation
            Pcg128Cm
        }
    }

    // Test with a valid 255-bit seed (32 bytes) where the last bit of seed[8] is ignored
    let seed = [
        0xFF, 0xFF, 0xFF, 0xFF, // First 25 bits (0-24)
        0xFF, 0xFF, 0xFF, 0xFF, // Next 25 bits (25-49)
        0xFF, 0x80, // Last 6 bits (50-55) - 255 bits total, last bit ignored
        0, 0, 0, 0, 0, 0, 0, 0 // Remaining bytes (56-255)
    ];
    
    let _ = Pcg128Cm::from_seed(seed);
}

#[test]
#[should_panic]
fn test_from_seed_invalid_panic() {
    struct Pcg128Cm;
    
    impl Pcg128Cm {
        type Seed = [u8; 32]; // Assuming 255-bit seed fits within 32 bytes
        
        fn from_seed(seed: Self::Seed) -> Self {
            let mut seed_u64 = [0u64; 4];
            le::read_u64_into(&seed, &mut seed_u64);
            let state = u128::from(seed_u64[0]) | (u128::from(seed_u64[1]) << 64);
            let incr = u128::from(seed_u64[2]) | (u128::from(seed_u64[3]) << 64);
            Self::from_state_incr(state, incr | 1)
        }
        
        fn from_state_incr(state: u128, incr: u128) -> Self {
            // Dummy implementation
            Pcg128Cm
        }
    }

    // Test with invalid input leading to panic (not respecting the byte size)
    let seed = [
        0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0x80 // Not a complete 32 bytes
    ];
    
    let _ = Pcg128Cm::from_seed(seed);
}

