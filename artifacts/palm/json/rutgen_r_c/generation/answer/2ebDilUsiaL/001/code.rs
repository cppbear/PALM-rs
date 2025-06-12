// Answer 0

#[test]
fn test_indent_with_zero_write() {
    struct MockWriter {
        written: Vec<u8>,
        fail_on_write: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.fail_on_write {
                Err(io::Error::new(io::ErrorKind::Other, "Write failed"))
            } else {
                self.written.extend_from_slice(buf);
                Ok(buf.len())
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut mock_writer = MockWriter {
        written: Vec::new(),
        fail_on_write: true,
    };
    
    let result = indent(&mut mock_writer, 5, b"test");
    
    assert!(result.is_err());
}

#[test]
fn test_indent_with_nonzero_write() {
    struct MockWriter {
        written: Vec<u8>,
        fail_on_write: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.fail_on_write {
                Err(io::Error::new(io::ErrorKind::Other, "Write failed"))
            } else {
                self.written.extend_from_slice(buf);
                Ok(buf.len())
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut mock_writer = MockWriter {
        written: Vec::new(),
        fail_on_write: true,
    };
    
    let result = indent(&mut mock_writer, 3, b"test");
    
    assert!(result.is_err());
}

