// Answer 0

#[test]
fn test_serialize_i8_negative() {
    let mut writer = Vec::new();
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    serializer.serialize_i8(-128);
}

#[test]
fn test_serialize_i8_negative_one() {
    let mut writer = Vec::new();
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    serializer.serialize_i8(-1);
}

#[test]
fn test_serialize_i8_zero() {
    let mut writer = Vec::new();
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    serializer.serialize_i8(0);
}

#[test]
fn test_serialize_i8_one() {
    let mut writer = Vec::new();
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    serializer.serialize_i8(1);
}

#[test]
fn test_serialize_i8_positive_max() {
    let mut writer = Vec::new();
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    serializer.serialize_i8(127);
}

struct MockFormatter;

impl Formatter for MockFormatter {
    fn write_i8(&mut self, _writer: &mut dyn Write, _value: i8) -> Result<()> {
        Ok(())
    }
}

