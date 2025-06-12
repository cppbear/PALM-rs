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
        Serializer {
            entries: Vec::new(),
        }
    }

    fn end(self) -> Result<Content, &'static str> {
        Ok(Content { entries: self.entries })
    }
}

#[test]
fn test_end_success() {
    let serializer = Serializer::new();
    let result = serializer.end();
    
    assert!(result.is_ok());
    let content = result.unwrap();
    assert_eq!(content.entries, Vec::<(String, String)>::new());
}

#[test]
fn test_end_with_entries() {
    let mut serializer = Serializer::new();
    serializer.entries.push(("key1".to_string(), "value1".to_string()));
    let result = serializer.end();

    assert!(result.is_ok());
    let content = result.unwrap();
    assert_eq!(content.entries.len(), 1);
    assert_eq!(content.entries[0], ("key1".to_string(), "value1".to_string()));
}

