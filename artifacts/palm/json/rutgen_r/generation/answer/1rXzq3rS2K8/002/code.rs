// Answer 0

fn serialize_f32_tests() -> Result<()> {
    struct TestFormatter;

    impl TestFormatter {
        fn begin_string(&mut self, _: &mut ()) -> Result<()> {
            Ok(())
        }

        fn write_f32(&mut self, _: &mut (), _: f32) -> Result<()> {
            Err(Error::io)
        }

        fn end_string(&mut self, _: &mut ()) -> Result<()> {
            Ok(())
        }
    }

    struct TestSerializer {
        formatter: TestFormatter,
        writer: (),
    }

    struct TestSerializerWrapper {
        ser: TestSerializer,
    }

    impl TestSerializerWrapper {
        fn serialize_f32(self, value: f32) -> Result<()> {
            if !value.is_finite() {
                return Err(Error::from("float key must be finite"));
            }

            tri!(self.ser.formatter.begin_string(&mut self.ser.writer).map_err(Error::io));
            tri!(self.ser.formatter.write_f32(&mut self.ser.writer, value).map_err(Error::io));
            self.ser.formatter.end_string(&mut self.ser.writer).map_err(Error::io)
        }
    }

    let test_serializer = TestSerializerWrapper {
        ser: TestSerializer {
            formatter: TestFormatter,
            writer: (),
        },
    };

    // This should return an error due to write_f32 failing with an Err variant
    let result = test_serializer.serialize_f32(3.14);
    assert!(result.is_err());

    Ok(())
}

