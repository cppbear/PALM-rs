// Answer 0

#[test]
fn test_sample_lowercase() {
    struct TestRng {
        state: u8,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: core::ops::Range<u8>) -> u8 {
            let value = self.state % (range.end - range.start) + range.start;
            self.state += 1;
            value
        }
    }

    let mut rng = TestRng { state: 0 };
    let alphabetic = Alphabetic;
    let result = alphabetic.sample(&mut rng);
}

#[test]
fn test_sample_uppercase() {
    struct TestRng {
        state: u8,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: core::ops::Range<u8>) -> u8 {
            let value = self.state % (range.end - range.start) + range.start;
            self.state += 1;
            value
        }
    }

    let mut rng = TestRng { state: 26 };
    let alphabetic = Alphabetic;
    let result = alphabetic.sample(&mut rng);
}

#[test]
fn test_sample_boundary_case() {
    struct TestRng {
        state: u8,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: core::ops::Range<u8>) -> u8 {
            let value = self.state % (range.end - range.start) + range.start;
            self.state += 1;
            value
        }
    }

    let mut rng = TestRng { state: 51 };
    let alphabetic = Alphabetic;
    let result = alphabetic.sample(&mut rng);
}

#[test]
fn test_sample_out_of_order() {
    struct TestRng {
        state: u8,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: core::ops::Range<u8>) -> u8 {
            let value = self.state % (range.end - range.start) + range.start;
            self.state += 2; // skip numbers to create a more varied test
            value
        }
    }

    let mut rng = TestRng { state: 5 };
    let alphabetic = Alphabetic;
    let result = alphabetic.sample(&mut rng);
}

