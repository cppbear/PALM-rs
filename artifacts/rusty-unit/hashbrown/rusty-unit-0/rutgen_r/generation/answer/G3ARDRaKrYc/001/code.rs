// Answer 0

#[derive(Debug)]
struct TestHashTable {
    data: Vec<(u64, String)>,
}

impl TestHashTable {
    fn new() -> Self {
        TestHashTable {
            data: vec![],
        }
    }

    fn insert(&mut self, hash: u64, value: String) {
        self.data.push((hash, value));
    }

    fn find<F>(&self, hash: u64, mut eq: F) -> Option<usize>
    where
        F: FnMut(&(u64, String)) -> bool,
    {
        self.data.iter().position(|&entry| eq(&entry))
    }

    unsafe fn remove(&mut self, idx: usize) -> (String, u64) {
        let (hash, value) = self.data.remove(idx);
        (value, hash)
    }

    pub fn remove_entry<F>(&mut self, hash: u64, eq: F) -> Option<String>
    where
        F: FnMut(&(u64, String)) -> bool,
    {
        match self.find(hash, eq) {
            Some(bucket) => Some(unsafe { self.remove(bucket).0 }),
            None => None,
        }
    }
}

#[test]
fn test_remove_entry_success() {
    let mut table = TestHashTable::new();
    table.insert(42, "value1".to_string());
    table.insert(43, "value2".to_string());

    let result = table.remove_entry(42, |&(h, _)| h == 42);
    assert_eq!(result, Some("value1".to_string()));
    assert_eq!(table.data.len(), 1);
}

#[test]
fn test_remove_entry_not_found() {
    let mut table = TestHashTable::new();
    table.insert(42, "value1".to_string());

    let result = table.remove_entry(100, |&(h, _)| h == 100);
    assert_eq!(result, None);
    assert_eq!(table.data.len(), 1);
}

#[test]
fn test_remove_entry_multiple() {
    let mut table = TestHashTable::new();
    table.insert(42, "value1".to_string());
    table.insert(43, "value2".to_string());
    table.insert(44, "value3".to_string());

    let result = table.remove_entry(43, |&(h, _)| h == 43);
    assert_eq!(result, Some("value2".to_string()));
    assert_eq!(table.data.len(), 2);
}

