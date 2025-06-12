// Answer 0

#[derive(Debug)]
struct TestKey;
#[derive(Debug)]
struct TestValue;

#[derive(Debug)]
struct TestMap {
    entries: Vec<(TestKey, TestValue)>,
}

impl TestMap {
    pub fn from_slice(entries: &[(TestKey, TestValue)]) -> Self {
        Self {
            entries: entries.to_vec(),
        }
    }

    pub fn split_last(&self) -> Option<((&TestKey, &TestValue), &Self)> {
        if let [rest @ .., last] = &self.entries[..] {
            Some((last.refs(), Self::from_slice(rest)))
        } else {
            None
        }
    }
}

impl TestMap {
    fn refs(&self) -> (&TestKey, &TestValue) {
        let last_entry = self.entries.last().unwrap();
        (&last_entry.0, &last_entry.1)
    }
}

#[test]
fn test_split_last_non_empty() {
    let map = TestMap {
        entries: vec![(TestKey, TestValue), (TestKey, TestValue)],
    };

    let result = map.split_last();
    assert!(result.is_some());

    let ((key, value), rest) = result.unwrap();
    assert_eq!(*key, TestKey);
    assert_eq!(*value, TestValue);
    assert_eq!(rest.entries.len(), 1);
}

#[test]
fn test_split_last_empty() {
    let map = TestMap {
        entries: Vec::new(),
    };

    let result = map.split_last();
    assert!(result.is_none());
}

