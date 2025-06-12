// Answer 0

#[test]
fn test_sample_upper_bound() {
    struct MockRng {
        value: u8,
    }
    
    impl Rng for MockRng {
        fn random_range(&mut self, range: core::ops::Range<u8>) -> u8 {
            self.value = range.start + 25; // Choose maximum value that should return 'Z'
            self.value
        }
    }

    let mut rng = MockRng { value: 0 };
    let alphabetic = Alphabetic;
    let result = alphabetic.sample(&mut rng);
    assert_eq!(result, b'Z'); // Expecting 'Z' which is the highest upper-case letter
}

#[test]
fn test_sample_lower_bound() {
    struct MockRng {
        value: u8,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: core::ops::Range<u8>) -> u8 {
            self.value = range.start + 0; // Choose minimum value that should return 'A'
            self.value
        }
    }

    let mut rng = MockRng { value: 0 };
    let alphabetic = Alphabetic;
    let result = alphabetic.sample(&mut rng);
    assert_eq!(result, b'A'); // Expecting 'A' which is the lowest upper-case letter
}

#[test]
fn test_sample_midpoint() {
    struct MockRng {
        value: u8,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: core::ops::Range<u8>) -> u8 {
            self.value = range.start + 13; // Choosing a midpoint value that falls after 'Z'
            self.value
        }
    }

    let mut rng = MockRng { value: 0 };
    let alphabetic = Alphabetic;
    let result = alphabetic.sample(&mut rng);
    assert_eq!(result, b'n'); // Expecting 'n' which is 13 letters after 'A'
}

#[test]
fn test_sample_edge_case_after_z() {
    struct MockRng {
        value: u8,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: core::ops::Range<u8>) -> u8 {
            self.value = range.start + 26; // Choosing a value that directly leads to lower-case letters
            self.value
        }
    }

    let mut rng = MockRng { value: 0 };
    let alphabetic = Alphabetic;
    let result = alphabetic.sample(&mut rng);
    assert_eq!(result, b'a'); // Expecting 'a' because it corresponds to the first lower-case letter
}

