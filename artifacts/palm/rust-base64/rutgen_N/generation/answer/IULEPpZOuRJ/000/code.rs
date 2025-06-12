// Answer 0

#[test]
fn test_into_inner_panic_after_finish() {
    struct TestWriter {
        finished: bool,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { finished: false }
        }
        
        fn finish(&mut self) {
            self.finished = true;
        }
    }

    struct EncoderWriter<W> {
        delegate: Option<W>,
    }

    impl<W> EncoderWriter<W> {
        fn new(writer: W) -> Self {
            Self {
                delegate: Some(writer),
            }
        }

        pub fn finish(&mut self) {
            self.delegate.take();
        }

        pub fn into_inner(mut self) -> W {
            self.delegate
                .take()
                .expect("Encoder has already had finish() called")
        }
    }

    let mut writer = TestWriter::new();
    let mut encoder = EncoderWriter::new(writer);

    encoder.finish();
    
    let result = std::panic::catch_unwind(|| {
        encoder.into_inner();
    });

    assert!(result.is_err());
}

#[test]
fn test_into_inner_success() {
    struct TestWriter {
        data: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { data: vec![] }
        }

        fn write(&mut self, bytes: &[u8]) {
            self.data.extend_from_slice(bytes);
        }
    }

    struct EncoderWriter<W> {
        delegate: Option<W>,
    }

    impl<W> EncoderWriter<W> {
        fn new(writer: W) -> Self {
            Self {
                delegate: Some(writer),
            }
        }

        pub fn finish(&mut self) {
            self.delegate.take();
        }

        pub fn into_inner(mut self) -> W {
            self.delegate
                .take()
                .expect("Encoder has already had finish() called")
        }
    }

    let writer = TestWriter::new();
    let encoder = EncoderWriter::new(writer);
    
    let inner_writer = encoder.into_inner();
    // Here we can check the inner_writer, but since we don't expect
    // additional specific behavior, we're just validating that we can get it.
    assert!(inner_writer.data.is_empty());
}

