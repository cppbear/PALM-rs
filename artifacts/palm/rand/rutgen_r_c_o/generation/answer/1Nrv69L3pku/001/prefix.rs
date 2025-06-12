// Answer 0

#[test]
fn test_sample_with_valid_rng() {
    struct TestRng {
        value: u32,
    }

    impl Rng for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = TestRng { value: 0 };
    let alphanumeric = Alphanumeric;
    alphanumeric.sample(&mut rng); // var will be 0, which is less than RANGE

    rng.value = 1;
    alphanumeric.sample(&mut rng); // var will be 1, which is less than RANGE

    rng.value = 61;
    alphanumeric.sample(&mut rng); // var will be 61, which is less than RANGE
}

#[test]
fn test_sample_with_edge_case() {
    struct EdgeRng {
        value: u32,
    }

    impl Rng for EdgeRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = EdgeRng { value: 62 };
    let alphanumeric = Alphanumeric;
    alphanumeric.sample(&mut rng); // var will be 62 (not returned since it's >= RANGE)

    rng.value = 63;
    alphanumeric.sample(&mut rng); // var will be 63 (not returned since it's >= RANGE)

    rng.value = 31;
    alphanumeric.sample(&mut rng); // var will be 31, which is less than RANGE
}

#[test]
#[should_panic]
fn test_sample_with_invalid_rng() {
    struct InvalidRng {
        value: u32,
    }

    impl Rng for InvalidRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = InvalidRng { value: 64 };
    let alphanumeric = Alphanumeric;
    alphanumeric.sample(&mut rng); // var will be 64 (not returned since it's >= RANGE)
}

