// Answer 0

#[derive(Debug)]
enum Content {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Char(char),
    String(String),
    Bytes(Vec<u8>),
    None,
    Some(Box<Content>),
    Unit,
    UnitStruct(&'static str),
    UnitVariant(&'static str, u32, &'static str),
    NewtypeStruct(&'static str, Box<Content>),
    NewtypeVariant(&'static str, u32, &'static str, Box<Content>),
    Seq(Vec<Content>),
    Tuple(Vec<Content>),
    TupleStruct(&'static str, Vec<Content>),
    TupleVariant(&'static str, u32, &'static str, Vec<Content>),
    Map(Vec<(Content, Content)>),
    Struct(&'static str, Vec<(&'static str, Content)>),
    StructVariant(&'static str, u32, &'static str, Vec<(&'static str, Content)>),
}

struct MockSerializer {
    output: Vec<String>,
}

impl MockSerializer {
    fn new() -> Self {
        MockSerializer { output: Vec::new() }
    }
}

impl serde::ser::Serializer for MockSerializer {
    type Ok = Vec<String>;
    type Error = serde::ser::Error;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.output.push(format!("Bool: {}", v));
        Ok(self.output.clone())
    }
    
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.output.push(format!("U8: {}", v));
        Ok(self.output.clone())
    }
    
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.output.push(format!("UnitStruct: {}", name));
        Ok(self.output.clone())
    }

    // Add required methods from Serializer trait… (for brevity, not all methods are shown)
    // Implementing only enough to cover the tests.
    
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output.push(format!("String: {}", v));
        Ok(self.output.clone())
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::Ok, Self::Error> {
        self.output.push(format!("Tuple of length: {}", len));
        Ok(self.output.clone())
    }

    fn serialize_field<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize,
    {
        // Mock serialize field as a String representation
        self.output.push(format!("Field: {:?}", value));
        Ok(self.output.clone())
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::Ok, Self::Error> {
        self.output.push(format!("Map of length: {:?}", len));
        Ok(self.output.clone())
    }
    
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.output)
    }
    
    // ... (implement other methods as needed)
}

#[test]
fn test_serialize_bool() {
    let content = Content::Bool(true);
    let serializer = MockSerializer::new();
    let result = content.serialize(serializer).unwrap();
    assert_eq!(result, vec!["Bool: true"]);
}

#[test]
fn test_serialize_u8() {
    let content = Content::U8(255);
    let serializer = MockSerializer::new();
    let result = content.serialize(serializer).unwrap();
    assert_eq!(result, vec!["U8: 255"]);
}

#[test]
fn test_serialize_str() {
    let content = Content::String("Hello".to_string());
    let serializer = MockSerializer::new();
    let result = content.serialize(serializer).unwrap();
    assert_eq!(result, vec!["String: Hello"]);
}

#[test]
fn test_serialize_unit_struct() {
    let content = Content::UnitStruct("MyUnit");
    let serializer = MockSerializer::new();
    let result = content.serialize(serializer).unwrap();
    assert_eq!(result, vec!["UnitStruct: MyUnit"]);
}

#[test]
fn test_serialize_tuple() {
    let content = Content::Tuple(vec![Content::U8(1), Content::U8(2)]);
    let serializer = MockSerializer::new();
    let result = content.serialize(serializer).unwrap();
    assert_eq!(result[0], "Tuple of length: 2");
}

#[test]
fn test_serialize_map() {
    let content = Content::Map(vec![(Content::String("key".to_string()), Content::U32(42))]);
    let serializer = MockSerializer::new();
    let result = content.serialize(serializer).unwrap();
    assert_eq!(result[0], "Map of length: Some(1)");
}

