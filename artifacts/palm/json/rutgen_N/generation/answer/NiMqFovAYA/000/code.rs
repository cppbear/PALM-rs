// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    struct TestSerializer {
        vec: Vec<serde_json::Value>,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer { vec: Vec::new() }
        }

        fn serialize_field<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            self.vec.push(serde_json::to_value(value)?);
            Ok(())
        }
    }

    #[test]
    fn test_serialize_field_integer() {
        let mut serializer = TestSerializer::new();
        let value = 42;
        let result = serializer.serialize_field(&value);
        assert!(result.is_ok());
        assert_eq!(serializer.vec.len(), 1);
        assert_eq!(serializer.vec[0], serde_json::json!(42));
    }

    #[test]
    fn test_serialize_field_string() {
        let mut serializer = TestSerializer::new();
        let value = "hello";
        let result = serializer.serialize_field(&value);
        assert!(result.is_ok());
        assert_eq!(serializer.vec.len(), 1);
        assert_eq!(serializer.vec[0], serde_json::json!("hello"));
    }

    #[test]
    fn test_serialize_field_struct() {
        #[derive(Serialize)]
        struct TestStruct {
            field: i32,
        }

        let mut serializer = TestSerializer::new();
        let value = TestStruct { field: 10 };
        let result = serializer.serialize_field(&value);
        assert!(result.is_ok());
        assert_eq!(serializer.vec.len(), 1);
        assert_eq!(serializer.vec[0], serde_json::json!({ "field": 10 }));
    }

    #[test]
    fn test_serialize_field_empty_vec() {
        let mut serializer = TestSerializer::new();
        let value: Vec<i32> = vec![];
        let result = serializer.serialize_field(&value);
        assert!(result.is_ok());
        assert_eq!(serializer.vec.len(), 1);
        assert_eq!(serializer.vec[0], serde_json::json!([]));
    }
}

