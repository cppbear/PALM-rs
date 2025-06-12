// Answer 0

#[derive(Default)]
struct Writer;

#[derive(Default)]
struct Formatter;

#[derive(Default)]
struct Ser {
    writer: Writer,
    formatter: Formatter,
}

impl Formatter {
    fn begin_string(&self, _writer: &mut Writer) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn write_u8(&self, _writer: &mut Writer, _value: u8) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn end_string(&self, _writer: &mut Writer) -> Result<(), std::io::Error> {
        Ok(())
    }
}

#[derive(Default)]
struct MySerializer {
    ser: Ser,
}

impl MySerializer {
    fn serialize_u8(self, value: u8) -> Result<(), std::io::Error> {
        self.ser.formatter.begin_string(&mut self.ser.writer)?;
        self.ser.formatter.write_u8(&mut self.ser.writer, value)?;
        self.ser.formatter.end_string(&mut self.ser.writer)?;
        Ok(())
    }
}

#[test]
fn test_serialize_u8_success() {
    let serializer = MySerializer::default();
    let result = serializer.serialize_u8(42);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_u8_zero() {
    let serializer = MySerializer::default();
    let result = serializer.serialize_u8(0);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_u8_max_value() {
    let serializer = MySerializer::default();
    let result = serializer.serialize_u8(255);
    assert!(result.is_ok());
}

