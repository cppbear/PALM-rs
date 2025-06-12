// Answer 0

#[test]
fn test_serialize_i128_success() {
    struct DummyWriter;

    impl DummyWriter {
        fn new() -> Self {
            DummyWriter {}
        }
    }

    struct DummyFormatter {
        success: bool,
    }

    impl DummyFormatter {
        fn begin_string(&mut self, _: &mut DummyWriter) -> Result<(), ()> {
            if self.success {
                Ok(())
            } else {
                Err(())
            }
        }

        fn write_i128(&mut self, _: &mut DummyWriter, _: i128) -> Result<(), ()> {
            if self.success {
                Ok(())
            } else {
                Err(())
            }
        }

        fn end_string(&mut self, _: &mut DummyWriter) -> Result<(), ()> {
            if self.success {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    struct Serializer {
        formatter: DummyFormatter,
        writer: DummyWriter,
    }

    struct SerializerWrapper {
        ser: Serializer,
    }

    impl SerializerWrapper {
        fn serialize_i128(self, value: i128) -> Result<(), ()> {
            self.ser.formatter.begin_string(&mut self.ser.writer)?;
            self.ser.formatter.write_i128(&mut self.ser.writer, value)?;
            self.ser.formatter.end_string(&mut self.ser.writer)?;
            Ok(())
        }
    }

    let formatter = DummyFormatter { success: true };
    let writer = DummyWriter::new();
    let ser = Serializer { formatter, writer };
    
    let wrapper = SerializerWrapper { ser };
    let result = wrapper.serialize_i128(123456789012345678901234567890_i128);
    assert!(result.is_ok());
}

#[should_panic]
#[test]
fn test_serialize_i128_failure_begin_string() {
    struct DummyWriter;

    struct DummyFormatter {
        success: bool,
    }

    impl DummyFormatter {
        fn begin_string(&mut self, _: &mut DummyWriter) -> Result<(), ()> {
            if self.success {
                Ok(())
            } else {
                Err(())
            }
        }

        fn write_i128(&mut self, _: &mut DummyWriter, _: i128) -> Result<(), ()> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut DummyWriter) -> Result<(), ()> {
            Ok(())
        }
    }

    struct Serializer {
        formatter: DummyFormatter,
        writer: DummyWriter,
    }

    struct SerializerWrapper {
        ser: Serializer,
    }

    impl SerializerWrapper {
        fn serialize_i128(self, value: i128) -> Result<(), ()> {
            self.ser.formatter.begin_string(&mut self.ser.writer)?;
            self.ser.formatter.write_i128(&mut self.ser.writer, value)?;
            self.ser.formatter.end_string(&mut self.ser.writer)?;
            Ok(())
        }
    }

    let formatter = DummyFormatter { success: false };
    let writer = DummyWriter {};
    let ser = Serializer { formatter, writer };
    
    let wrapper = SerializerWrapper { ser };
    wrapper.serialize_i128(123456789012345678901234567890_i128);
}

#[should_panic]
#[test]
fn test_serialize_i128_failure_write_i128() {
    struct DummyWriter;

    struct DummyFormatter {
        success: bool,
    }

    impl DummyFormatter {
        fn begin_string(&mut self, _: &mut DummyWriter) -> Result<(), ()> {
            Ok(())
        }

        fn write_i128(&mut self, _: &mut DummyWriter, _: i128) -> Result<(), ()> {
            if self.success {
                Ok(())
            } else {
                Err(())
            }
        }

        fn end_string(&mut self, _: &mut DummyWriter) -> Result<(), ()> {
            Ok(())
        }
    }

    struct Serializer {
        formatter: DummyFormatter,
        writer: DummyWriter,
    }

    struct SerializerWrapper {
        ser: Serializer,
    }

    impl SerializerWrapper {
        fn serialize_i128(self, value: i128) -> Result<(), ()> {
            self.ser.formatter.begin_string(&mut self.ser.writer)?;
            self.ser.formatter.write_i128(&mut self.ser.writer, value)?;
            self.ser.formatter.end_string(&mut self.ser.writer)?;
            Ok(())
        }
    }

    let formatter = DummyFormatter { success: false };
    let writer = DummyWriter {};
    let ser = Serializer { formatter, writer };

    let wrapper = SerializerWrapper { ser };
    wrapper.serialize_i128(123456789012345678901234567890_i128);
}

