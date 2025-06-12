// Answer 0

#[test]
fn test_internal_encode_no_fast_loop_with_rem_2() {
    struct TestEngine {
        encode_table: [u8; 64],
        config: GeneralPurposeConfig,
    }

    impl Engine for TestEngine {
        type Config = GeneralPurposeConfig;
        type DecodeEstimate = GeneralPurposeEstimate;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let mut input_index: usize = 0;
            const LOW_SIX_BITS_U8: u8 = 0x3F;
            let rem = input.len() % 3;
            let start_of_rem = input.len() - rem;
            let mut output_index = 0;

            while input_index < start_of_rem {
                let input_chunk = &input[input_index..(input_index + 3)];
                let output_chunk = &mut output[output_index..(output_index + 4)];

                output_chunk[0] = self.encode_table[(input_chunk[0] >> 2) as usize];
                output_chunk[1] = self.encode_table
                    [((input_chunk[0] << 4 | input_chunk[1] >> 4) & LOW_SIX_BITS_U8) as usize];
                output_chunk[2] = self.encode_table
                    [((input_chunk[1] << 2 | input_chunk[2] >> 6) & LOW_SIX_BITS_U8) as usize];
                output_chunk[3] = self.encode_table[(input_chunk[2] & LOW_SIX_BITS_U8) as usize];

                input_index += 3;
                output_index += 4;
            }

            if rem == 2 {
                output[output_index] = self.encode_table[(input[start_of_rem] >> 2) as usize];
                output[output_index + 1] =
                    self.encode_table[((input[start_of_rem] << 4 | input[start_of_rem + 1] >> 4) & LOW_SIX_BITS_U8) as usize];
                output[output_index + 2] =
                    self.encode_table[((input[start_of_rem + 1] << 2) & LOW_SIX_BITS_U8) as usize];
                output_index += 3;
            }

            output_index
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            unimplemented!()
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &self.config
        }
    }

    let test_engine = TestEngine {
        encode_table: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
        config: GeneralPurposeConfig {
            encode_padding: true,
            decode_allow_trailing_bits: false,
            decode_padding_mode: DecodePaddingMode::Indifferent,
        },
    };

    let input = b"ab"; // 2 bytes will trigger rem == 2 case
    let mut output = [0u8; 4]; // Allocating enough space for output

    let output_index = test_engine.internal_encode(input, &mut output);

    assert_eq!(output_index, 3);
    assert_eq!(&output[0..3], b"YWI");
}

#[test]
fn test_internal_encode_no_fast_loop_with_start_of_rem() {
    struct TestEngine {
        encode_table: [u8; 64],
        config: GeneralPurposeConfig,
    }

    impl Engine for TestEngine {
        type Config = GeneralPurposeConfig;
        type DecodeEstimate = GeneralPurposeEstimate;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let mut input_index: usize = 0;
            const LOW_SIX_BITS_U8: u8 = 0x3F;

            let rem = input.len() % 3;
            let start_of_rem = input.len() - rem;
            let mut output_index = 0;

            // Start processing once input_index >= start_of_rem
            input_index = start_of_rem;

            while input_index < start_of_rem {
                // This will not execute
                let input_chunk = &input[input_index..(input_index + 3)];
                let output_chunk = &mut output[output_index..(output_index + 4)];

                output_chunk[0] = self.encode_table[(input_chunk[0] >> 2) as usize];
                output_chunk[1] = self.encode_table
                    [((input_chunk[0] << 4 | input_chunk[1] >> 4) & LOW_SIX_BITS_U8) as usize];
                output_chunk[2] = self.encode_table
                    [((input_chunk[1] << 2 | input_chunk[2] >> 6) & LOW_SIX_BITS_U8) as usize];
                output_chunk[3] = self.encode_table[(input_chunk[2] & LOW_SIX_BITS_U8) as usize];

                input_index += 3;
                output_index += 4;
            }

            if rem == 2 {
                output[output_index] = self.encode_table[(input[start_of_rem] >> 2) as usize];
                output[output_index + 1] =
                    self.encode_table[((input[start_of_rem] << 4 | input[start_of_rem + 1] >> 4) & LOW_SIX_BITS_U8) as usize];
                output[output_index + 2] =
                    self.encode_table[((input[start_of_rem + 1] << 2) & LOW_SIX_BITS_U8) as usize];
                output_index += 3;
            }

            output_index
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            unimplemented!()
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &self.config
        }
    }

    let test_engine = TestEngine {
        encode_table: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
        config: GeneralPurposeConfig {
            encode_padding: true,
            decode_allow_trailing_bits: false,
            decode_padding_mode: DecodePaddingMode::Indifferent,
        },
    };

    let input = b"abc"; // 3 bytes for base case
    let mut output = [0u8; 4]; // Allocating enough space for output

    let output_index = test_engine.internal_encode(input, &mut output);

    // Since we have 3 original bytes, the output should have 4 encoded bytes.
    assert_eq!(output_index, 4);
    assert_eq!(&output, b"YWJj");
}

