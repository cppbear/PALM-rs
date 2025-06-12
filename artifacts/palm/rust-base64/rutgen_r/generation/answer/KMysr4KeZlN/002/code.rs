// Answer 0

#[test]
fn test_flush_successful() {
    struct DummyWriter;
    
    impl std::io::Write for DummyWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct Encoder {
        delegate: Option<DummyWriter>,
    }

    impl Encoder {
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

    let mut encoder = Encoder {
        delegate: Some(DummyWriter),
    };

    assert!(encoder.flush().is_ok());
}

#[test]
#[should_panic(expected = "Writer must be present")]
fn test_flush_writer_panic() {
    struct Encoder {
        delegate: Option<DummyWriter>,
    }

    impl Encoder {
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

    let mut encoder = Encoder {
        delegate: None,
    };

    let _ = encoder.flush();
}

