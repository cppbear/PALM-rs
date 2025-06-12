// Answer 0

#[derive(Debug)]
struct MockWriter {
    finished: bool,
}

impl MockWriter {
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
    fn new(delegate: W) -> Self {
        Self {
            delegate: Some(delegate),
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

#[test]
fn test_into_inner_without_finish() {
    let writer = MockWriter::new();
    let encoder = EncoderWriter::new(writer);
    let inner_writer = encoder.into_inner();
    assert!(inner_writer.finished == false);
}

#[test]
#[should_panic(expected = "Encoder has already had finish() called")]
fn test_into_inner_after_finish() {
    let mut writer = MockWriter::new();
    let mut encoder = EncoderWriter::new(writer);
    encoder.finish();
    encoder.into_inner(); // This should panic
}

