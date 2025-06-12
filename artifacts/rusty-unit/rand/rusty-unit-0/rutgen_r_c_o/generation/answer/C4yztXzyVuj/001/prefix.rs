// Answer 0

#[test]
fn test_random_ratio_one_over_minimum() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 1 }
        fn next_u64(&mut self) -> u64 { 1 }
    }

    let mut rng = TestRng;
    let mut coin_flipper = CoinFlipper::new(rng);
    let result = coin_flipper.random_ratio_one_over(1);
}

#[test]
fn test_random_ratio_one_over_small() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 2 }
        fn next_u64(&mut self) -> u64 { 2 }
    }

    let mut rng = TestRng;
    let mut coin_flipper = CoinFlipper::new(rng);
    let result = coin_flipper.random_ratio_one_over(2);
}

#[test]
fn test_random_ratio_one_over_large() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 4 }
        fn next_u64(&mut self) -> u64 { 4 }
    }

    let mut rng = TestRng;
    let mut coin_flipper = CoinFlipper::new(rng);
    let result = coin_flipper.random_ratio_one_over(2147483647);
}

#[test]
#[should_panic]
fn test_random_ratio_one_over_zero() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
    }

    let mut rng = TestRng;
    let mut coin_flipper = CoinFlipper::new(rng);
    let _ = coin_flipper.random_ratio_one_over(0);
} 

#[test]
fn test_random_ratio_one_over_maximum() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { u32::MAX }
        fn next_u64(&mut self) -> u64 { u64::MAX }
    }

    let mut rng = TestRng;
    let mut coin_flipper = CoinFlipper::new(rng);
    let result = coin_flipper.random_ratio_one_over(4294967295);
}

