// Answer 0

#[test]
fn test_serialize_u64_begin_string_error() {
    struct MockFormatter {
        begin_string_error: bool,
    }

    impl MockFormatter {
        fn new(begin_string_error: bool) -> Self {
            Self { begin_string_error }
        }

        fn begin_string(&self, writer: &mut ()) -> Result<(), ()> {
            if self.begin_string_error {
                Err(())
            } else {
                Ok(())
            }
        }

        fn write_u64(&self, writer: &mut (), value: u64) -> Result<(), ()> {
            Ok(())
        }

        fn end_string(&self, writer: &mut ()) -> Result<(), ()> {
            Ok(())
        }
    }

    struct MockWriter;

    struct Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    struct SerializerWrapper {
        ser: Serializer,
    }

    impl SerializerWrapper {
        fn serialize_u64(self, value: u64) -> Result<(), ()> {
            tri!(self.ser.formatter.begin_string(&mut self.ser.writer).map_err(|_| ()));
            tri!(self.ser.formatter.write_u64(&mut self.ser.writer, value).map_err(|_| ()));
            self.ser.formatter.end_string(&mut self.ser.writer).map_err(|_| ())
        }
    }

    let formatter = MockFormatter::new(true); // simulate error on begin_string
    let serializer = Serializer { formatter, writer: MockWriter };
    let wrapper = SerializerWrapper { ser: serializer };

    let result = wrapper.serialize_u64(42);
    assert!(result.is_err());
}

