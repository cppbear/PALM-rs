// Answer 0

#[test]
fn test_flush_with_error_from_write_all_encoded_output() {
    struct MockWriter {
        should_fail: bool,
    }
    
    impl MockWriter {
        fn new(should_fail: bool) -> Self {
            MockWriter { should_fail }
        }
    }

    impl std::io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct Encoder {
        delegate: Option<MockWriter>,
    }

    impl Encoder {
        fn new(delegate: Option<MockWriter>) -> Self {
            Encoder { delegate }
        }

        fn write_all_encoded_output(&mut self) -> Result<()> {
            if self.delegate.as_ref().unwrap().should_fail {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "write error"))
            } else {
                Ok(())
            }
        }

        fn flush(&mut self) -> Result<()> {
            self.write_all_encoded_output()?;
            self.delegate
                .as_mut()
                .expect("Writer must be present")
                .flush()
        }
    }

    let mut encoder = Encoder::new(Some(MockWriter::new(true)));
    let result = encoder.flush();
    assert!(result.is_err());
}

#[test]
fn test_flush_without_error() {
    struct MockWriter {
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter {}
        }
    }

    impl std::io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct Encoder {
        delegate: Option<MockWriter>,
    }

    impl Encoder {
        fn new(delegate: Option<MockWriter>) -> Self {
            Encoder { delegate }
        }

        fn write_all_encoded_output(&mut self) -> Result<()> {
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            self.write_all_encoded_output()?;
            self.delegate
                .as_mut()
                .expect("Writer must be present")
                .flush()
        }
    }

    let mut encoder = Encoder::new(Some(MockWriter::new()));
    let result = encoder.flush();
    assert!(result.is_ok());
}

