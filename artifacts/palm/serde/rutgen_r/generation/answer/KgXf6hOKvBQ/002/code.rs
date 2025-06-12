// Answer 0

#[derive(Debug)]
struct MockMapAccess {
    entries: Vec<Result<(IgnoredAny, IgnoredAny), String>>,
    index: usize,
}

impl MockMapAccess {
    fn new(entries: Vec<Result<(IgnoredAny, IgnoredAny), String>>) -> Self {
        Self { entries, index: 0 }
    }
}

impl<'de> MapAccess<'de> for MockMapAccess {
    type Error = String;

    fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error>
    where
        K: Deserialize<'de>,
        V: Deserialize<'de>,
    {
        if self.index < self.entries.len() {
            let result = self.entries[self.index].clone();
            self.index += 1;
            Ok(result.map(|entry| (IgnoredAny, IgnoredAny)).transpose())
        } else {
            Ok(None)
        }
    }
}

#[test]
fn test_visit_map_err() {
    let mock_access = MockMapAccess::new(vec![
        Ok((IgnoredAny, IgnoredAny)),
        Err("An error occurred".to_string()),
    ]);
    
    let result = visit_map(mock_access);
    assert_eq!(result, Err("An error occurred".to_string()));
}

#[test]
fn test_visit_map_ok() {
    let mock_access = MockMapAccess::new(vec![
        Ok((IgnoredAny, IgnoredAny)),
        Ok((IgnoredAny, IgnoredAny)),
        Ok((IgnoredAny, IgnoredAny)),
    ]);

    let result = visit_map(mock_access);
    assert_eq!(result, Ok(()));
}

