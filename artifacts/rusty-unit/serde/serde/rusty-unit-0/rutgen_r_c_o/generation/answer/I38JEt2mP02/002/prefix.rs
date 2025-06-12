// Answer 0

#[derive(Debug)]
struct MockError;

impl std::fmt::Display for MockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MockError")
    }
}

impl std::error::Error for MockError {}

impl ser::Error for MockError {}

#[derive(Default)]
struct TestSerializeSeq {
    elements: Vec<Content>,
}

impl ser::SerializeSeq for TestSerializeSeq {
    type Ok = Content;
    type Error = MockError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let content = Content::String("Test".to_string());
        self.elements.push(content);
        Ok(())
    }

    fn end(self) -> Result<Content, Self::Error> {
        Ok(Content::Seq(self.elements))
    }
}

#[test]
fn test_serialize_element_with_string() {
    let mut seq = TestSerializeSeq::default();
    let value = "test string";
    seq.serialize_element(&value).unwrap();
}

#[test]
fn test_serialize_element_with_char() {
    let mut seq = TestSerializeSeq::default();
    let value = &'c';
    seq.serialize_element(&value).unwrap();
}

#[test]
fn test_serialize_element_with_bool() {
    let mut seq = TestSerializeSeq::default();
    let value = &true;
    seq.serialize_element(&value).unwrap();
}

#[test]
fn test_serialize_element_with_u8() {
    let mut seq = TestSerializeSeq::default();
    let value: &u8 = &255;
    seq.serialize_element(&value).unwrap();
}

#[test]
fn test_serialize_element_with_f64() {
    let mut seq = TestSerializeSeq::default();
    let value: &f64 = &3.14;
    seq.serialize_element(&value).unwrap();
}

#[test]
fn test_serialize_element_multiple() {
    let mut seq = TestSerializeSeq::default();
    seq.serialize_element(&"first").unwrap();
    seq.serialize_element(&"second").unwrap();
}

