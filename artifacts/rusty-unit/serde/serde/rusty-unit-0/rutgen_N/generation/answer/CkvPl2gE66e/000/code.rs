// Answer 0

#[test]
fn test_map_deserializer_end_with_no_remaining_elements() {
    struct MockDeserializer {
        iter: Vec<(String, String)>,
        count: usize,
    }

    impl MockDeserializer {
        fn end(self) -> Result<(), String> {
            let remaining = self.iter.len();
            if remaining == 0 {
                Ok(())
            } else {
                Err(format!("invalid length: expected {}, found {}", self.count + remaining, self.count))
            }
        }
    }

    let deserializer = MockDeserializer {
        iter: vec![],
        count: 0,
    };

    assert!(deserializer.end().is_ok());
}

#[test]
fn test_map_deserializer_end_with_remaining_elements() {
    struct MockDeserializer {
        iter: Vec<(String, String)>,
        count: usize,
    }

    impl MockDeserializer {
        fn end(self) -> Result<(), String> {
            let remaining = self.iter.len();
            if remaining == 0 {
                Ok(())
            } else {
                Err(format!("invalid length: expected {}, found {}", self.count + remaining, self.count))
            }
        }
    }

    let deserializer = MockDeserializer {
        iter: vec![("key1".to_string(), "value1".to_string())],
        count: 0,
    };

    let result = deserializer.end();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "invalid length: expected 1, found 0");
}

