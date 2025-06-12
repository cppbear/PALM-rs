// Answer 0

#[test]
fn test_random_ratio_n_equals_d() {
    use crate::RngCore;

    struct MockRng {
        count: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.count += 1;
            if self.count % 2 == 0 {
                0b11111111_11111111_11111111_11111111 // All bits are 1s for simulating heads
            } else {
                0b00000000_00000000_00000000_00000000 // All bits are 0s for simulating tails
            }
        }
    }

    let mut rng = MockRng { count: 0 };
    let mut coin_flipper = CoinFlipper::new(rng);

    // Now we test with n == d
    let n = 5;
    let d = 5;
    assert_eq!(coin_flipper.random_ratio(n, d), true);
}

#[test]
fn test_random_ratio_n_equals_d_large() {
    use crate::RngCore;

    struct MockRng {
        count: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            0b11111111_11111111_11111111_11111111 // simulating all heads
        }
    }

    let mut rng = MockRng { count: 0 };
    let mut coin_flipper = CoinFlipper::new(rng);

    // Test with a large value where n == d
    let n = usize::MAX;
    let d = usize::MAX;
    assert_eq!(coin_flipper.random_ratio(n, d), true);
}

