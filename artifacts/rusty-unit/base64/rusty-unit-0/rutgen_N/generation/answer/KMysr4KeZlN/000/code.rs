// Answer 0

#[test]
fn test_flush_success() {
    struct DummyWriter {
        output: Vec<u8>,
        flushed: bool,
    }

    impl std::io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.flushed = true;
            Ok(())
        }
    }

    struct Encoder {
        delegate: Option<DummyWriter>,
    }

    impl Encoder {
        fn write_all_encoded_output(&mut self) -> Result<(), std::io::Error> {
            // Dummy implementation for testing purpose
            Ok(())
        }

        fn flush(&mut self) -> Result<(), std::io::Error> {
            self.write_all_encoded_output()?;
            self.delegate
                .as_mut()
                .expect("Writer must be present")
                .flush()
        }
    }

    let mut writer = DummyWriter {
        output: Vec::new(),
        flushed: false,
    };

    let mut encoder = Encoder {
        delegate: Some(writer),
    };

    let result = encoder.flush();
    assert!(result.is_ok());
    assert!(encoder.delegate.as_ref().unwrap().flushed);
}

#[test]
#[should_panic]
fn test_flush_missing_writer() {
    struct Encoder {
        delegate: Option<DummyWriter>,
    }

    impl Encoder {
        fn flush(&mut self) -> Result<(), std::io::Error> {
            self.delegate
                .as_mut()
                .expect("Writer must be present")
                .flush()
        }
    }

    let mut encoder = Encoder {
        delegate: None,
    };

    // This should panic because the writer is missing
    let _ = encoder.flush();
}

