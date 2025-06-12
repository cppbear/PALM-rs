// Answer 0

#[test]
fn test_write_all_encoded_output_interrupted_error() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0
        }
        
        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<(), io::Error> {
            Ok(())
        }
        
        fn config(&self) -> &Self::Config {
            &()
        }
    }

    struct MockWriter {
        will_interrupt: bool,
        written: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.will_interrupt {
                self.will_interrupt = false;
                return Err(io::Error::new(ErrorKind::Interrupted, "Interrupted"));
            }
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let engine = TestEngine;
    let output_buffer: [u8; BUF_SIZE] = [0; BUF_SIZE];
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(MockWriter { will_interrupt: true, written: Vec::new() }),
        extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
        extra_input_occupied_len: 0,
        output: output_buffer,
        output_occupied_len: 3, // Set to a number > 0 to test the loop
        panicked: false,
    };

    let result = encoder_writer.write_all_encoded_output();
    assert!(result.is_ok());
    assert_eq!(encoder_writer.output_occupied_len, 0);
}

#[test]
fn test_write_all_encoded_output_other_error() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0
        }
        
        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<(), io::Error> {
            Ok(())
        }
        
        fn config(&self) -> &Self::Config {
            &()
        }
    }

    struct MockWriter {
        error_on_write: bool,
        written: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            if self.error_on_write {
                return Err(io::Error::new(ErrorKind::Other, "Some other error"));
            }
            Ok(0) // Simulate no data written
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let engine = TestEngine;
    let output_buffer: [u8; BUF_SIZE] = [0; BUF_SIZE];
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(MockWriter { error_on_write: true, written: Vec::new() }),
        extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
        extra_input_occupied_len: 0,
        output: output_buffer,
        output_occupied_len: 3, // Set to a number > 0 to test the loop
        panicked: false,
    };

    let result = encoder_writer.write_all_encoded_output();
    assert!(result.is_err());
    assert_eq!(encoder_writer.output_occupied_len, 3); // Output length should not change
}

#[test]
fn test_write_all_encoded_output_success() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0
        }
        
        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<(), io::Error> {
            Ok(())
        }
        
        fn config(&self) -> &Self::Config {
            &()
        }
    }

    struct MockWriter {
        written: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let engine = TestEngine;
    let output_buffer: [u8; BUF_SIZE] = [0; BUF_SIZE];
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(MockWriter { written: Vec::new() }),
        extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
        extra_input_occupied_len: 0,
        output: output_buffer,
        output_occupied_len: 3, // Pre-filled output to test successful completion
        panicked: false,
    };

    let result = encoder_writer.write_all_encoded_output();
    assert!(result.is_ok());
    assert_eq!(encoder_writer.output_occupied_len, 0); // Output length should now be 0
}

