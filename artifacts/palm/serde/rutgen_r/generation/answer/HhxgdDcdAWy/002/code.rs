// Answer 0

#[derive(Default)]
struct MockMap;

impl MockMap {
    fn serialize_value(&self, _content: &Content) -> Result<(), &'static str> {
        Ok(())
    }
}

struct Content {
    name: String,
    fields: Vec<String>,
}

impl Content {
    fn new(name: &str, fields: Vec<&str>) -> Self {
        Content {
            name: name.to_string(),
            fields: fields.iter().map(|&s| s.to_string()).collect(),
        }
    }
}

struct MockSerializer {
    map: MockMap,
    name: String,
    fields: Vec<String>,
}

impl MockSerializer {
    fn new(name: &str, fields: Vec<&str>) -> Self {
        MockSerializer {
            map: MockMap::default(),
            name: name.to_string(),
            fields: fields.iter().map(|&s| s.to_string()).collect(),
        }
    }

    fn end(self) -> Result<(), &'static str> {
        self.map.serialize_value(&Content::new(&self.name, self.fields))
    }
}

#[test]
fn test_end_success() {
    let serializer = MockSerializer::new("TestStruct", vec!["field1", "field2"]);
    assert_eq!(serializer.end(), Ok(()));
}

#[test]
fn test_end_empty_fields() {
    let serializer = MockSerializer::new("EmptyStruct", vec![]);
    assert_eq!(serializer.end(), Ok(()));
}

#[should_panic]
fn test_end_panics_on_failure() {
    struct FailingMap;

    impl FailingMap {
        fn serialize_value(&self, _content: &Content) -> Result<(), &'static str> {
            Err("Serialization failed")
        }
    }

    struct FailingSerializer {
        map: FailingMap,
        name: String,
        fields: Vec<String>,
    }

    impl FailingSerializer {
        fn new(name: &str, fields: Vec<&str>) -> Self {
            FailingSerializer {
                map: FailingMap,
                name: name.to_string(),
                fields: fields.iter().map(|&s| s.to_string()).collect(),
            }
        }

        fn end(self) -> Result<(), &'static str> {
            self.map.serialize_value(&Content::new(&self.name, self.fields))
        }
    }

    let failing_serializer = FailingSerializer::new("FailingStruct", vec!["field1"]);
    let _ = failing_serializer.end();
}

