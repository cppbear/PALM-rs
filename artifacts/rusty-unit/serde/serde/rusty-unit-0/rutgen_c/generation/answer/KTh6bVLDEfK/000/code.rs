// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    struct TestError;

    impl ser::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    #[test]
    fn test_end() {
        let name = "test_struct";
        let fields = vec![
            ("field1", Content::U32(42)),
            ("field2", Content::String("test".into())),
        ];

        let serialize_struct = SerializeStruct {
            name,
            fields,
            error: PhantomData::<TestError>,
        };

        let result = serialize_struct.end();
        assert!(result.is_ok());
        
        if let Ok(content) = result {
            match content {
                Content::Struct(struct_name, struct_fields) => {
                    assert_eq!(struct_name, name);
                    assert_eq!(struct_fields.len(), 2);
                    assert_eq!(struct_fields[0], ("field1", Content::U32(42)));
                    assert_eq!(struct_fields[1], ("field2", Content::String("test".into())));
                }
                _ => panic!("Expected a Content::Struct"),
            }
        }
    }

    #[test]
    fn test_end_empty() {
        let name = "empty_struct";
        let fields: Vec<(&'static str, Content)> = Vec::new();

        let serialize_struct = SerializeStruct {
            name,
            fields,
            error: PhantomData::<TestError>,
        };

        let result = serialize_struct.end();
        assert!(result.is_ok());
        
        if let Ok(content) = result {
            match content {
                Content::Struct(struct_name, struct_fields) => {
                    assert_eq!(struct_name, name);
                    assert!(struct_fields.is_empty());
                }
                _ => panic!("Expected a Content::Struct"),
            }
        }
    }
}

