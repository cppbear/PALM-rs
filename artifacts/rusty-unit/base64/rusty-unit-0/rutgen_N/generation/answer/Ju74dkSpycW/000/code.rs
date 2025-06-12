// Answer 0

#[test]
fn test_write_success() {
    struct MockEncoder {
        written: usize,
    }

    impl MockEncoder {
        fn new() -> Self {
            MockEncoder { written: 0 }
        }

        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.written += buf.len();
            Ok(buf.len())
        }
    }

    struct EncoderStringWriter {
        encoder: MockEncoder,
    }

    impl EncoderStringWriter {
        fn new() -> Self {
            EncoderStringWriter {
                encoder: MockEncoder::new(),
            }
        }

        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.encoder.write(buf)
        }
    }

    let mut writer = EncoderStringWriter::new();
    let data = b"Hello, World!";
    let result = writer.write(data).unwrap();

    assert_eq!(result, data.len());
}

#[test]
fn test_write_empty_buffer() {
    struct MockEncoder {
        written: usize,
    }

    impl MockEncoder {
        fn new() -> Self {
            MockEncoder { written: 0 }
        }

        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.written += buf.len();
            Ok(buf.len())
        }
    }

    struct EncoderStringWriter {
        encoder: MockEncoder,
    }

    impl EncoderStringWriter {
        fn new() -> Self {
            EncoderStringWriter {
                encoder: MockEncoder::new(),
            }
        }

        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.encoder.write(buf)
        }
    }

    let mut writer = EncoderStringWriter::new();
    let result = writer.write(&[]).unwrap();

    assert_eq!(result, 0);
}

