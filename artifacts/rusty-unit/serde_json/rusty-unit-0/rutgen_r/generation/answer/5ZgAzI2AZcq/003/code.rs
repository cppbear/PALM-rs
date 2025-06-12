// Answer 0

#[test]
fn test_serialize_i16_success() {
    struct MockFormatter;
    struct MockWriter {
        output: Vec<u8>,
    }
    
    impl MockFormatter {
        fn begin_string(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
        
        fn write_i16(&self, _writer: &mut MockWriter, _value: i16) -> Result<(), std::io::Error> {
            Ok(())
        }
        
        fn end_string(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct Serializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }
    
    struct Context {
        ser: Serializer,
    }
    
    impl Context {
        fn serialize_i16(self, value: i16) -> Result<()> {
            tri!(self
                .ser
                .formatter
                .begin_string(&mut self.ser.writer)
                .map_err(Error::io));
            tri!(self
                .ser
                .formatter
                .write_i16(&mut self.ser.writer, value)
                .map_err(Error::io));
            self.ser
                .formatter
                .end_string(&mut self.ser.writer)
                .map_err(Error::io)
        }
    }
    
    let writer = MockWriter { output: vec![] };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    let context = Context { ser: serializer };
    
    let result = context.serialize_i16(12345);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_i16_begin_string_fail() {
    struct MockFormatter;
    struct MockWriter {
        output: Vec<u8>,
    }

    impl MockFormatter {
        fn begin_string(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "begin_string failed"))
        }

        fn write_i16(&self, _writer: &mut MockWriter, _value: i16) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct Serializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    struct Context {
        ser: Serializer,
    }

    impl Context {
        fn serialize_i16(self, value: i16) -> Result<()> {
            tri!(self
                .ser
                .formatter
                .begin_string(&mut self.ser.writer)
                .map_err(Error::io));
            tri!(self
                .ser
                .formatter
                .write_i16(&mut self.ser.writer, value)
                .map_err(Error::io));
            self.ser
                .formatter
                .end_string(&mut self.ser.writer)
                .map_err(Error::io)
        }
    }

    let writer = MockWriter { output: vec![] };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    let context = Context { ser: serializer };

    context.serialize_i16(12345).unwrap();
}

