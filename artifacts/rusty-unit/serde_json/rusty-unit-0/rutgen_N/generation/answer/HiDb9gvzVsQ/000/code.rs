// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    struct TestWriter {
        output: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: Vec::new() }
        }
    }

    impl std::io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    #[test]
    fn test_with_formatter() {
        let writer = TestWriter::new();
        let formatter = TestFormatter;

        let serializer = with_formatter(writer, formatter);

        assert!(serializer.writer.output.is_empty());
    }
}

