// Answer 0

#[derive(Debug)]
struct Formatter;
#[derive(Debug)]
struct Writer;

impl Formatter {
    fn write_null(&self, _writer: &mut Writer) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn write_f64(&self, _writer: &mut Writer, _value: f64) -> Result<(), std::io::Error> {
        Ok(())
    }
}

struct SerializeF64<'a> {
    formatter: &'a Formatter,
    writer: &'a mut Writer,
}

impl<'a> SerializeF64<'a> {
    fn serialize_f64(self, value: f64) -> Result<(), std::io::Error> {
        match value.classify() {
            std::num::FpCategory::Nan | std::num::FpCategory::Infinite => {
                self.formatter.write_null(self.writer)
            }
            _ => self.formatter.write_f64(self.writer, value),
        }
    }
}

#[test]
fn test_serialize_f64_normal() {
    let formatter = Formatter;
    let mut writer = Writer;

    let serializer = SerializeF64 {
        formatter: &formatter,
        writer: &mut writer,
    };

    // Testing with a normal finite number
    let result = serializer.serialize_f64(3.14);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_f64_negative_normal() {
    let formatter = Formatter;
    let mut writer = Writer;

    let serializer = SerializeF64 {
        formatter: &formatter,
        writer: &mut writer,
    };

    // Testing with a normal negative finite number
    let result = serializer.serialize_f64(-2.718);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_f64_zero() {
    let formatter = Formatter;
    let mut writer = Writer;

    let serializer = SerializeF64 {
        formatter: &formatter,
        writer: &mut writer,
    };

    // Testing with zero
    let result = serializer.serialize_f64(0.0);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_f64_subnormal() {
    let formatter = Formatter;
    let mut writer = Writer;

    let serializer = SerializeF64 {
        formatter: &formatter,
        writer: &mut writer,
    };

    // Testing with a subnormal number
    let result = serializer.serialize_f64(1.0e-300);
    assert!(result.is_ok());
}

