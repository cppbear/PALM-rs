// Answer 0

#[test]
fn test_encode_with_padding_no_padding() {
    struct TestEngine {
        config: Config,
    }

    impl Engine for TestEngine {
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            // Mock encoding: just copy the input over for testing
            output[..input.len()].copy_from_slice(input);
            input.len()
        }

        fn config(&self) -> &Config {
            &self.config
        }
    }

    let input = b"hello";
    let expected_size = 5; // Length of "hello"
    let mut output = vec![0u8; expected_size];
    let engine = TestEngine { config: Config::new(false) }; // No padding

    encode_with_padding(input, &mut output, &engine, expected_size);
    
    assert_eq!(output, input);
}

#[test]
fn test_encode_with_padding_with_padding() {
    struct TestEngine {
        config: Config,
    }

    impl Engine for TestEngine {
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            // Mock encoding: just copy input over 
            output[..input.len()].copy_from_slice(input);
            input.len()
        }

        fn config(&self) -> &Config {
            &self.config
        }
    }

    let input = b"hi"; // Length is 2, needs padding
    let expected_size = 4; // "hi" would need 2 padding bytes to make 4
    let mut output = vec![0u8; expected_size];
    let engine = TestEngine { config: Config::new(true) }; // With padding

    encode_with_padding(input, &mut output, &engine, expected_size);
    
    assert_eq!(&output[..2], input);
    assert_eq!(&output[2..], &[PAD_BYTE, PAD_BYTE]);
}

#[test]
fn test_encode_with_padding_overflow() {
    struct TestEngine {
        config: Config,
    }

    impl Engine for TestEngine {
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..input.len()].copy_from_slice(input);
            input.len()
        }

        fn config(&self) -> &Config {
            &self.config
        }
    }

    let input = b"";
    let expected_size = 0; // No input, so no output
    let mut output = vec![0u8; expected_size];
    let engine = TestEngine { config: Config::new(false) }; // No padding

    encode_with_padding(input, &mut output, &engine, expected_size);
    
    assert!(output.is_empty());
}

#[test]
#[should_panic(expected = "usize overflow when calculating b64 length")]
fn test_encode_with_padding_panic_on_overflow() {
    struct TestEngine {
        config: Config,
    }

    impl Engine for TestEngine {
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            usize::MAX // Force overflow
        }

        fn config(&self) -> &Config {
            &self.config
        }
    }

    let input = b"test";
    let expected_size = 0; // Invalid size
    let mut output = vec![0u8; 0];
    let engine = TestEngine { config: Config::new(true) }; // With padding

    encode_with_padding(input, &mut output, &engine, expected_size);
}

