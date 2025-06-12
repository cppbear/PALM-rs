// Answer 0

#[test]
fn test_write_final_leftovers_no_extra_input() {
    struct DummyEngine;
    impl Engine for DummyEngine {
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
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<(), ()> {
            Ok(())
        }
        fn config(&self) -> &Self::Config {
            &()
        }
        fn encode<T: AsRef<[u8]>>(&self, input: T) -> String {
            base64::encode(input.as_ref())
        }
        fn decode<T: AsRef<[u8]>>(&self, _input: T) -> Result<Vec<u8>, ()> {
            Ok(vec![])
        }
    }

    struct DummyWriter {
        written: Vec<u8>,
    }
    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    let engine = DummyEngine;
    let writer = DummyWriter { written: Vec::new() };
    let mut encoder = EncoderWriter {
        engine: &engine,
        delegate: Some(writer),
        extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
        extra_input_occupied_len: 0,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let result = encoder.write_final_leftovers();
    assert!(result.is_ok());
}

#[test]
fn test_write_final_leftovers_with_extra_input() {
    struct DummyEngine;
    impl Engine for DummyEngine {
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
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<(), ()> {
            Ok(())
        }
        fn config(&self) -> &Self::Config {
            &()
        }
        fn encode<T: AsRef<[u8]>>(&self, input: T) -> String {
            base64::encode(input.as_ref())
        }
        fn decode<T: AsRef<[u8]>>(&self, _input: T) -> Result<Vec<u8>, ()> {
            Ok(vec![])
        }
    }

    struct DummyWriter {
        written: Vec<u8>,
    }
    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    let engine = DummyEngine;
    let writer = DummyWriter { written: Vec::new() };
    let mut encoder = EncoderWriter {
        engine: &engine,
        delegate: Some(writer),
        extra_input: [1, 2, 3], // Example data
        extra_input_occupied_len: 3,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };
    
    let result = encoder.write_final_leftovers();
    assert!(result.is_ok());
}

