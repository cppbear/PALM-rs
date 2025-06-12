// Answer 0

#[derive(Debug)]
struct MockMapAccess<'de> {
    entries: Vec<Result<(IgnoredAny, IgnoredAny), &'de str>>,
    index: usize,
}

impl<'de> MapAccess<'de> for MockMapAccess<'de> {
    type Error = &'de str;

    fn next_entry(&mut self) -> Result<Option<(IgnoredAny, IgnoredAny)>, Self::Error> {
        if self.index < self.entries.len() {
            let result = self.entries[self.index].clone();
            self.index += 1;
            Ok(Some(result.unwrap()))
        } else {
            Ok(None)
        }
    }
}

#[test]
fn test_visit_map_success() {
    let entries = vec![
        Ok((IgnoredAny, IgnoredAny)), 
        Ok((IgnoredAny, IgnoredAny)), 
    ];
    let mut map = MockMapAccess { entries, index: 0 };
    
    let result: Result<IgnoredAny, &str> = visit_map(map);
    assert_eq!(result, Ok(IgnoredAny));
}

#[test]
fn test_visit_map_err() {
    let entries = vec![
        Ok((IgnoredAny, IgnoredAny)), 
        Err("error"),
    ];
    let mut map = MockMapAccess { entries, index: 0 };
    
    let result: Result<IgnoredAny, &str> = visit_map(map);
    assert_eq!(result, Err("error"));
}

