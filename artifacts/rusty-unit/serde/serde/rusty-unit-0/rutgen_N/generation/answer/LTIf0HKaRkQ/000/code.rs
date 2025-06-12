// Answer 0

#[derive(Debug)]
struct MockMap;

impl MockMap {
    fn serialize_value(&self, _content: &Content) -> Result<(), String> {
        Ok(())
    }

    fn end(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug)]
enum Content {
    TupleStruct(String, Vec<String>),
}

struct Serializer<M> {
    map: M,
    name: String,
    fields: Vec<String>,
}

impl<M> Serializer<M> where M: Map {
    fn end(mut self) -> Result<M::Ok, M::Error> {
        tri!(self.map.serialize_value(&Content::TupleStruct(self.name, self.fields)));
        self.map.end()
    }
}

trait Map {
    type Ok;
    type Error;

    fn serialize_value(&self, content: &Content) -> Result<Self::Ok, Self::Error>;
    fn end(&self) -> Result<Self::Ok, Self::Error>;
}

#[test]
fn test_end_success() {
    let mock_map = MockMap;
    let serializer = Serializer {
        map: mock_map,
        name: "Test".to_string(),
        fields: vec!["field1".to_string(), "field2".to_string()],
    };

    let result: Result<(), String> = serializer.end();
    assert!(result.is_ok());
}

