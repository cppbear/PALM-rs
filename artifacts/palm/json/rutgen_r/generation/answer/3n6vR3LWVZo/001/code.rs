// Answer 0

fn begin_object_value_test() -> std::io::Result<()> {
    use std::io::Cursor;

    struct TestWriter {
        buffer: Vec<u8>,
    }

    impl std::io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_begin_object_value() -> std::io::Result<()> {
        let mut writer = TestWriter { buffer: Vec::new() };
        begin_object_value(&mut writer)?;

        assert_eq!(writer.buffer, b": ");

        Ok(())
    }

    #[test]
    fn test_begin_object_value_empty_writer() -> std::io::Result<()> {
        let mut writer = Cursor::new(Vec::new());
        begin_object_value(&mut writer)?;

        assert_eq!(writer.get_ref(), &b": "[..]);

        Ok(())
    }

    #[should_panic(expected = "not enough bytes read")]
    #[test]
    fn test_begin_object_value_fail() {
        struct FailingWriter;

        impl std::io::Write for FailingWriter {
            fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "write failed"))
            }

            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }

        let mut writer = FailingWriter;
        let _ = begin_object_value(&mut writer);
    }

    Ok(())
}

