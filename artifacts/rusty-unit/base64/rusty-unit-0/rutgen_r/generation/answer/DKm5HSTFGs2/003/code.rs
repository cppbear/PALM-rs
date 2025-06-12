// Answer 0

#[test]
#[should_panic(expected = "Encoder has already had finish() called")]
fn test_finish_panics_when_delegate_is_none() {
    struct Encoder<W> {
        delegate: Option<W>,
    }

    impl<W> Encoder<W> {
        pub fn finish(&mut self) -> Result<W, &'static str> {
            assert!(self.delegate.is_some(), "Encoder has already had finish() called");
            // Placeholder implementation logic
            Ok(self.delegate.take().expect("Writer must be present"))
        }
    }

    let mut encoder: Encoder<()> = Encoder { delegate: None };
    let _result = encoder.finish(); // This should trigger a panic
}

