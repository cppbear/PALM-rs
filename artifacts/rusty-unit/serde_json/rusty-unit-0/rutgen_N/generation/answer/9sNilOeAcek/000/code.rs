// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use std::io::Cursor;

    #[test]
    fn test_to_writer_pretty_serializes_basic_struct() {
        #[derive(Serialize)]
        struct TestStruct {
            field1: String,
            field2: i32,
        }

        let value = TestStruct {
            field1: "test".to_string(),
            field2: 42,
        };
        
        let mut buffer = Vec::new();
        let writer = Cursor::new(&mut buffer);
        
        let result = to_writer_pretty(writer, &value);
        
        assert!(result.is_ok());
        let result_str = String::from_utf8(buffer).unwrap();
        assert!(result_str.contains("\"field1\": \"test\""));
        assert!(result_str.contains("\"field2\": 42"));
    }

    #[test]
    fn test_to_writer_pretty_serializes_empty_struct() {
        #[derive(Serialize)]
        struct EmptyStruct {}

        let value = EmptyStruct {};
        
        let mut buffer = Vec::new();
        let writer = Cursor::new(&mut buffer);
        
        let result = to_writer_pretty(writer, &value);
        
        assert!(result.is_ok());
        let result_str = String::from_utf8(buffer).unwrap();
        assert_eq!(result_str, "{}\n");
    }

    #[test]
    #[should_panic]
    fn test_to_writer_pretty_serializes_failed_case() {
        #[derive(Serialize)]
        struct NonSerializableStruct {
            key: std::collections::HashMap<String, i32>,
        }

        let mut test_map = std::collections::HashMap::new();
        test_map.insert("key".to_string(), 1);
        
        let value = NonSerializableStruct { key: test_map };
        
        let mut buffer = Vec::new();
        let writer = Cursor::new(&mut buffer);
        
        let _ = to_writer_pretty(writer, &value).unwrap();
    }
}

