// Answer 0

#[test]
fn test_serialize_newtype_variant() {
    use serde::Serialize;
    use crate::error::Result;

    struct MockWriter(Vec<u8>);
    
    impl crate::io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.0.extend_from_slice(buf);
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

    struct MockFormatter;

    impl crate::ser::Formatter for MockFormatter {
        fn begin_object(&self, writer: &mut dyn crate::io::Write) -> Result<()> {
            writer.write(b"{")?;
            Ok(())
        }

        fn begin_object_key(&self, writer: &mut dyn crate::io::Write, _: bool) -> Result<()> {
            Ok(())
        }

        fn end_object_key(&self, writer: &mut dyn crate::io::Write) -> Result<()> {
            writer.write(b":")?;
            Ok(())
        }

        fn begin_object_value(&self, writer: &mut dyn crate::io::Write) -> Result<()> {
            Ok(())
        }

        fn end_object_value(&self, writer: &mut dyn crate::io::Write) -> Result<()> {
            writer.write(b"}")?;
            Ok(())
        }

        fn end_object(&self, writer: &mut dyn crate::io::Write) -> Result<()> {
            writer.write(b"}")?;
            Ok(())
        }
    }

    let mut writer = MockWriter(Vec::new());
    let formatter = MockFormatter;

    let mut serializer = crate::Serializer { writer: &mut writer, formatter };

    #[derive(Serialize)]
    struct TestStruct {
        value: i32,
    }

    let test_value = TestStruct { value: 42 };

    let result = serializer.serialize_newtype_variant("Test", 0, "variant_name", &test_value);

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(writer.0).unwrap(), r#"{"variant_name":{"value":42}}"#);
}

