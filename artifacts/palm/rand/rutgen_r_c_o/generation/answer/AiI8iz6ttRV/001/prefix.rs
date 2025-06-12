// Answer 0

#[test]
fn test_coin_flipper_with_standard_rng() {
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    let mut rng = StdRng::seed_from_u64(42);
    let _flipper = CoinFlipper::new(rng);
}

#[test]
fn test_coin_flipper_with_thread_rng() {
    use rand::rngs::ThreadRng;
    use rand::thread_rng;

    let rng: ThreadRng = thread_rng();
    let _flipper = CoinFlipper::new(rng);
}

#[test]
fn test_coin_flipper_with_mock_rng() {
    use rand::RngCore;
    
    struct MockRng {
        value: u8,
    }
    
    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value as u32
        }
        fn next_u64(&mut self) -> u64 {
            self.value as u64
        }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = self.value;
            }
        }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::error::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mock_rng = MockRng { value: 127 };
    let _flipper = CoinFlipper::new(mock_rng);
}

