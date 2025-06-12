// Answer 0


struct MockEngine {
    padding: bool,
}

impl Config for MockEngine {
    // Implement necessary methods for MockEngine
}

impl Engine for MockEngine {
    type Config = MockEngine;
    type DecodeEstimate = usize; // or an appropriate type

    fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
        // A simple mock implementation that just copies bytes
        let len = input.len().min(output.len());
        output[..len].copy_from_slice(&input[..len]);
        len
    }

    fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
        0 // Placeholder for the estimate
    }

    fn internal_decode(
        &self,
        _input: &[u8],
        _output: &mut [u8],
        _decode_estimate: Self::DecodeEstimate,
    ) -> Result<DecodeMetadata, DecodeSliceError> {
        unimplemented!()
    }

    fn config(&self) -> &Self::Config {
        &self
    }
}

impl MockEngine {
    fn encode_padding(&self) -> bool {
        self.padding
    }
}

struct MockSink {
    should_fail: bool,
}

impl Sink for MockSink {
    type Error = ();

    fn write_encoded_bytes(&mut self, _data: &[u8]) -> Result<(), Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            Ok(())
        }
    }
}

#[test]
fn test_encode_partial_chunk_without_padding() {
    let engine = MockEngine { padding: false };
    let encoder = ChunkedEncoder::new(&engine);
    let mut sink = MockSink { should_fail: false };
    let input_data = vec![1, 2, 3, 4, 5]; // Partial chunk

    let result = encoder.encode(&input_data, &mut sink);
    assert_eq!(result, Ok(()));
}

#[test]
#[should_panic]
fn test_encode_partial_chunk_panicking_sink() {
    let engine = MockEngine { padding: false };
    let encoder = ChunkedEncoder::new(&engine);
    let mut sink = MockSink { should_fail: true };
    let input_data = vec![1, 2, 3, 4, 5]; // Partial chunk

    let _ = encoder.encode(&input_data, &mut sink); // This should panic
}


