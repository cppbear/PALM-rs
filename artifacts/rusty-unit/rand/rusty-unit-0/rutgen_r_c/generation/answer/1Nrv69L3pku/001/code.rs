// Answer 0

#[test]
fn test_alphanumeric_sample_with_valid_rng() {
    struct MockRng {
        next_u32_return: u32,
    }

    impl Rng for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.next_u32_return
        }
    }

    let mut rng = MockRng { next_u32_return: 0b001111 }; // Should correspond to valid index
    let alphanumeric = Alphanumeric;
    let result = alphanumeric.sample(&mut rng);
    assert!(result >= b'A' && result <= b'9', "Result should be a valid ASCII character from the charset.");
}

#[test]
fn test_alphanumeric_sample_with_rejection_sampling() {
    struct MockRng {
        calls: usize,
    }

    impl Rng for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.calls += 1;
            if self.calls == 1 {
                return 0b111111; // 63: invalid, should reject
            }
            0b000011; // 3: valid, should accept next call
        }
    }

    let mut rng = MockRng { calls: 0 };
    let alphanumeric = Alphanumeric;
    let result = alphanumeric.sample(&mut rng);
    assert!(result >= b'A' && result <= b'9', "Result should be a valid ASCII character from the charset.");
}

#[test]
fn test_alphanumeric_sample_high_values() {
    struct MockRng {
        value: u32,
    }

    impl Rng for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0b111110 }; // 62: valid index
    let alphanumeric = Alphanumeric;
    let result = alphanumeric.sample(&mut rng);
    assert_eq!(result, b'9', "Result should be '9' for index 62.");
}

#[test]
fn test_alphanumeric_sample_rejects_high_invalid() {
    struct MockRng {
        value: u32,
    }

    impl Rng for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0b111111 }; // 63: invalid index
    let alphanumeric = Alphanumeric;
    let result = alphanumeric.sample(&mut rng);
    assert!(result < b'A' || (result > b'Z' && result < b'a') || result > b'9', " Should be rejected for invalid range, forcing a loop iteration.");
}

