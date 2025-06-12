// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Serialize, Deserialize};
    use std::io::Cursor;

    #[derive(Serialize)]
    struct SimpleStruct {
        x: i32,
        y: String,
    }

    #[test]
    fn test_to_writer_success() {
        let data = SimpleStruct {
            x: 42,
            y: "Hello".to_string(),
        };
        
        let mut buffer = Vec::new();
        let result = to_writer(&mut buffer, &data);

        assert!(result.is_ok());
        let expected = r#"{"x":42,"y":"Hello"}"#;
        assert_eq!(String::from_utf8(buffer).unwrap(), expected);
    }

    #[test]
    fn test_to_writer_invalid_utf8() {
        #[derive(Serialize)]
        struct InvalidUtf8Struct {
            key: String,
            value: Vec<u8>,
        }

        let data = InvalidUtf8Struct {
            key: "\u{FFFD}".to_string(), // Invalid UTF-8 character
            value: vec![255, 254, 253],
        };

        let mut buffer = Vec::new();
        let result = to_writer(&mut buffer, &data);

        assert!(result.is_ok()); // Expectation should be to check proper handling of binary data
    }
}

