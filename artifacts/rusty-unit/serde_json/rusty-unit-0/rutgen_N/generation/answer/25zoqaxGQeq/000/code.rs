// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_serialize_char() {
        let value: char = 'a';
        let expected = Value::String("a".to_string());
        let result = serialize_char(value);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_serialize_char_special() {
        let value: char = '!';
        let expected = Value::String("!".to_string());
        let result = serialize_char(value);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_serialize_char_unicode() {
        let value: char = '😊';
        let expected = Value::String("😊".to_string());
        let result = serialize_char(value);
        assert_eq!(result, Ok(expected));
    }
}

