// Answer 0

#[test]
fn test_random_ratio_equal_n_d() {
    struct CoinFlipper;

    impl CoinFlipper {
        fn flip_c_heads(&self, _c: u32) -> bool {
            true // Simulating heads
        }

        fn random_ratio(&mut self, mut n: usize, d: usize) -> bool {
            while n < d {
                let c = n.leading_zeros().saturating_sub(d.leading_zeros() + 1).clamp(1, 32);
                if self.flip_c_heads(c) {
                    n = n.saturating_mul(2_usize.pow(c));
                } else {
                    if c == 1 {
                        let next_n = n.wrapping_add(n).wrapping_sub(d);
                        if next_n == 0 || next_n > n { return false; }
                        n = next_n;
                    } else {
                        return false;
                    }
                }
            }
            true
        }
    }

    let mut flipper = CoinFlipper;
    assert!(flipper.random_ratio(5, 5)); // test where n == d
}

