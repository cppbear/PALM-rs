// Answer 0

#[test]
fn test_random_ratio_both_heads() {
    struct MockCoinFlipper;

    impl MockCoinFlipper {
        fn flip_c_heads(&self, _c: u32) -> bool {
            true // all flips return heads
        }
    }

    let mut flipper = MockCoinFlipper;
    assert_eq!(flipper.random_ratio(1, 2), true); // n < d, expect true after potential doubling
}

#[test]
fn test_random_ratio_n_equals_d() {
    struct MockCoinFlipper;

    impl MockCoinFlipper {
        fn flip_c_heads(&self, _c: u32) -> bool {
            true // force return heads so we can exit the loop
        }
    }

    let mut flipper = MockCoinFlipper;
    assert_eq!(flipper.random_ratio(2, 2), true); // n == d, expect true directly
}

#[test]
fn test_random_ratio_doubling_to_reach_true() {
    struct MockCoinFlipper;

    impl MockCoinFlipper {
        fn flip_c_heads(&self, _c: u32) -> bool {
            true // all flips return heads
        }
    }

    let mut flipper = MockCoinFlipper;
    assert_eq!(flipper.random_ratio(3, 8), true); // starting with n < d, expect true after doubling
}

#[test]
fn test_random_ratio_flips_to_false() {
    struct MockCoinFlipper;

    impl MockCoinFlipper {
        fn flip_c_heads(&self, _c: u32) -> bool {
            false // at least one flip returns tails
        }
    }

    let mut flipper = MockCoinFlipper;
    assert_eq!(flipper.random_ratio(1, 3), false); // n < d and flipping to tails
}

