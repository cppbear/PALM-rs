// Answer 0

#[derive(Debug)]
struct KeyValue<K, V> {
    key: K,
    value: V,
}

struct Bucket<K, V> {
    entries: Vec<KeyValue<K, V>>,
}

impl<K, V> Bucket<K, V> {
    fn new(entries: Vec<KeyValue<K, V>>) -> Self {
        Self { entries }
    }
    
    fn refs(&self) -> (&K, &V) {
        let last_entry = self.entries.last().expect("Bucket is empty");
        (&last_entry.key, &last_entry.value)
    }
    
    fn as_entries(&self) -> &[KeyValue<K, V>] {
        &self.entries
    }
}

impl<K: Clone, V: Clone> Bucket<K, V> {
    pub fn last(&self) -> Option<(&K, &V)> {
        self.as_entries().last().map(Bucket::refs)
    }
}

#[test]
fn test_bucket_last_non_empty() {
    let entries = vec![
        KeyValue { key: 1, value: "a" },
        KeyValue { key: 2, value: "b" },
        KeyValue { key: 3, value: "c" },
    ];
    
    let bucket = Bucket::new(entries);
    let last = bucket.last();
    
    assert_eq!(last, Some((&3, &"c")));
}

#[test]
fn test_bucket_last_empty() {
    let entries: Vec<KeyValue<i32, &str>> = Vec::new();
    let bucket = Bucket::new(entries);
    
    let last = bucket.last();
    
    assert_eq!(last, None);
}

