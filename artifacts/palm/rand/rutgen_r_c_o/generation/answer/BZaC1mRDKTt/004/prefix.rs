// Answer 0

#[test]
fn test_random_ratio_case1() {
    struct DummyRng;
    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 {
            0xFFFFFFFF
        }
    }

    let mut flipper = CoinFlipper::new(DummyRng);
    let n = 2;
    let d = 2;
    flipper.random_ratio(n, d);
}

#[test]
fn test_random_ratio_case2() {
    struct DummyRng;
    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 {
            0xFFFFFFFF
        }
    }

    let mut flipper = CoinFlipper::new(DummyRng);
    let n = 1;
    let d = 2;
    flipper.random_ratio(n, d);
}

#[test]
fn test_random_ratio_case3() {
    struct DummyRng;
    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 {
            0xFFFFFFFF
        }
    }

    let mut flipper = CoinFlipper::new(DummyRng);
    let n = 3;
    let d = 4;
    flipper.random_ratio(n, d);
}

#[test]
fn test_random_ratio_case4() {
    struct DummyRng;
    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 {
            0xFFFFFFFF
        }
    }

    let mut flipper = CoinFlipper::new(DummyRng);
    let n = 5;
    let d = 10;
    flipper.random_ratio(n, d);
}

