// Answer 0

#[derive(Debug)]
struct DummyMapAccess {
    entries: Vec<(IgnoredAny, IgnoredAny)>,
    index: usize,
}

impl<'de> MapAccess<'de> for DummyMapAccess {
    type Error = ();

    fn next_entry(&mut self) -> Option<Result<(IgnoredAny, IgnoredAny), Self::Error>> {
        if self.index < self.entries.len() {
            let entry = &self.entries[self.index];
            self.index += 1;
            Some(Ok(entry.clone()))
        } else {
            None
        }
    }
}

#[test]
fn test_visit_map_no_entries() {
    let dummy_map = DummyMapAccess {
        entries: vec![],
        index: 0,
    };
    let result: Result<IgnoredAny, ()> = visit_map(dummy_map);
    assert_eq!(result, Ok(IgnoredAny));
}

#[test]
fn test_visit_map_with_entries() {
    let dummy_map = DummyMapAccess {
        entries: vec![(IgnoredAny, IgnoredAny)],
        index: 0,
    };
    let result: Result<IgnoredAny, ()> = visit_map(dummy_map);
    assert_eq!(result, Ok(IgnoredAny));
}

