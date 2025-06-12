// Answer 0

#[test]
fn test_serialize_newtype_struct_string() {
    use serde::ser::{Serializer, Serialize};
    use serde_json::{Value, Serializer as JsonSerializer};

    struct NewtypeStruct<'a>(&'a str);

    impl<'a> Serialize for NewtypeStruct<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(self.0)
        }
    }

    let value = NewtypeStruct("test");
    let mut serializer = JsonSerializer::new(serde_json::Serializer::new());

    let result = serializer.serialize_newtype_struct("NewtypeStruct", &value).unwrap();
    assert_eq!(result, Value::String("test".to_string()));
}

#[test]
fn test_serialize_newtype_struct_empty_string() {
    use serde::ser::{Serializer, Serialize};
    use serde_json::{Value, Serializer as JsonSerializer};

    struct NewtypeStruct<'a>(&'a str);

    impl<'a> Serialize for NewtypeStruct<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(self.0)
        }
    }

    let value = NewtypeStruct("");
    let mut serializer = JsonSerializer::new(serde_json::Serializer::new());

    let result = serializer.serialize_newtype_struct("NewtypeStruct", &value).unwrap();
    assert_eq!(result, Value::String("".to_string()));
}

