// Answer 0

#[test]
fn test_read_with_non_empty_buffer() {
    struct DummyEngine;
    struct DummyReader {
        data: Vec<u8>,
        position: usize,
    }

    impl io::Read for DummyReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let available = &self.data[self.position..];
            let len = available.len().min(buf.len());
            buf[..len].copy_from_slice(&available[..len]);
            self.position += len;
            Ok(len)
        }
    }

    impl Engine for DummyEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            let decoded_len = input.len().min(output.len());
            output[..decoded_len].copy_from_slice(&input[..decoded_len]);
            Ok(DecodeMetadata { decoded_len })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let data = b"SGVsbG8gV29ybGQh"; // "Hello World!" in base64
    let reader = DummyReader { data: data.to_vec(), position: 0 };
    let engine = DummyEngine;
    let mut decoder_reader = DecoderReader::new(reader, &engine);
    let mut buffer = [0_u8; 4]; // size is less than DECODED_CHUNK_SIZE

    let result = decoder_reader.read(&mut buffer);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 3); // Expecting 3 bytes to be written
    assert_eq!(&buffer[..3], b"Hel"); // Expecting the first three bytes of "Hello"
}

#[test]
fn test_read_at_eof_with_partial_read() {
    struct DummyEngine;
    struct DummyReader {
        data: Vec<u8>,
        position: usize,
    }

    impl io::Read for DummyReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let available = &self.data[self.position..];
            let len = available.len().min(buf.len());
            buf[..len].copy_from_slice(&available[..len]);
            self.position += len;
            Ok(len)
        }
    }

    impl Engine for DummyEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            let decoded_len = input.len().min(output.len());
            output[..decoded_len].copy_from_slice(&input[..decoded_len]);
            Ok(DecodeMetadata { decoded_len })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let data = b"SGVsbG8g"; // "Hello " in base64 with padding
    let reader = DummyReader { data: data.to_vec(), position: 0 };
    let engine = DummyEngine;
    let mut decoder_reader = DecoderReader::new(reader, &engine);
    let mut buffer = [0_u8; 3]; // Exact number of bytes can be read based on base64 decode

    let result = decoder_reader.read(&mut buffer);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 3); // Expecting 3 bytes to be written
    assert_eq!(&buffer[..3], b"Hel"); // Expect "Hel"
}

