// Answer 0

#[test]
fn test_encode_with_padding_short_input() {
    struct DummyEngine {
        config: Config,
    }

    impl Engine for DummyEngine {
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..input.len()].copy_from_slice(input);
            input.len()
        }

        fn config(&self) -> &Config {
            &self.config
        }
    }

    let engine = DummyEngine {
        config: Config::default().with_encode_padding(true),
    };
    let input = &[0u8, 1u8, 2u8];
    let mut output = [0u8; 4];
    let expected_encoded_size = 4;

    encode_with_padding(input, &mut output, &engine, expected_encoded_size);
}

#[test]
fn test_encode_with_padding_exact_padding() {
    struct DummyEngine {
        config: Config,
    }

    impl Engine for DummyEngine {
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..input.len()].copy_from_slice(input);
            input.len()
        }

        fn config(&self) -> &Config {
            &self.config
        }
    }

    let engine = DummyEngine {
        config: Config::default().with_encode_padding(true),
    };
    let input = &[0u8, 1u8, 2u8];
    let mut output = [0u8; 8];
    let expected_encoded_size = 8;

    encode_with_padding(input, &mut output, &engine, expected_encoded_size);
}

#[test]
fn test_encode_with_padding_no_padding() {
    struct DummyEngine {
        config: Config,
    }

    impl Engine for DummyEngine {
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..input.len()].copy_from_slice(input);
            input.len()
        }

        fn config(&self) -> &Config {
            &self.config
        }
    }

    let engine = DummyEngine {
        config: Config::default().with_encode_padding(false),
    };
    let input = &[0u8, 1u8, 2u8];
    let mut output = [0u8; 4];
    let expected_encoded_size = 4;

    encode_with_padding(input, &mut output, &engine, expected_encoded_size);
}

#[test]
fn test_encode_with_padding_maximum_padding() {
    struct DummyEngine {
        config: Config,
    }

    impl Engine for DummyEngine {
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..input.len()].copy_from_slice(input);
            input.len()
        }

        fn config(&self) -> &Config {
            &self.config
        }
    }

    let engine = DummyEngine {
        config: Config::default().with_encode_padding(true),
    };
    let input = &[0u8, 1u8, 2u8, 3u8];
    let mut output = [0u8; 8];
    let expected_encoded_size = 8;

    encode_with_padding(input, &mut output, &engine, expected_encoded_size);
}

