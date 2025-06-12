// Answer 0

fn test_serialize_f64_infinite() {
    struct MockFormatter;

    impl MockFormatter {
        fn write_null(&self, _: &mut dyn std::io::Write) -> std::io::Result<()> {
            Ok(())
        }

        fn write_f64(&self, _: &mut dyn std::io::Write, _: f64) -> std::io::Result<()> {
            panic!("write_f64 should not be called");
        }
    }

    struct MockWriter;

    impl std::io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct Serializer<'a> {
        writer: &'a mut dyn std::io::Write,
        formatter: MockFormatter,
    }

    impl<'a> Serializer<'a> {
        fn serialize_f64(self, value: f64) -> std::io::Result<()> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => {
                    self.formatter.write_null(&mut self.writer)
                }
                _ => self.formatter.write_f64(&mut self.writer, value),
            }
        }
    }

    let mut writer = MockWriter;
    let serializer = Serializer {
        writer: &mut writer,
        formatter: MockFormatter,
    };
    let result = serializer.serialize_f64(f64::INFINITY);
    assert!(result.is_ok());
}

fn test_serialize_f64_nan() {
    struct MockFormatter;

    impl MockFormatter {
        fn write_null(&self, _: &mut dyn std::io::Write) -> std::io::Result<()> {
            Ok(())
        }

        fn write_f64(&self, _: &mut dyn std::io::Write, _: f64) -> std::io::Result<()> {
            panic!("write_f64 should not be called");
        }
    }

    struct MockWriter;

    impl std::io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct Serializer<'a> {
        writer: &'a mut dyn std::io::Write,
        formatter: MockFormatter,
    }

    impl<'a> Serializer<'a> {
        fn serialize_f64(self, value: f64) -> std::io::Result<()> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => {
                    self.formatter.write_null(&mut self.writer)
                }
                _ => self.formatter.write_f64(&mut self.writer, value),
            }
        }
    }

    let mut writer = MockWriter;
    let serializer = Serializer {
        writer: &mut writer,
        formatter: MockFormatter,
    };
    let result = serializer.serialize_f64(f64::NAN);
    assert!(result.is_ok());
}

