// Answer 0

#[derive(Debug)]
struct HeaderName(String);

#[derive(Debug)]
struct HashValue(u64);

#[derive(Debug)]
struct Bucket<T> {
    hash: HashValue,
    key: HeaderName,
    value: T,
    links: Option<Box<Bucket<T>>>,
}

struct Map<T> {
    entries: Vec<Bucket<T>>,
}

impl<T> Map<T> {
    fn new() -> Self {
        Self { entries: Vec::new() }
    }
}

struct MaxSizeReached;

impl MaxSizeReached {
    fn new() -> Self {
        MaxSizeReached
    }
}

const MAX_SIZE: usize = 2;

#[test]
fn test_try_insert_entry_success() {
    let mut map: Map<i32> = Map::new();
    let hash = HashValue(1);
    let key = HeaderName("Key1".to_string());
    let value = 42;

    let result = map.try_insert_entry(hash, key.clone(), value);
    assert!(result.is_ok());
    assert_eq!(map.entries.len(), 1);
}

#[test]
fn test_try_insert_entry_max_size_reached() {
    let mut map: Map<i32> = Map::new();
    let hash1 = HashValue(1);
    let key1 = HeaderName("Key1".to_string());
    let value1 = 42;

    let hash2 = HashValue(2);
    let key2 = HeaderName("Key2".to_string());
    let value2 = 43;

    let _ = map.try_insert_entry(hash1, key1.clone(), value1);
    let _ = map.try_insert_entry(hash2, key2.clone(), value2);

    let hash3 = HashValue(3);
    let key3 = HeaderName("Key3".to_string());
    let value3 = 44;

    let result = map.try_insert_entry(hash3, key3.clone(), value3);
    assert_eq!(result, Err(MaxSizeReached::new()));
    assert_eq!(map.entries.len(), 2);
}

