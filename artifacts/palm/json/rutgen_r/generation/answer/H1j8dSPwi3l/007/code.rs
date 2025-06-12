// Answer 0

#[test]
fn test_parse_escape_for_f_character() {
    use std::io::Cursor;
    use std::result::Result;

    struct TestReader<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> Read<'a> for TestReader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
            let chunk = &self.data[self.position..];
            let len = buf.len().min(chunk.len());
            buf[..len].copy_from_slice(&chunk[..len]);
            self.position += len;
            Ok(len)
        }
    }

    let mut scratch = Vec::new();
    let input_data = vec![b'\\', b'f'];
    let mut reader = TestReader { data: &input_data, position: 0 };

    let result = parse_escape(&mut reader, false, &mut scratch);
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![b'\x0c']);
}

#[test]
fn test_parse_escape_invalid_character() {
    use std::io::Cursor;

    struct TestReader<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> Read<'a> for TestReader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
            let chunk = &self.data[self.position..];
            let len = buf.len().min(chunk.len());
            buf[..len].copy_from_slice(&chunk[..len]);
            self.position += len;
            Ok(len)
        }
    }

    let mut scratch = Vec::new();
    let input_data = vec![b'\\', b'x']; // invalid escape
    let mut reader = TestReader { data: &input_data, position: 0 };

    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_err());
}

