// Answer 0

#[test]
fn test_sample_alphabetic() {
    struct MockRng {
        value: u8,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::Range<u8>) -> u8 {
            self.value = range.start + (self.value % (range.end - range.start));
            self.value
        }
    }

    let mut rng = MockRng { value: 0 };
    let alphabetic = Alphabetic;

    // Test generating upper case letter 'A' (Offset 0)
    let result_a = alphabetic.sample(&mut rng);
    assert_eq!(result_a, b'A');

    // Change the rng value to get upper 'Z' (Offset 25)
    rng.value = 25;
    let result_z = alphabetic.sample(&mut rng);
    assert_eq!(result_z, b'Z');

    // Change the rng value to get lower case letter 'a' (Offset 26)
    rng.value = 26;
    let result_a_lower = alphabetic.sample(&mut rng);
    assert_eq!(result_a_lower, b'a');

    // Change the rng value to get lower 'z' (Offset 51)
    rng.value = 51;
    let result_z_lower = alphabetic.sample(&mut rng);
    assert_eq!(result_z_lower, b'z');
}

