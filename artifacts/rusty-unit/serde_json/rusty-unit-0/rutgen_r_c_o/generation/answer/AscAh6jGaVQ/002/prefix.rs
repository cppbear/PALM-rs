// Answer 0

#[test]
fn test_ignore_escape_backslash() {
    struct TestReader {
        bytes: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(bytes: Vec<u8>) -> Self {
            TestReader { bytes, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                let byte = self.bytes[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut reader = TestReader::new(vec![b'\\', b'r']);
    let _ = ignore_escape(&mut reader);
}

#[test]
fn test_ignore_escape_double_quote() {
    struct TestReader {
        bytes: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(bytes: Vec<u8>) -> Self {
            TestReader { bytes, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                let byte = self.bytes[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut reader = TestReader::new(vec![b'\\', b'"']);
    let _ = ignore_escape(&mut reader);
}

#[test]
fn test_ignore_escape_newline() {
    struct TestReader {
        bytes: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(bytes: Vec<u8>) -> Self {
            TestReader { bytes, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                let byte = self.bytes[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut reader = TestReader::new(vec![b'\\', b'n']);
    let _ = ignore_escape(&mut reader);
}

#[test]
fn test_ignore_escape_tab() {
    struct TestReader {
        bytes: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(bytes: Vec<u8>) -> Self {
            TestReader { bytes, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                let byte = self.bytes[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut reader = TestReader::new(vec![b'\\', b't']);
    let _ = ignore_escape(&mut reader);
}

#[test]
fn test_ignore_escape_form_feed() {
    struct TestReader {
        bytes: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(bytes: Vec<u8>) -> Self {
            TestReader { bytes, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                let byte = self.bytes[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut reader = TestReader::new(vec![b'\\', b'f']);
    let _ = ignore_escape(&mut reader);
}

#[test]
fn test_ignore_escape_slash() {
    struct TestReader {
        bytes: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(bytes: Vec<u8>) -> Self {
            TestReader { bytes, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                let byte = self.bytes[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut reader = TestReader::new(vec![b'\\', b'/']);
    let _ = ignore_escape(&mut reader);
}

#[test]
fn test_ignore_escape_backspace() {
    struct TestReader {
        bytes: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(bytes: Vec<u8>) -> Self {
            TestReader { bytes, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                let byte = self.bytes[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut reader = TestReader::new(vec![b'\\', b'b']);
    let _ = ignore_escape(&mut reader);
}

#[test]
fn test_ignore_escape_unicode() {
    struct TestReader {
        bytes: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(bytes: Vec<u8>) -> Self {
            TestReader { bytes, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                let byte = self.bytes[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut reader = TestReader::new(vec![b'\\', b'u']);
    let _ = ignore_escape(&mut reader);
}

#[test]
#[should_panic]
fn test_ignore_escape_invalid() {
    struct TestReader {
        bytes: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(bytes: Vec<u8>) -> Self {
            TestReader { bytes, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                let byte = self.bytes[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut reader = TestReader::new(vec![b'\\', b'x']);
    let _ = ignore_escape(&mut reader);
}

