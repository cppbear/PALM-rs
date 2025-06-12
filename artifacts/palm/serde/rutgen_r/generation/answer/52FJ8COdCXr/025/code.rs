// Answer 0

#[derive(Debug)]
enum Content {
    Seq(Vec<i32>),
    // Add other variants as needed for testing, omitted for brevity
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
    type Ok = ();
    type Error = serde::ser::Error;

    // Implement necessary methods
    // Here we'll provide basic implementations just for the sake of testing
    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        self.output.push("bool".to_string());
        Ok(())
    }
    
    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        self.output.push("u8".to_string());
        Ok(())
    }

    // Implement other serialize_x methods as needed...

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.output.push("seq".to_string());
        Ok(MockSerializeSeq { serializer: self })
    }
}

struct MockSerializeSeq {
    serializer: MockSerializer,
}

impl serde::ser::SerializeSeq for MockSerializeSeq {
    type Ok = ();
    type Error = serde::ser::Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error> 
    where T: serde::Serialize {
        self.serializer.output.push("element".to_string());
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.output.push("end_seq".to_string());
        Ok(())
    }
}

#[test]
fn test_serialize_seq_content() {
    let content = Content::Seq(vec![1, 2, 3]);
    let mut serializer = MockSerializer::new();
    
    let result = content.serialize(serializer);
    
    assert!(result.is_ok());
    assert_eq!(serializer.output, vec!["seq", "element", "element", "element", "end_seq"]);
}

#[test]
fn test_serialize_empty_seq_content() {
    let content = Content::Seq(vec![]);
    let mut serializer = MockSerializer::new();
    
    let result = content.serialize(serializer);
    
    assert!(result.is_ok());
    assert_eq!(serializer.output, vec!["seq", "end_seq"]);
}

