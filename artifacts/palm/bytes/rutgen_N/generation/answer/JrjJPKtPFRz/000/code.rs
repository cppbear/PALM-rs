// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use bytes::Buf;
    use bytes::BytesMut;

    struct TestReader {
        buf: BytesMut,
    }

    impl TestReader {
        fn new(data: &[u8]) -> Self {
            Self {
                buf: BytesMut::from(data),
            }
        }

        fn read(&mut self, dst: &mut [u8]) -> io::Result<usize> {
            let len = std::cmp::min(self.buf.remaining(), dst.len());
            Buf::copy_to_slice(&mut self.buf, &mut dst[0..len]);
            Ok(len)
        }
    }

    #[test]
    fn test_read_fits() {
        let mut reader = TestReader::new(&[1, 2, 3, 4, 5]);
        let mut buffer = [0u8; 5];
        let result = reader.read(&mut buffer);
        assert_eq!(result.unwrap(), 5);
        assert_eq!(buffer, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_read_partial() {
        let mut reader = TestReader::new(&[1, 2, 3]);
        let mut buffer = [0u8; 5];
        let result = reader.read(&mut buffer);
        assert_eq!(result.unwrap(), 3);
        assert_eq!(buffer[..3], [1, 2, 3]);
        assert_eq!(buffer[3..], [0, 0]);
    }

    #[test]
    fn test_read_empty() {
        let mut reader = TestReader::new(&[]);
        let mut buffer = [0u8; 5];
        let result = reader.read(&mut buffer);
        assert_eq!(result.unwrap(), 0);
        assert_eq!(buffer, [0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_read_exceed() {
        let mut reader = TestReader::new(&[1, 2]);
        let mut buffer = [0u8; 2];
        let result = reader.read(&mut buffer);
        assert_eq!(result.unwrap(), 2);
        assert_eq!(buffer, [1, 2]);
    }

    #[test]
    fn test_read_buffer_too_small() {
        let mut reader = TestReader::new(&[1, 2, 3]);
        let mut buffer = [0u8; 2];
        let result = reader.read(&mut buffer);
        assert_eq!(result.unwrap(), 2);
        assert_eq!(buffer, [1, 2]);
    }
}

