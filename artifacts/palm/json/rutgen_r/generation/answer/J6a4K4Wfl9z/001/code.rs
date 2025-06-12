// Answer 0

#[test]
#[should_panic]
fn test_serialize_i8_begin_string_err() {
    struct Formatter;

    impl Formatter {
        fn begin_string(&mut self, _writer: &mut String) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "begin_string error"))
        }

        fn write_i8(&mut self, _writer: &mut String, _value: i8) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut String) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct Ser {
        writer: String,
        formatter: Formatter,
    }

    struct Serializer {
        ser: Ser,
    }

    impl Serializer {
        fn serialize_i8(self, value: i8) -> Result<()> {
            tri!(self.ser.formatter.begin_string(&mut self.ser.writer).map_err(Error::io));
            tri!(self.ser.formatter.write_i8(&mut self.ser.writer, value).map_err(Error::io));
            self.ser.formatter.end_string(&mut self.ser.writer).map_err(Error::io)
        }
    }

    let formatter = Formatter {};
    let ser = Ser {
        writer: String::new(),
        formatter,
    };
    let serializer = Serializer { ser };

    // This part should panic due to begin_string returning an error
    let _ = serializer.serialize_i8(42);
}

