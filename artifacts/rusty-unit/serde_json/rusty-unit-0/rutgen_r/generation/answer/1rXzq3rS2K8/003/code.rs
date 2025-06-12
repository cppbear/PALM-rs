// Answer 0

#[derive(Default)]
struct MockWriter;

struct Formatter<'a> {
    writer: &'a mut MockWriter,
}

impl<'a> Formatter<'a> {
    fn begin_string(&mut self) -> Result<(), ()> {
        // Simulate successful operation
        Ok(())
    }

    fn write_f32(&mut self, value: f32) -> Result<(), ()> {
        // Simulate successful operation
        Ok(())
    }

    fn end_string(&mut self) -> Result<(), ()> {
        // Simulate successful operation
        Ok(())
    }
}

struct Serializer<'a> {
    formatter: Formatter<'a>,
}

impl<'a> Serializer<'a> {
    fn new(writer: &'a mut MockWriter) -> Self {
        Serializer {
            formatter: Formatter { writer },
        }
    }

    fn serialize_f32(self, value: f32) -> Result<(), ()> {
        if !value.is_finite() {
            return Err(());
        }

        self.formatter.begin_string().map_err(|_| ())?;
        self.formatter.write_f32(value).map_err(|_| ())?;
        self.formatter.end_string().map_err(|_| ())
    }
}

#[test]
fn test_serialize_f32_valid() {
    let mut writer = MockWriter::default();
    let serializer = Serializer::new(&mut writer);

    assert!(serializer.serialize_f32(1.0).is_ok());
}

#[test]
fn test_serialize_f32_valid_negative() {
    let mut writer = MockWriter::default();
    let serializer = Serializer::new(&mut writer);

    assert!(serializer.serialize_f32(-1.0).is_ok());
}

#[test]
#[should_panic]
fn test_serialize_f32_infinite() {
    let mut writer = MockWriter::default();
    let serializer = Serializer::new(&mut writer);

    assert!(serializer.serialize_f32(f32::INFINITY).is_err());
}

#[test]
#[should_panic]
fn test_serialize_f32_nan() {
    let mut writer = MockWriter::default();
    let serializer = Serializer::new(&mut writer);

    assert!(serializer.serialize_f32(f32::NAN).is_err());
}

