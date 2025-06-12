// Answer 0

#[test]
fn test_fill_bytes() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            42
        }

        fn next_u64(&mut self) -> u64 {
            84
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = 1; // Fill with 1s for simplicity
            }
        }
    }

    impl Rng for StdRng {
        fn random<T>(&mut self) -> T {
            unimplemented!()
        }

        fn random_range<T, R>(&mut self, _range: R) -> T {
            unimplemented!()
        }

        fn random_bool(&mut self, _p: f64) -> bool {
            unimplemented!()
        }

        fn random_ratio(&mut self, _numerator: u32, _denominator: u32) -> bool {
            unimplemented!()
        }

        fn sample<T, D: Distribution<T>>(&mut self, _distr: D) -> T {
            unimplemented!()
        }
    }

    let mut rng = StdRng(TestRng);
    let mut buffer = [0u8; 10];
    rng.fill(&mut buffer);
    assert_eq!(buffer, [1; 10]);
}

