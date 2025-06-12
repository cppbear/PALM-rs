// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use serde_json::ser::Serializer;

    #[test]
    fn test_serialize_some_with_valid_data() {
        struct SimpleStruct {
            name: String,
            age: u32,
        }

        let value = SimpleStruct {
            name: String::from("Alice"),
            age: 30,
        };

        let mut serializer = Serializer::new(vec![]);
        let result = serializer.serialize_some(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn test_serialize_some_with_empty_string() {
        struct SimpleStruct {
            name: String,
        }

        let value = SimpleStruct {
            name: String::new(),
        };

        let mut serializer = Serializer::new(vec![]);
        let result = serializer.serialize_some(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn test_serialize_some_with_nested_struct() {
        #[derive(Serialize)]
        struct NestedStruct {
            data: String,
        }

        #[derive(Serialize)]
        struct OuterStruct {
            inner: NestedStruct,
        }

        let value = OuterStruct {
            inner: NestedStruct {
                data: String::from("Nested Data"),
            },
        };

        let mut serializer = Serializer::new(vec![]);
        let result = serializer.serialize_some(&value);
        assert!(result.is_ok());
    }

    #[should_panic]
    #[test]
    fn test_serialize_some_with_unserializable_data() {
        struct UnserializableStruct;

        let value = UnserializableStruct;

        let mut serializer = Serializer::new(vec![]);
        let _result = serializer.serialize_some(&value); 
    }
}

