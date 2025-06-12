// Answer 0

fn test_write_final_leftovers_success() -> Result<()> {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..input.len()].copy_from_slice(input);
            input.len()
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<u8, ()> {
            Ok(0)
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let input = b"ABC";
    let mut output = [0u8; BUF_SIZE];
    let delegate = Vec::new();
    
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(delegate),
        extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
        extra_input_occupied_len: input.len(),
        output,
        output_occupied_len: 0,
        panicked: false,
    };

    encoder_writer.extra_input[..input.len()].copy_from_slice(input);

    let result = encoder_writer.write_final_leftovers();
    
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn test_write_final_leftovers_no_delegate() -> Result<()> {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..input.len()].copy_from_slice(input);
            input.len()
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<u8, ()> {
            Ok(0)
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let delegate = Vec::new();
    
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: None,
        extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
        extra_input_occupied_len: 0,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let result = encoder_writer.write_final_leftovers();
    
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn test_write_final_leftovers_with_remaining_extra_input() -> Result<()> {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..input.len()].copy_from_slice(input);
            input.len()
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<u8, ()> {
            Ok(0)
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let input = b"XYZ";
    let mut output = [0u8; BUF_SIZE];
    let delegate = Vec::new(); // This will be our "writer".
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(delegate),
        extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
        extra_input_occupied_len: input.len(),
        output,
        output_occupied_len: 0,
        panicked: false,
    };

    encoder_writer.extra_input[..input.len()].copy_from_slice(input);

    let result = encoder_writer.write_final_leftovers();
    
    assert!(result.is_ok());
    Ok(())
}

