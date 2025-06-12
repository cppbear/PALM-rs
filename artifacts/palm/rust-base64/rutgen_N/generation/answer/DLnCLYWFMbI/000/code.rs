// Answer 0

#[test]
fn test_flush_success() {
    use std::io::{self, Write};
    
    struct MockEncoder {
        should_fail: bool,
    }
    
    impl MockEncoder {
        fn flush(&mut self) -> io::Result<()> {
            if self.should_fail {
                Err(io::Error::new(io::ErrorKind::Other, "flush failed"))
            } else {
                Ok(())
            }
        }
    }

    struct EncoderStringWriter {
        encoder: MockEncoder,
    }

    impl EncoderStringWriter {
        fn flush(&mut self) -> io::Result<()> {
            self.encoder.flush()
        }
    }

    let mut writer = EncoderStringWriter {
        encoder: MockEncoder { should_fail: false },
    };
    let result = writer.flush();
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "flush failed")]
fn test_flush_failure() {
    use std::io;

    struct MockEncoder {
        should_fail: bool,
    }

    impl MockEncoder {
        fn flush(&mut self) -> io::Result<()> {
            if self.should_fail {
                Err(io::Error::new(io::ErrorKind::Other, "flush failed"))
            } else {
                Ok(())
            }
        }
    }

    struct EncoderStringWriter {
        encoder: MockEncoder,
    }

    impl EncoderStringWriter {
        fn flush(&mut self) -> io::Result<()> {
            self.encoder.flush().unwrap()
        }
    }

    let mut writer = EncoderStringWriter {
        encoder: MockEncoder { should_fail: true },
    };
    writer.flush(); // This should panic
}

