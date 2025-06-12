// Answer 0

#[derive(Debug)]
struct PcgState {
    state: u128,
    increment: u128,
}

impl PcgState {
    fn from_state_incr(state: u128, increment: u128) -> Self {
        Self { state, increment }
    }
}

#[repr(transparent)]
struct Seed([u8; 32]);

impl Seed {
    fn new(seed: [u8; 32]) -> Self {
        Self(seed)
    }
}

impl PcgState {
    fn from_seed(seed: Seed) -> Self {
        let mut seed_u64 = [0u64; 4];
        let bytes = &seed.0;

        for i in 0..4 {
            seed_u64[i] = u64::from_le_bytes([
                bytes[i * 8],
                bytes[i * 8 + 1],
                bytes[i * 8 + 2],
                bytes[i * 8 + 3],
                bytes[i * 8 + 4],
                bytes[i * 8 + 5],
                bytes[i * 8 + 6],
                bytes[i * 8 + 7],
            ]);
        }
        
        let state = u128::from(seed_u64[0]) | (u128::from(seed_u64[1]) << 64);
        let incr = u128::from(seed_u64[2]) | (u128::from(seed_u64[3]) << 64);
        
        Self::from_state_incr(state, incr | 1)
    }
}

#[test]
fn test_from_seed() {
    let seed = Seed::new([
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8,
        9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8, 16u8,
        17u8, 18u8, 19u8, 20u8, 21u8, 22u8, 23u8, 24u8,
    ]);
    let pcg = PcgState::from_seed(seed);
    assert_eq!(pcg.state, 0x0203040506070801);
    assert_eq!(pcg.increment, 0x090a0b0c0d0e0f01);
}

#[test]
fn test_from_seed_odd_increment() {
    let seed = Seed::new([
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
    ]);
    let pcg = PcgState::from_seed(seed);
    assert_eq!(pcg.increment & 1, 1); // Verify that increment is odd
}

