// Answer 0

#[derive(Debug)]
struct HashTable {
    bucket_mask: usize,
}

impl HashTable {
    fn is_empty_singleton(&self) -> bool {
        self.bucket_mask == 0
    }
}

#[test]
fn test_is_empty_singleton_when_empty() {
    let table = HashTable { bucket_mask: 0 };
    assert!(table.is_empty_singleton());
}

#[test]
fn test_is_empty_singleton_when_not_empty() {
    let table = HashTable { bucket_mask: 1 };
    assert!(!table.is_empty_singleton());
}

#[test]
fn test_is_empty_singleton_with_large_bucket_mask() {
    let table = HashTable { bucket_mask: 10 };
    assert!(!table.is_empty_singleton());
}

