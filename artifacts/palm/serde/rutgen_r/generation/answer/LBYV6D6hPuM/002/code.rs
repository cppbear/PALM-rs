// Answer 0

#[derive(Debug)]
struct MockDelegate {
    serialize_map_return: Result<MockMap, String>,
}

impl MockDelegate {
    fn serialize_map(&self, _: Option<usize>) -> Result<MockMap, String> {
        self.serialize_map_return.clone()
    }
}

#[derive(Debug)]
struct MockMap {
    entries: Vec<(String, String)>,
    ended: bool,
}

impl MockMap {
    fn new() -> Self {
        MockMap {
            entries: vec![],
            ended: false,
        }
    }

    fn serialize_entry(&mut self, key: String, value: String) -> Result<(), String> {
        if self.ended {
            return Err("map already ended".to_string());
        }
        self.entries.push((key, value));
        Ok(())
    }

    fn end(self) -> Result<(), String> {
        self.ended = true;
        Ok(())
    }
}

struct TestStruct<'a> {
    delegate: MockDelegate,
    tag: String,
    variant_name: &'a str,
}

impl TestStruct<'_> {
    fn serialize_unit(self) -> Result<(), String> {
        let mut map = self.delegate.serialize_map(Some(1))?;
        map.serialize_entry(self.tag, self.variant_name)?;
        map.end()
    }
}

#[test]
fn test_serialize_unit_success() {
    let mock_map = MockMap::new();
    let delegate = MockDelegate {
        serialize_map_return: Ok(mock_map),
    };

    let test_struct = TestStruct {
        delegate,
        tag: "key".to_string(),
        variant_name: "value",
    };

    let result = test_struct.serialize_unit();
    assert_eq!(result, Ok(()));
}

#[test]
fn test_serialize_unit_serialize_map_error() {
    let delegate = MockDelegate {
        serialize_map_return: Err("failed to serialize map".to_string()),
    };

    let test_struct = TestStruct {
        delegate,
        tag: "key".to_string(),
        variant_name: "value",
    };

    let result = test_struct.serialize_unit();
    assert_eq!(result, Err("failed to serialize map".to_string()));
}

#[test]
fn test_serialize_unit_entry_error() {
    let mut mock_map = MockMap::new();
    let delegate = MockDelegate {
        serialize_map_return: Ok(mock_map),
    };

    let test_struct = TestStruct {
        delegate,
        tag: "key".to_string(),
        variant_name: "value",
    };

    // Simulate map already ended to trigger entry error
    mock_map.end().unwrap();

    let result = test_struct.serialize_unit();
    assert_eq!(result, Err("map already ended".to_string()));
}

