// Answer 0

#[derive(Debug, Deserialize)]
struct TestKey {
    id: i32,
}

#[derive(Debug, Deserialize)]
struct TestValue {
    name: String,
}

struct TestMapAccess {
    entries: Vec<(TestKey, TestValue)>,
    index: usize,
}

impl TestMapAccess {
    fn new(entries: Vec<(TestKey, TestValue)>) -> Self {
        TestMapAccess { entries, index: 0 }
    }

    fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, ()>
    where
        K: Deserialize<'static>,
        V: Deserialize<'static>,
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
fn test_next_entry_some() {
    let mut map_access = TestMapAccess::new(vec![
        (TestKey { id: 1 }, TestValue { name: "Value1".to_string() }),
        (TestKey { id: 2 }, TestValue { name: "Value2".to_string() }),
    ]);

    assert_eq!(
        map_access.next_entry::<TestKey, TestValue>().unwrap(),
        Some((TestKey { id: 1 }, TestValue { name: "Value1".to_string() }))
    );

    assert_eq!(
        map_access.next_entry::<TestKey, TestValue>().unwrap(),
        Some((TestKey { id: 2 }, TestValue { name: "Value2".to_string() }))
    );
}

#[test]
fn test_next_entry_none() {
    let mut map_access = TestMapAccess::new(vec![]);

    assert_eq!(map_access.next_entry::<TestKey, TestValue>().unwrap(), None);
}

