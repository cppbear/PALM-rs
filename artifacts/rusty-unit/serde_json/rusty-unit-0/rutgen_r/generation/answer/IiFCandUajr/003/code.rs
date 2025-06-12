// Answer 0

#[test]
fn test_serialize_f64_nan() {
    struct Writer;
    struct Formatter {
        writer: Writer,
    }

    impl Formatter {
        fn write_null(&self, _writer: &mut Writer) -> Result<()> {
            // Simulation of writing null
            Ok(())
        }

        fn write_f64(&self, _writer: &mut Writer, _value: f64) -> Result<()> {
            // Simulation of writing f64
            Ok(())
        }
    }

    struct Serializer {
        formatter: Formatter,
        writer: Writer,
    }

    impl Serializer {
        fn serialize_f64(self, value: f64) -> Result<()> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => self
                    .formatter
                    .write_null(&mut self.writer)
                    .map_err(Error::io),
                _ => self
                    .formatter
                    .write_f64(&mut self.writer, value)
                    .map_err(Error::io),
            }
        }
    }

    let writer = Writer;
    let formatter = Formatter { writer };
    let serializer = Serializer { formatter, writer };

    let result = serializer.serialize_f64(f64::NAN);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_f64_infinite() {
    struct Writer;
    struct Formatter {
        writer: Writer,
    }

    impl Formatter {
        fn write_null(&self, _writer: &mut Writer) -> Result<()> {
            // Simulation of writing null
            Ok(())
        }

        fn write_f64(&self, _writer: &mut Writer, _value: f64) -> Result<()> {
            // Simulation of writing f64
            Ok(())
        }
    }

    struct Serializer {
        formatter: Formatter,
        writer: Writer,
    }

    impl Serializer {
        fn serialize_f64(self, value: f64) -> Result<()> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => self
                    .formatter
                    .write_null(&mut self.writer)
                    .map_err(Error::io),
                _ => self
                    .formatter
                    .write_f64(&mut self.writer, value)
                    .map_err(Error::io),
            }
        }
    }

    let writer = Writer;
    let formatter = Formatter { writer };
    let serializer = Serializer { formatter, writer };

    let result = serializer.serialize_f64(f64::INFINITY);
    assert!(result.is_ok());
}

