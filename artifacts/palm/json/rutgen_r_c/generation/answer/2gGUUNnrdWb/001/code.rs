// Answer 0

#[test]
fn test_serialize_u32_err_begin_string() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write error"))
        }

        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Err(Error::from("formatter error"))
        }

        fn write_u32(&mut self, _writer: &mut MockWriter, _value: u32) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    struct TestSerializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    impl ser::Serializer for TestSerializer {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<(), Error>;
        type SerializeTuple = Impossible<(), Error>;
        type SerializeTupleStruct = Impossible<(), Error>;
        type SerializeTupleVariant = Impossible<(), Error>;
        type SerializeMap = Impossible<(), Error>;
        type SerializeStruct = Impossible<(), Error>;
        type SerializeStructVariant = Impossible<(), Error>;

        fn serialize_u32(self, value: u32) -> Result<()> {
            tri!(self.formatter.begin_string(&mut self.writer).map_err(Error::io));
            tri!(self.formatter.write_u32(&mut self.writer, value).map_err(Error::io));
            self.formatter.end_string(&mut self.writer).map_err(Error::io)
        }
    }

    let serializer = TestSerializer {
        writer: MockWriter,
        formatter: MockFormatter,
    };

    let result = serializer.serialize_u32(42);
    assert!(result.is_err());
}

#[test]
fn test_serialize_u32_err_write_u32() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }

        fn write_u32(&mut self, _writer: &mut MockWriter, _value: u32) -> Result<()> {
            Err(Error::from("write error"))
        }

        fn end_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    struct TestSerializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    impl ser::Serializer for TestSerializer {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<(), Error>;
        type SerializeTuple = Impossible<(), Error>;
        type SerializeTupleStruct = Impossible<(), Error>;
        type SerializeTupleVariant = Impossible<(), Error>;
        type SerializeMap = Impossible<(), Error>;
        type SerializeStruct = Impossible<(), Error>;
        type SerializeStructVariant = Impossible<(), Error>;

        fn serialize_u32(self, value: u32) -> Result<()> {
            tri!(self.formatter.begin_string(&mut self.writer).map_err(Error::io));
            tri!(self.formatter.write_u32(&mut self.writer, value).map_err(Error::io));
            self.formatter.end_string(&mut self.writer).map_err(Error::io)
        }
    }

    let serializer = TestSerializer {
        writer: MockWriter,
        formatter: MockFormatter,
    };

    let result = serializer.serialize_u32(42);
    assert!(result.is_err());
}

#[test]
fn test_serialize_u32_err_end_string() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }

        fn write_u32(&mut self, _writer: &mut MockWriter, _value: u32) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Err(Error::from("end error"))
        }
    }

    struct TestSerializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    impl ser::Serializer for TestSerializer {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<(), Error>;
        type SerializeTuple = Impossible<(), Error>;
        type SerializeTupleStruct = Impossible<(), Error>;
        type SerializeTupleVariant = Impossible<(), Error>;
        type SerializeMap = Impossible<(), Error>;
        type SerializeStruct = Impossible<(), Error>;
        type SerializeStructVariant = Impossible<(), Error>;

        fn serialize_u32(self, value: u32) -> Result<()> {
            tri!(self.formatter.begin_string(&mut self.writer).map_err(Error::io));
            tri!(self.formatter.write_u32(&mut self.writer, value).map_err(Error::io));
            self.formatter.end_string(&mut self.writer).map_err(Error::io)
        }
    }

    let serializer = TestSerializer {
        writer: MockWriter,
        formatter: MockFormatter,
    };

    let result = serializer.serialize_u32(42);
    assert!(result.is_err());
}

