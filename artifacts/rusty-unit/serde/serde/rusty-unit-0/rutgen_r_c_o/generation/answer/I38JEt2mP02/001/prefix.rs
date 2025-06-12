// Answer 0

#[derive(Debug)]
struct CustomError;

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Custom error occurred")
    }
}

impl std::error::Error for CustomError {}

struct FailingSerialize;

impl Serialize for FailingSerialize {
    fn serialize<S>(&self, _serializer: S) -> Result<Self::Ok, Self::Error>
    where
        S: Serializer,
    {
        Err(CustomError)
    }
}

struct TestSerializeSeq {
    elements: Vec<Content>,
}

impl SerializeSeq for TestSerializeSeq {
    type Ok = Content;
    type Error = CustomError;
    
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = tri!(value.serialize(ContentSerializer::<CustomError>::new()));
        self.elements.push(value);
        Ok(())
    }

    fn end(self) -> Result<Content, Self::Error> {
        Ok(Content::Seq(self.elements))
    }
}

#[test]
fn test_serialize_element_fails() {
    let mut seq = TestSerializeSeq {
        elements: Vec::new(),
    };
    let failing_value = FailingSerialize;

    let result = seq.serialize_element(&failing_value);
}

#[test]
fn test_serialize_empty_fail() {
    let mut seq = TestSerializeSeq {
        elements: Vec::new(),
    };
    let empty_value: &str = "";

    let result = seq.serialize_element(&empty_value);
}

#[test]
fn test_serialize_none_fails() {
    let mut seq = TestSerializeSeq {
        elements: Vec::new(),
    };
    
    let result = seq.serialize_element(&None::<&str>);
}

