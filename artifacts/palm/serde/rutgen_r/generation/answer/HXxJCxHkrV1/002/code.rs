// Answer 0

#[derive(Debug)]
struct MockMap;

impl MockMap {
    fn serialize_value(&self, _content: &Content) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug)]
struct Content {
    fields: Vec<i32>,
}

impl Content {
    fn new(fields: Vec<i32>) -> Self {
        Content { fields }
    }
}

#[derive(Debug)]
struct TestStruct {
    map: MockMap,
    fields: Vec<i32>,
}

impl TestStruct {
    fn new(map: MockMap, fields: Vec<i32>) -> Self {
        TestStruct { map, fields }
    }

    fn end(self) -> Result<(), String> {
        self.map.serialize_value(&Content::new(self.fields))
    }
}

#[test]
fn test_end_success() {
    let map = MockMap;
    let fields = vec![1, 2, 3];
    let test_struct = TestStruct::new(map, fields);
    let result = test_struct.end();
    assert_eq!(result, Ok(()));
}

#[test]
fn test_end_empty_fields() {
    let map = MockMap;
    let fields: Vec<i32> = vec![];
    let test_struct = TestStruct::new(map, fields);
    let result = test_struct.end();
    assert_eq!(result, Ok(()));
}

#[test]
fn test_end_large_fields() {
    let map = MockMap;
    let fields: Vec<i32> = (0..1000).collect();
    let test_struct = TestStruct::new(map, fields);
    let result = test_struct.end();
    assert_eq!(result, Ok(()));
}

