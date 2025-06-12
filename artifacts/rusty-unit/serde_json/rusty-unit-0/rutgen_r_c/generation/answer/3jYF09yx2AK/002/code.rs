// Answer 0

#[test]
fn test_serialize_newtype_variant_success() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.write(buf)?;
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn begin_object(&mut self, _: &mut TestWriter) -> Result<()> {
            Ok(())
        }

        fn begin_object_key(&mut self, _: &mut TestWriter, _: bool) -> Result<()> {
            Ok(())
        }

        fn end_object_key(&mut self, _: &mut TestWriter) -> Result<()> {
            Ok(())
        }

        fn begin_object_value(&mut self, _: &mut TestWriter) -> Result<()> {
            Ok(())
        }

        fn end_object_value(&mut self, _: &mut TestWriter) -> Result<()> {
            Ok(())
        }

        fn end_object(&mut self, _: &mut TestWriter) -> Result<()> {
            Ok(())
        }
    }

    impl Serialize for String {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.serialize_str(self)
        }
    }

    let mut writer = TestWriter { output: Vec::new() };
    let mut formatter = TestFormatter;
    let serializer = &mut Serializer { writer, formatter };

    let result = serializer.serialize_newtype_variant("name", 0, "variant", &"value".to_string());
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_newtype_variant_failure_on_begin_object_key() {
    struct TestWriter {
        error: bool,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn write_all(&mut self, _: &[u8]) -> Result<()> {
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct FailingFormatter;

    impl Formatter for FailingFormatter {
        fn begin_object(&mut self, _: &mut TestWriter) -> Result<()> {
            Ok(())
        }

        fn begin_object_key(&mut self, _: &mut TestWriter, _: bool) -> Result<()> {
            Err(Error::io(io::Error::new(io::ErrorKind::Other, "key error")))
        }

        fn end_object_key(&mut self, _: &mut TestWriter) -> Result<()> {
            Ok(())
        }

        fn begin_object_value(&mut self, _: &mut TestWriter) -> Result<()> {
            Ok(())
        }

        fn end_object_value(&mut self, _: &mut TestWriter) -> Result<()> {
            Ok(())
        }

        fn end_object(&mut self, _: &mut TestWriter) -> Result<()> {
            Ok(())
        }
    }

    impl Serialize for String {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.serialize_str(self)
        }
    }

    let writer = TestWriter { error: false };
    let formatter = FailingFormatter;
    let serializer = &mut Serializer { writer, formatter };

    let _ = serializer.serialize_newtype_variant("name", 0, "variant", &"value".to_string());
}

