// Answer 0

#[derive(Debug)]
struct ContentDeserializer {
    data: String,
}

impl ContentDeserializer {
    fn new(data: String) -> Self {
        ContentDeserializer { data }
    }
}

struct TestStruct {
    value: String,
}

impl TestStruct {
    fn into_deserializer(self) -> ContentDeserializer {
        ContentDeserializer::new(self.value)
    }
}

#[test]
fn test_into_deserializer() {
    let test_instance = TestStruct {
        value: String::from("test data"),
    };
    let deserializer = test_instance.into_deserializer();
    assert_eq!(deserializer.data, "test data");
}

