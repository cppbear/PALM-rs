// Answer 0

#[test]
fn test_serialize_tuple_variant_success() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.output.extend_from_slice(buf);
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn begin_object(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }

        fn begin_object_key(&mut self, _writer: &mut MockWriter, _first: bool) -> Result<()> {
            Ok(())
        }

        fn end_object_key(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }

        fn begin_object_value(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter;
    let serializer = Serializer { writer: &mut writer, formatter };

    let result = serializer.serialize_tuple_variant("TestName", 0, "TestVariant", 0);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_tuple_variant_fail_on_writer_error() {
    struct FailingWriter;

    impl io::Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(Error::from(ErrorCode::Io))
        }

        fn write_all(&mut self, _buf: &[u8]) -> Result<()> {
            Err(Error::from(ErrorCode::Io))
        }

        fn flush(&mut self) -> Result<()> {
            Err(Error::from(ErrorCode::Io))
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn begin_object(&mut self, _writer: &mut FailingWriter) -> Result<()> {
            Ok(())
        }

        fn begin_object_key(&mut self, _writer: &mut FailingWriter, _first: bool) -> Result<()> {
            Ok(())
        }

        fn end_object_key(&mut self, _writer: &mut FailingWriter) -> Result<()> {
            Ok(())
        }

        fn begin_object_value(&mut self, _writer: &mut FailingWriter) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = FailingWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer: &mut writer, formatter };

    let _result = serializer.serialize_tuple_variant("Name", 0, "Variant", 1);
}

