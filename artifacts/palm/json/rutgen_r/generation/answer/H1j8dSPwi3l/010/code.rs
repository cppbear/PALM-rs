// Answer 0

#[test]
fn test_parse_escape_double_quote() {
    use std::io::Cursor;
    use std::io::Read;
    use serde_json::error::ErrorCode;
    use serde_json::Result;

    struct TestReader<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> Read for TestReader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
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
    let mut reader = TestReader { data: b"\\\"", position: 0 };
    let result = parse_escape(&mut reader, true, &mut scratch);
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, b"\"".to_vec());
}

#[test]
fn test_parse_escape_backslash() {
    use std::io::Cursor;
    use std::io::Read;
    use serde_json::error::ErrorCode;
    use serde_json::Result;

    struct TestReader<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> Read for TestReader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
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
    let mut reader = TestReader { data: b"\\\\", position: 0 };
    let result = parse_escape(&mut reader, true, &mut scratch);
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, b"\\".to_vec());
}

#[test]
fn test_parse_escape_slash() {
    use std::io::Cursor;
    use std::io::Read;
    use serde_json::error::ErrorCode;
    use serde_json::Result;

    struct TestReader<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> Read for TestReader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
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
    let mut reader = TestReader { data: b"\\/", position: 0 };
    let result = parse_escape(&mut reader, true, &mut scratch);
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, b"/".to_vec());
}

#[test]
fn test_parse_escape_b() {
    use std::io::Cursor;
    use std::io::Read;
    use serde_json::error::ErrorCode;
    use serde_json::Result;

    struct TestReader<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> Read for TestReader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
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
    let mut reader = TestReader { data: b"\\b", position: 0 };
    let result = parse_escape(&mut reader, true, &mut scratch);
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, b"\x08".to_vec());
}

#[test]
fn test_parse_escape_f() {
    use std::io::Cursor;
    use std::io::Read;
    use serde_json::error::ErrorCode;
    use serde_json::Result;

    struct TestReader<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> Read for TestReader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
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
    let mut reader = TestReader { data: b"\\f", position: 0 };
    let result = parse_escape(&mut reader, true, &mut scratch);
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, b"\x0c".to_vec());
}

#[test]
fn test_parse_escape_n() {
    use std::io::Cursor;
    use std::io::Read;
    use serde_json::error::ErrorCode;
    use serde_json::Result;

    struct TestReader<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> Read for TestReader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
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
    let mut reader = TestReader { data: b"\\n", position: 0 };
    let result = parse_escape(&mut reader, true, &mut scratch);
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, b"\n".to_vec());
}

#[test]
fn test_parse_escape_r() {
    use std::io::Cursor;
    use std::io::Read;
    use serde_json::error::ErrorCode;
    use serde_json::Result;

    struct TestReader<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> Read for TestReader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
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
    let mut reader = TestReader { data: b"\\r", position: 0 };
    let result = parse_escape(&mut reader, true, &mut scratch);
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, b"\r".to_vec());
}

#[test]
fn test_parse_escape_t() {
    use std::io::Cursor;
    use std::io::Read;
    use serde_json::error::ErrorCode;
    use serde_json::Result;

    struct TestReader<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> Read for TestReader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
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
    let mut reader = TestReader { data: b"\\t", position: 0 };
    let result = parse_escape(&mut reader, true, &mut scratch);
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, b"\t".to_vec());
}

