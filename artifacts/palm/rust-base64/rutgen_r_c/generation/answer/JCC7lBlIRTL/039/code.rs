// Answer 0

#[test]
fn test_write_with_full_input() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let len = input.len().min(3); // Encode only available bytes
            output[..len].copy_from_slice(&input[..len]);
            len * 4 / 3 // Simulate base64 encoding
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len * 3 / 4 // Simulate decoded length estimation
        }

        fn internal_decode(&self, _: &[u8], _: &mut [u8], _: Self::DecodeEstimate) -> Result<(), ()> {
            Ok(())
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let input_data = [1, 2, 3];
    let mut output_buffer = [0; 1024];
    let delegate_writer = io::Cursor::new(Vec::new());
    let mut writer = EncoderWriter::new(delegate_writer, &engine);
    
    let result = writer.write(&input_data);

    assert_eq!(result.unwrap(), MIN_ENCODE_CHUNK_SIZE);
}

#[test]
fn test_write_partial_input() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let len = input.len().min(3); // Encode only available bytes
            output[..len].copy_from_slice(&input[..len]);
            len * 4 / 3 // Simulate base64 encoding
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len * 3 / 4 // Simulate decoded length estimation
        }

        fn internal_decode(&self, _: &[u8], _: &mut [u8], _: Self::DecodeEstimate) -> Result<(), ()> {
            Ok(())
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let input_data = [1, 2];
    let mut output_buffer = [0; 1024];
    let delegate_writer = io::Cursor::new(Vec::new());
    let mut writer = EncoderWriter::new(delegate_writer, &engine);
    
    // forcing the extra_input_occupied_len to 0
    writer.extra_input_occupied_len = 0;
    let result = writer.write(&input_data);

    assert_eq!(result.unwrap(), input_data.len());
}

