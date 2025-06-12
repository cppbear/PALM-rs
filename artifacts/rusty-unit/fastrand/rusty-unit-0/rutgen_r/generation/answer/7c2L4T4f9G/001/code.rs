// Answer 0

#[test]
fn test_random_bool_even() {
    struct MockRng;

    impl MockRng {
        fn u8(&mut self, range: std::ops::Range<u8>) -> u8 {
            0 // returning a value that simulates an even output
        }

        fn bool(&mut self) -> bool {
            self.u8(0..255) % 2 == 0
        }
    }

    let mut rng = MockRng;
    assert_eq!(rng.bool(), true);
}

#[test]
fn test_random_bool_odd() {
    struct MockRng;

    impl MockRng {
        fn u8(&mut self, range: std::ops::Range<u8>) -> u8 {
            1 // returning a value that simulates an odd output
        }

        fn bool(&mut self) -> bool {
            self.u8(0..255) % 2 == 0
        }
    }

    let mut rng = MockRng;
    assert_eq!(rng.bool(), false);
}

#[test]
#[should_panic]
fn test_random_bool_panic() {
    struct MockRng;

    impl MockRng {
        fn u8(&mut self, _range: std::ops::Range<u8>) -> u8 {
            panic!("Simulating panic in u8 method")
        }

        fn bool(&mut self) -> bool {
            self.u8(0..255) % 2 == 0
        }
    }

    let mut rng = MockRng;
    rng.bool();
}

