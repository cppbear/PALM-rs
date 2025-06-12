// Answer 0

#[test]
fn test_decoder_read_with_valid_base64() -> Result<(), std::io::Error> {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl std::io::Read for MockReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let remaining = &self.data[self.position..];
            let bytes_to_read = std::cmp::min(buf.len(), remaining.len());
            buf[..bytes_to_read].copy_from_slice(&remaining[..bytes_to_read]);
            self.position += bytes_to_read;
            Ok(bytes_to_read)
        }
    }

    // Setup the decoder with valid base64 input
    let mut decoder = Decoder::new(MockReader::new(b"SGVsbG8sIFdvcmxkIQ==".to_vec()));

    let mut buf = [0u8; 2]; // Expecting less than DECODED_CHUNK_SIZE
    let bytes_read = decoder.read(&mut buf)?;

    assert_eq!(bytes_read, 2);
    assert_eq!(&buf[..bytes_read], b"Hello, "); // Check the decoded content
    Ok(())
}

#[test]
fn test_decoder_read_with_empty_buf() -> Result<(), std::io::Error> {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl std::io::Read for MockReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            Ok(0) // Simulate EOF
        }
    }

    let mut decoder = Decoder::new(MockReader::new(b"".to_vec()));
    let mut buf = [0u8; 0]; // Empty buffer

    let bytes_read = decoder.read(&mut buf)?;

    assert_eq!(bytes_read, 0); // No bytes read
    Ok(())
}

#[test]
fn test_decoder_read_with_large_buf_and_incomplete_chunk() -> Result<(), std::io::Error> {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl std::io::Read for MockReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let remaining = &self.data[self.position..];
            let bytes_to_read = std::cmp::min(buf.len(), remaining.len());
            buf[..bytes_to_read].copy_from_slice(&remaining[..bytes_to_read]);
            self.position += bytes_to_read;
            Ok(bytes_to_read)
        }
    }

    let mut decoder = Decoder::new(MockReader::new(b"UmFuZG9tIA==".to_vec())); // "Random "
    
    let mut buf = [0u8; 5]; // Buffer smaller than DECODED_CHUNK_SIZE
    let bytes_read = decoder.read(&mut buf)?;

    assert_eq!(bytes_read, 5);
    assert_eq!(&buf[..bytes_read], b"Random"); // Check if the decoded data is correct
    Ok(())
}

