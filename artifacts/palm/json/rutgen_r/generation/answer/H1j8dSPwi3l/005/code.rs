// Answer 0

#[test]
fn test_parse_escape_backslash_r() {
    use std::io::{self, Read};

    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            TestReader { data, position: 0 }
        }
    }

    impl Read for TestReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            if self.position >= self.data.len() {
                return Ok(0);
            }
            let bytes_to_read = std::cmp::min(buf.len(), self.data.len() - self.position);
            buf[..bytes_to_read].copy_from_slice(&self.data[self.position..self.position + bytes_to_read]);
            self.position += bytes_to_read;
            Ok(bytes_to_read)
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestReader::new(vec![b'\\', b'r']);
    
    let result = parse_escape(&mut reader, true, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\r']);
}

#[test]
fn test_parse_escape_backslash_b() {
    use std::io::{self, Read};

    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            TestReader { data, position: 0 }
        }
    }

    impl Read for TestReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            if self.position >= self.data.len() {
                return Ok(0);
            }
            let bytes_to_read = std::cmp::min(buf.len(), self.data.len() - self.position);
            buf[..bytes_to_read].copy_from_slice(&self.data[self.position..self.position + bytes_to_read]);
            self.position += bytes_to_read;
            Ok(bytes_to_read)
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestReader::new(vec![b'\\', b'b']);
    
    let result = parse_escape(&mut reader, true, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\x08']);
}

