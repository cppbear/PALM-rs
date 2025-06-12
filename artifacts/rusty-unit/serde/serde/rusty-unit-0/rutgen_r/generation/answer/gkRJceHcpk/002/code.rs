// Answer 0

#[test]
fn test_serialize_field_success() {
    use serde::Serialize;
    use serde::ser::{Serializer, SerializeStruct};

    struct MyStruct {
        field1: String,
    }

    impl Serialize for MyStruct {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("MyStruct", 1)?;
            state.serialize_field("field1", &self.field1)?;
            state.end()
        }
    }

    struct TestSerializer {
        fields: Vec<(&'static str, String)>,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                fields: Vec::new(),
            }
        }

        fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), String>
        where
            T: ?Sized + Serialize,
        {
            let value = value.serialize(ContentSerializer::new()).map_err(|_| "Serialization Error".to_string())?;
            self.fields.push((key, value));
            Ok(())
        }
    }

    struct ContentSerializer;

    impl ContentSerializer {
        fn new() -> Self {
            ContentSerializer
        }
    }

    let mut serializer = TestSerializer::new();
    let my_struct = MyStruct {
        field1: String::from("test_value"),
    };

    let result = serializer.serialize_field("my_key", &my_struct);
    assert_eq!(result, Ok(()));
}

