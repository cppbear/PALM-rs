// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::ser::{Serializer, Error};
    use serde_json::Serializer as SerdeSerializer;

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

    struct TestSerializer {
        writer: TestWriter,
        formatter: SerdeSerializer<TestWriter>,
    }

    impl TestSerializer {
        fn new() -> Self {
            let writer = TestWriter::new();
            let formatter = SerdeSerializer::new(writer);
            TestSerializer { writer, formatter }
        }

        fn serialize_bool(self, value: bool) -> Result<()> {
            tri!(self
                .formatter
                .begin_string(&mut self.writer)
                .map_err(Error::io));
            tri!(self
                .formatter
                .write_bool(&mut self.writer, value)
                .map_err(Error::io));
            self.formatter
                .end_string(&mut self.writer)
                .map_err(Error::io)
        }
    }

    #[test]
    fn test_serialize_bool_true() {
        let serializer = TestSerializer::new();
        let result = serializer.serialize_bool(true);
        assert!(result.is_ok());
        assert_eq!(serializer.writer.output, b"true");
    }

    #[test]
    fn test_serialize_bool_false() {
        let serializer = TestSerializer::new();
        let result = serializer.serialize_bool(false);
        assert!(result.is_ok());
        assert_eq!(serializer.writer.output, b"false");
    }
}

