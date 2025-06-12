// Answer 0

#[derive(Debug)]
struct TestSerializer {
    output: Vec<(String, String)>,
}

impl TestSerializer {
    fn new() -> Self {
        TestSerializer { output: Vec::new() }
    }
}

trait Serializer {
    type Ok;
    type Error;

    fn serialize_map(self, len: Option<usize>) -> Result<Box<dyn Map<Self>>, Self::Error>;
}

trait Map<S: Serializer> {
    fn serialize_entry(&mut self, key: &str, value: &str) -> Result<(), S::Error>;
    fn end(self) -> Result<S::Ok, S::Error>;
}

impl Serializer for TestSerializer {
    type Ok = Vec<(String, String)>;
    type Error = &'static str;

    fn serialize_map(self, _: Option<usize>) -> Result<Box<dyn Map<Self>>, Self::Error> {
        Ok(Box::new(TestMap { serializer: self }))
    }
}

struct TestMap {
    serializer: TestSerializer,
}

impl<S: Serializer> Map<S> for TestMap {
    fn serialize_entry(&mut self, key: &str, value: &str) -> Result<(), S::Error> {
        self.serializer.output.push((key.to_string(), value.to_string()));
        Ok(())
    }

    fn end(self) -> Result<S::Ok, S::Error> {
        Ok(self.serializer.output)
    }
}

fn serialize_unit_struct(serializer: TestSerializer, tag: &'static str, variant_name: &'static str) -> Result<TestSerializer::Ok, TestSerializer::Error> {
    let mut map = serializer.serialize_map(Some(1))?;
    map.serialize_entry(tag, variant_name)?;
    map.end()
}

#[test]
fn test_serialize_unit_struct() {
    let serializer = TestSerializer::new();
    let result = serialize_unit_struct(serializer, "tag", "variant_name").unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], ("tag".to_string(), "variant_name".to_string()));
}

