// Answer 0

#[derive(Debug)]
struct MyMapAccess {
    entries: Vec<(IgnoredAny, IgnoredAny)>,
    index: usize,
}

impl<'de> MapAccess<'de> for MyMapAccess {
    type Error = serde::de::Error;

    fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error>
    where
        K: Deserialize<'de>,
        V: Deserialize<'de>,
    {
        if self.index < self.entries.len() {
            let entry = &self.entries[self.index];
            self.index += 1;
            Ok(Some((entry.0.clone(), entry.1.clone())))
        } else {
            Ok(None)
        }
    }
}

#[test]
fn test_visit_map_with_entries() {
    let access = MyMapAccess {
        entries: vec![(IgnoredAny, IgnoredAny), (IgnoredAny, IgnoredAny)],
        index: 0,
    };
    let result = visit_map(access);
    assert!(result.is_ok());
}

#[test]
fn test_visit_map_with_no_entries() {
    let access = MyMapAccess {
        entries: vec![],
        index: 0,
    };
    let result = visit_map(access);
    assert!(result.is_ok());
}

