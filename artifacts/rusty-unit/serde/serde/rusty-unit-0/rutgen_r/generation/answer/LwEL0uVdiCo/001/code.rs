// Answer 0

#[derive(Debug)]
struct Content {
    entries: Vec<(String, String)>,
}

#[derive(Debug)]
struct Serializer {
    entries: Vec<(String, String)>,
}

impl Serializer {
    fn new() -> Self {
        Serializer { entries: vec![] }
    }

    fn add_entry(&mut self, key: String, value: String) {
        self.entries.push((key, value));
    }

    fn end(self) -> Result<Content, &'static str> {
        Ok(Content::Map(self.entries))
    }
}

#[test]
fn test_end_with_multiple_entries() {
    let serializer = Serializer::new();
    let mut serializer_with_entries = serializer;
    serializer_with_entries.add_entry("key1".to_string(), "value1".to_string());
    serializer_with_entries.add_entry("key2".to_string(), "value2".to_string());

    let result = serializer_with_entries.end();
    assert!(result.is_ok());
    if let Ok(content) = result {
        assert_eq!(content.entries.len(), 2);
        assert_eq!(content.entries[0], ("key1".to_string(), "value1".to_string()));
        assert_eq!(content.entries[1], ("key2".to_string(), "value2".to_string()));
    }
}

#[test]
fn test_end_with_no_entries() {
    let serializer = Serializer::new();
    let result = serializer.end();
    assert!(result.is_ok());
    if let Ok(content) = result {
        assert_eq!(content.entries.len(), 0);
    }
}

