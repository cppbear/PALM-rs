// Answer 0

#[derive(Default)]
struct MockEncoder {
    should_panic: bool,
}

impl MockEncoder {
    fn flush(&mut self) -> io::Result<()> {
        if self.should_panic {
            return Err(io::Error::new(io::ErrorKind::Other, "Mock panic!"));
        }
        Ok(())
    }
}

struct StringWriter {
    encoder: MockEncoder,
}

impl StringWriter {
    fn flush(&mut self) -> io::Result<()> {
        self.encoder.flush()
    }
}

#[test]
fn test_flush_success() {
    let mut writer = StringWriter {
        encoder: MockEncoder::default(),
    };
    let result = writer.flush();
    assert!(result.is_ok());
}

#[test]
fn test_flush_panic_condition() {
    let mut writer = StringWriter {
        encoder: MockEncoder { should_panic: true },
    };
    let result = writer.flush();
    assert!(result.is_err());
}

