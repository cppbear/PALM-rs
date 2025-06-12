// Answer 0

#[test]
fn test_serialize_f32_nan() {
    struct DummyFormatter;

    impl DummyFormatter {
        fn write_null(&self, _: &mut dyn std::io::Write) -> std::io::Result<()> {
            Ok(())
        }

        fn write_f32(&self, _: &mut dyn std::io::Write, _: f32) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct DummyWriter {
        data: Vec<u8>,
    }

    impl std::io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct Serializer {
        formatter: DummyFormatter,
        writer: DummyWriter,
    }

    impl Serializer {
        fn new() -> Self {
            Serializer {
                formatter: DummyFormatter,
                writer: DummyWriter { data: vec![] },
            }
        }

        fn serialize_f32(self, value: f32) -> Result<(), std::io::Error> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => 
                    self.formatter.write_null(&mut self.writer),
                _ =>
                    self.formatter.write_f32(&mut self.writer, value),
            }
        }
    }

    let serializer = Serializer::new();
    let result = serializer.serialize_f32(f32::NAN);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_f32_infinite() {
    struct DummyFormatter;

    impl DummyFormatter {
        fn write_null(&self, _: &mut dyn std::io::Write) -> std::io::Result<()> {
            Ok(())
        }

        fn write_f32(&self, _: &mut dyn std::io::Write, _: f32) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct DummyWriter {
        data: Vec<u8>,
    }

    impl std::io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct Serializer {
        formatter: DummyFormatter,
        writer: DummyWriter,
    }

    impl Serializer {
        fn new() -> Self {
            Serializer {
                formatter: DummyFormatter,
                writer: DummyWriter { data: vec![] },
            }
        }

        fn serialize_f32(self, value: f32) -> Result<(), std::io::Error> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => 
                    self.formatter.write_null(&mut self.writer),
                _ =>
                    self.formatter.write_f32(&mut self.writer, value),
            }
        }
    }

    let serializer = Serializer::new();
    let result = serializer.serialize_f32(f32::INFINITY);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_f32_finite() {
    struct DummyFormatter;

    impl DummyFormatter {
        fn write_null(&self, _: &mut dyn std::io::Write) -> std::io::Result<()> {
            Ok(())
        }

        fn write_f32(&self, _: &mut dyn std::io::Write, _: f32) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct DummyWriter {
        data: Vec<u8>,
    }

    impl std::io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct Serializer {
        formatter: DummyFormatter,
        writer: DummyWriter,
    }

    impl Serializer {
        fn new() -> Self {
            Serializer {
                formatter: DummyFormatter,
                writer: DummyWriter { data: vec![] },
            }
        }

        fn serialize_f32(self, value: f32) -> Result<(), std::io::Error> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => 
                    self.formatter.write_null(&mut self.writer),
                _ =>
                    self.formatter.write_f32(&mut self.writer, value),
            }
        }
    }

    let serializer = Serializer::new();
    let result = serializer.serialize_f32(42.0);
    assert!(result.is_ok());
}

