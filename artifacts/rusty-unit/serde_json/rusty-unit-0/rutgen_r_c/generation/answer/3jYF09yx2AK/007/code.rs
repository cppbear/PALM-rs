// Answer 0

#[test]
fn test_serialize_newtype_variant_success() {
    struct MockWriter {
        data: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.data.extend_from_slice(buf);
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

    impl Formatter for MockFormatter {
        fn begin_object(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write(b"{")?;
            Ok(())
        }

        fn begin_object_key(&mut self, writer: &mut dyn io::Write, _: bool) -> Result<()> {
            writer.write(b"\"key\":")?;
            Ok(())
        }

        fn end_object_key(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write(b"")?;
            Ok(())
        }

        fn begin_object_value(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write(b"\"value\"")?;
            Ok(())
        }

        fn end_object_value(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write(b"}")?;
            Ok(())
        }

        fn end_object(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write(b"}")?;
            Ok(())
        }
    }

    struct MockSerializer<'a> {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    impl<'a> ser::Serializer for &mut MockSerializer<'a> {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();

        fn serialize_str(self, value: &str) -> Result<()> {
            self.writer.write(value.as_bytes())?;
            Ok(())
        }

        fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            tri!(self.formatter.begin_object(&mut self.writer).map_err(Error::io));
            tri!(self.formatter.begin_object_key(&mut self.writer, true).map_err(Error::io));
            tri!(self.serialize_str(variant));
            tri!(self.formatter.end_object_key(&mut self.writer).map_err(Error::io));
            tri!(self.formatter.begin_object_value(&mut self.writer).map_err(Error::io));
            tri!(value.serialize(&mut *self));
            tri!(self.formatter.end_object_value(&mut self.writer).map_err(Error::io));
            self.formatter.end_object(&mut self.writer).map_err(Error::io)
        }

        fn collect_str<T>(self, value: &T) -> Result<()>
        where
            T: ?Sized + Display,
        {
            self.serialize_str(&value.to_string())
        }
    }

    let mut writer = MockWriter { data: vec![] };
    let mut serializer = MockSerializer { writer, formatter: MockFormatter };

    let result = serializer.serialize_newtype_variant("MyEnum", 0, "Variant", &"a value");
    assert!(result.is_ok());
    assert_eq!(serializer.writer.data, b"{\"key\":\"Variant\"\"value\"}");
}

#[test]
#[should_panic]
fn test_serialize_newtype_variant_failure() {
    struct PanicWriter;

    impl io::Write for PanicWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            panic!("Write failure");
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

    impl Formatter for MockFormatter {
        fn begin_object(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn begin_object_key(&mut self, _: &mut dyn io::Write, _: bool) -> Result<()> {
            Ok(())
        }

        fn end_object_key(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn begin_object_value(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_object_value(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_object(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    struct MockSerializer<'a> {
        writer: PanicWriter,
        formatter: MockFormatter,
    }

    impl<'a> ser::Serializer for &mut MockSerializer<'a> {
        type Ok = ();
        type Error = Error;

        fn serialize_str(self, value: &str) -> Result<()> {
            self.writer.write(value.as_bytes())?;
            Ok(())
        }

        fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            tri!(self.formatter.begin_object(&mut self.writer).map_err(Error::io));
            tri!(self.formatter.begin_object_key(&mut self.writer, true).map_err(Error::io));
            tri!(self.serialize_str(variant));
            tri!(self.formatter.end_object_key(&mut self.writer).map_err(Error::io));
            tri!(self.formatter.begin_object_value(&mut self.writer).map_err(Error::io));
            tri!(value.serialize(&mut *self));
            tri!(self.formatter.end_object_value(&mut self.writer).map_err(Error::io));
            self.formatter.end_object(&mut self.writer).map_err(Error::io)
        }

        fn collect_str<T>(self, value: &T) -> Result<()>
        where
            T: ?Sized + Display,
        {
            self.serialize_str(&value.to_string())
        }
    }

    let mut writer = PanicWriter;
    let mut serializer = MockSerializer { writer, formatter: MockFormatter };

    let _ = serializer.serialize_newtype_variant("MyEnum", 0, "Variant", &"a value");
}

