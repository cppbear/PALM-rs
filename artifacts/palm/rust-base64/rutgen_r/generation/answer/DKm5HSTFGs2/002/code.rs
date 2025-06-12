// Answer 0

#[test]
fn test_finish_success() {
    struct TestWriter {
        data: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { data: Vec::new() }
        }

        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }
    }

    struct Encoder<W> {
        delegate: Option<W>,
    }

    impl<W> Encoder<W> {
        fn new(delegate: W) -> Self {
            Encoder {
                delegate: Some(delegate),
            }
        }

        fn write_final_leftovers(&mut self) -> std::io::Result<()> {
            // Simulate successful final write
            Ok(())
        }

        pub fn finish(&mut self) -> Result<W, std::io::Error> {
            assert!(
                self.delegate.is_some(),
                "Encoder has already had finish() called"
            );

            self.write_final_leftovers()?;

            let writer = self.delegate.take().expect("Writer must be present");

            Ok(writer)
        }
    }

    let mut writer = TestWriter::new();
    let mut encoder = Encoder::new(writer);
    
    let result = encoder.finish();
    
    assert!(result.is_ok());
}

