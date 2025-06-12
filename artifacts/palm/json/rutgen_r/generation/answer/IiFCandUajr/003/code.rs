// Answer 0

#[test]
fn test_serialize_nan() {
    struct MockFormatter {
        called_write_null: bool,
        called_write_f64: bool,
    }

    impl MockFormatter {
        fn new() -> Self {
            Self {
                called_write_null: false,
                called_write_f64: false,
            }
        }

        fn write_null(&mut self, _writer: &mut std::io::Write) -> Result<(), std::io::Error> {
            self.called_write_null = true;
            Ok(())
        }

        fn write_f64(&mut self, _writer: &mut std::io::Write, _value: f64) -> Result<(), std::io::Error> {
            self.called_write_f64 = true;
            Ok(())
        }
    }

    struct Serializer {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    impl Serializer {
        fn new() -> Self {
            Self {
                formatter: MockFormatter::new(),
                writer: Vec::new(),
            }
        }

        fn serialize_f64(self, value: f64) -> Result<(), std::io::Error> {
            match value.classify() {
                std::num::FpCategory::NaN | std::num::FpCategory::Infinite => {
                    self.formatter.write_null(&mut self.writer)
                },
                _ => {
                    self.formatter.write_f64(&mut self.writer, value)
                },
            }
        }
    }

    let serializer = Serializer::new();
    let result = serializer.serialize_f64(std::f64::NAN);

    assert!(result.is_ok());
    assert!(serializer.formatter.called_write_null);
    assert!(!serializer.formatter.called_write_f64);
}

#[test]
fn test_serialize_infinite() {
    struct MockFormatter {
        called_write_null: bool,
        called_write_f64: bool,
    }

    impl MockFormatter {
        fn new() -> Self {
            Self {
                called_write_null: false,
                called_write_f64: false,
            }
        }

        fn write_null(&mut self, _writer: &mut std::io::Write) -> Result<(), std::io::Error> {
            self.called_write_null = true;
            Ok(())
        }

        fn write_f64(&mut self, _writer: &mut std::io::Write, _value: f64) -> Result<(), std::io::Error> {
            self.called_write_f64 = true;
            Ok(())
        }
    }

    struct Serializer {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    impl Serializer {
        fn new() -> Self {
            Self {
                formatter: MockFormatter::new(),
                writer: Vec::new(),
            }
        }

        fn serialize_f64(self, value: f64) -> Result<(), std::io::Error> {
            match value.classify() {
                std::num::FpCategory::NaN | std::num::FpCategory::Infinite => {
                    self.formatter.write_null(&mut self.writer)
                },
                _ => {
                    self.formatter.write_f64(&mut self.writer, value)
                },
            }
        }
    }

    let serializer = Serializer::new();
    let result = serializer.serialize_f64(std::f64::INFINITY);

    assert!(result.is_ok());
    assert!(serializer.formatter.called_write_null);
    assert!(!serializer.formatter.called_write_f64);
}

