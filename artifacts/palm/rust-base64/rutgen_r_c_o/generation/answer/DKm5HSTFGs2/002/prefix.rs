// Answer 0

#[test]
fn test_finish_with_valid_writer() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            // Mock encoding logic for testing
            let len = input.len().min(output.len());
            output[..len].copy_from_slice(&input[..len]);
            len
        }
        
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }
        
        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata::default())
        }
        
        fn config(&self) -> &Self::Config {
            &()
        }
        
        fn encode<T: AsRef<[u8]>>(&self, input: T) -> String {
            input.as_ref().to_vec().into_iter().map(|byte| byte as char).collect()
        }
        
        fn encode_slice<T: AsRef<[u8]>>(
            &self,
            input: T,
            output_buf: &mut [u8],
        ) -> Result<usize, EncodeSliceError> {
            Ok(self.internal_encode(input.as_ref(), output_buf))
        }
    }
    
    use std::io::Cursor;

    let writer = Cursor::new(Vec::new());
    let engine = TestEngine;
    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    encoder_writer.extra_input_occupied_len = 0;
    encoder_writer.output_occupied_len = 0;
    
    let result = encoder_writer.finish();
}

#[test]
fn test_finish_with_partial_input() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let len = input.len().min(output.len());
            output[..len].copy_from_slice(&input[..len]);
            len
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata::default())
        }

        fn config(&self) -> &Self::Config {
            &()
        }

        fn encode<T: AsRef<[u8]>>(&self, input: T) -> String {
            input.as_ref().to_vec().into_iter().map(|byte| byte as char).collect()
        }

        fn encode_slice<T: AsRef<[u8]>>(
            &self,
            input: T,
            output_buf: &mut [u8],
        ) -> Result<usize, EncodeSliceError> {
            Ok(self.internal_encode(input.as_ref(), output_buf))
        }
    }

    use std::io::Cursor;

    let writer = Cursor::new(Vec::new());
    let engine = TestEngine;
    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    encoder_writer.extra_input_occupied_len = 1; // Simulate 1 byte buffered input
    encoder_writer.output_occupied_len = 0;

    let result = encoder_writer.finish();
}

#[test]
#[should_panic(expected = "Encoder has already had finish() called")]
fn test_finish_when_already_finished() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let len = input.len().min(output.len());
            output[..len].copy_from_slice(&input[..len]);
            len
        }
        
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }
        
        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata::default())
        }
        
        fn config(&self) -> &Self::Config {
            &()
        }
        
        fn encode<T: AsRef<[u8]>>(&self, input: T) -> String {
            input.as_ref().to_vec().into_iter().map(|byte| byte as char).collect()
        }
        
        fn encode_slice<T: AsRef<[u8]>>(
            &self,
            input: T,
            output_buf: &mut [u8],
        ) -> Result<usize, EncodeSliceError> {
            Ok(self.internal_encode(input.as_ref(), output_buf))
        }
    }

    use std::io::Cursor;

    let writer = Cursor::new(Vec::new());
    let engine = TestEngine;
    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    
    encoder_writer.finish(); // First call, should succeed
    encoder_writer.finish(); // Second call, should panic
}

