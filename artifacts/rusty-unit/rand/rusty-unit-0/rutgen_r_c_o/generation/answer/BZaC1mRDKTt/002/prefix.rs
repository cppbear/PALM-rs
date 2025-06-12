// Answer 0

#[test]
fn test_random_ratio_case_1() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 1 };
    let mut coin_flipper = CoinFlipper::new(rng);
    
    let n = 1;
    let d = 2;
    let _ = coin_flipper.random_ratio(n, d);
}

#[test]
fn test_random_ratio_case_2() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 2 };
    let mut coin_flipper = CoinFlipper::new(rng);
    
    let n = 1;
    let d = 3;
    let _ = coin_flipper.random_ratio(n, d);
}

#[test]
fn test_random_ratio_case_3() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 3 };
    let mut coin_flipper = CoinFlipper::new(rng);
    
    let n = 2;
    let d = 4;
    let _ = coin_flipper.random_ratio(n, d);
}

#[test]
fn test_random_ratio_case_4() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0 };
    let mut coin_flipper = CoinFlipper::new(rng);
    
    let n = 1;
    let d = 4;
    let _ = coin_flipper.random_ratio(n, d);
}

