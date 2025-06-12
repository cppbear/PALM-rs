// Answer 0

#[test]
#[should_panic]
fn test_random_ratio_one_over_panic() {
    let dummy_rng = DummyRng {};
    let mut flipper = CoinFlipper::new(dummy_rng);
    flipper.random_ratio_one_over(0);
}

#[test]
fn test_random_ratio_one_over_false_case() {
    let dummy_rng = DummyRng {};
    let mut flipper = CoinFlipper::new(dummy_rng);
    let result = flipper.random_ratio_one_over(1);
}

struct DummyRng {}

impl RngCore for DummyRng {
    fn next_u32(&mut self) -> u32 {
        0 // It will ensure that flip_c_heads returns false
    }

    fn next_u64(&mut self) -> u64 {
        0
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for byte in dest {
            *byte = 0;
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

