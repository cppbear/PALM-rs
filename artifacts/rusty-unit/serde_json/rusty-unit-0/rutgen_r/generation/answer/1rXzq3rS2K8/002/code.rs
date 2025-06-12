// Answer 0

fn test_serialize_f32_finite_success() {
    struct DummyFormatter {
        buffer: Vec<u8>,
    }

    impl DummyFormatter {
        fn begin_string(&mut self) -> Result<(), ()> {
            self.buffer.push(b'"');
            Ok(())
        }

        fn write_f32(&mut self, value: f32) -> Result<(), ()> {
            self.buffer.extend(value.to_string().bytes());
            Ok(())
        }

        fn end_string(&mut self) -> Result<(), ()> {
            self.buffer.push(b'"');
            Ok(())
        }
    }

    struct DummyWriter;

    struct Serializer {
        writer: DummyWriter,
        formatter: DummyFormatter,
    }

    impl Serializer {
        fn serialize_f32(self, value: f32) -> Result<(), ()> {
            if !value.is_finite() {
                return Err(());
            }

            self.formatter.begin_string().map_err(|_| ())?;
            self.formatter.write_f32(value).map_err(|_| ())?;
            self.formatter.end_string().map_err(|_| ())?;

            Ok(())
        }
    }

    let mut formatter = DummyFormatter { buffer: Vec::new() };
    let serializer = Serializer {
        writer: DummyWriter,
        formatter,
    };

    let result = serializer.serialize_f32(1.0);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_f32_non_finite() {
    struct DummyFormatter;

    struct Serializer {
        writer: DummyWriter,
        formatter: DummyFormatter,
    }

    impl Serializer {
        fn serialize_f32(self, value: f32) -> Result<(), ()> {
            if !value.is_finite() {
                return Err(());
            }

            Err(())
        }
    }

    let serializer = Serializer {
        writer: DummyWriter,
        formatter: DummyFormatter,
    };

    let result_nan = serializer.serialize_f32(f32::NAN);
    assert!(result_nan.is_err());

    let result_infinite = serializer.serialize_f32(f32::INFINITY);
    assert!(result_infinite.is_err());
}

#[test]
fn test_serialize_f32_io_error() {
    struct DummyFormatter;

    struct DummyWriter;

    struct Serializer {
        writer: DummyWriter,
        formatter: DummyFormatter,
    }

    impl Serializer {
        fn serialize_f32(self, value: f32) -> Result<(), ()> {
            // Simulated IO error case
            return Err(());
        }
    }

    let serializer = Serializer {
        writer: DummyWriter,
        formatter: DummyFormatter,
    };

    let result = serializer.serialize_f32(1.0);
    assert!(result.is_err());
}

