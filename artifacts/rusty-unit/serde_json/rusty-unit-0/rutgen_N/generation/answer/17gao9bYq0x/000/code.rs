// Answer 0

#[derive(Serialize)]
struct TestStruct {
    field1: String,
    field2: i32,
}

struct TestFormatter;

impl TestFormatter {
    fn begin_object_value(&mut self, writer: &mut Vec<u8>) -> std::io::Result<()> {
        writer.push(b'{');
        Ok(())
    }

    fn end_object_value(&mut self, writer: &mut Vec<u8>) -> std::io::Result<()> {
        writer.push(b'}');
        Ok(())
    }
}

struct TestSerializer {
    formatter: TestFormatter,
    writer: Vec<u8>,
}

impl TestSerializer {
    fn new() -> Self {
        Self {
            formatter: TestFormatter,
            writer: Vec::new(),
        }
    }
}

struct Compound {
    ser: TestSerializer,
}

impl Compound {
    fn new() -> Self {
        Self {
            ser: TestSerializer::new(),
        }
    }
}

#[test]
fn test_serialize_value() {
    let mut compound = Compound::new();
    let test_value = TestStruct {
        field1: String::from("test"),
        field2: 42,
    };
    
    let result = compound.ser.serialize_value(&test_value);
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(compound.ser.writer).unwrap(), r#"{"field1":"test","field2":42}"#);
}

