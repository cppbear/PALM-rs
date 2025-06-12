// Answer 0

#[test]
fn test_from_seed() {
    struct Mcg128Xsl64 {
        state: u128,
    }

    impl Mcg128Xsl64 {
        fn new(state: u128) -> Self {
            Mcg128Xsl64 { state }
        }
    }

    mod le {
        pub fn read_u64_into(src: &[u8], dst: &mut [u64]) {
            for (i, chunk) in src.chunks(8).enumerate() {
                dst[i] = chunk.iter().enumerate().fold(0u64, |acc, (j, &b)| {
                    acc | ((b as u64) << (j * 8))
                });
            }
        }
    }

    fn from_seed(seed: [u8; 16]) -> Mcg128Xsl64 {
        let mut seed_u64 = [0u64; 2];
        le::read_u64_into(&seed, &mut seed_u64);
        let state = u128::from(seed_u64[0]) | (u128::from(seed_u64[1]) << 64);
        Mcg128Xsl64::new(state)
    }

    let seed: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let mcg = from_seed(seed);
    assert_eq!(mcg.state, u128::from(0x0706050403020100u64) | (u128::from(0x0F0E0D0C0B0A0908u64) << 64));
}

