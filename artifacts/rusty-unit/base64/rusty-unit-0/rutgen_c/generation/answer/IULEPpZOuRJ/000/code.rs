// Answer 0

#[test]
fn test_into_inner_success() {
    struct DummyEngine;
    struct DummyWriter {
        inner: Vec<u8>,
    }
    
    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.inner.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    
    let engine = DummyEngine;
    let writer = DummyWriter { inner: Vec::new() };
    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    
    let inner_writer = encoder_writer.into_inner();
    assert!(inner_writer.inner.is_empty());
}

#[test]
#[should_panic(expected = "Encoder has already had finish() called")]
fn test_into_inner_after_finish_panics() {
    struct DummyEngine;
    struct DummyWriter {
        inner: Vec<u8>,
    }
    
    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.inner.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    
    let engine = DummyEngine;
    let writer = DummyWriter { inner: Vec::new() };
    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    
    let _ = encoder_writer.finish(); // Assume this is called correctly and would normally not panic
    
    // This call should panic as per the context provided
    encoder_writer.into_inner();
}

#[test]
fn test_into_inner_no_panic_before_finish() {
    struct DummyEngine;
    struct DummyWriter {
        inner: Vec<u8>,
    }
    
    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.inner.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    
    let engine = DummyEngine;
    let writer = DummyWriter { inner: Vec::new() };
    let encoder_writer = EncoderWriter::new(writer, &engine);
    
    // Just call into_inner without calling finish; should not panic
    let inner_writer = encoder_writer.into_inner();
    assert!(!inner_writer.inner.is_empty());
}

