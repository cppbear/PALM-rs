// Answer 0

#[test]
fn test_retain_in_order_basic() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.indices = Indices::with_capacity(2);
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });
    
    map.retain_in_order(|key, value| {
        *key % 2 == 0 // Keep only even keys
    });
}

#[test]
fn test_retain_in_order_non_panic() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.indices = Indices::with_capacity(3);
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 3, value: 30 });
    
    // This will not panic since self.entries.len() < self.indices.len() is true.
    map.retain_in_order(|key, value| {
        *key < 2 // Keep keys less than 2
    });
}

#[test]
fn test_retain_in_order_edge_case() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.indices = Indices::with_capacity(2);
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 100 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 200 });
    
    map.retain_in_order(|key, value| {
        *key == 1 // Keep only the key 1
    });
}

#[test]
fn test_retain_in_order_empty() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.indices = Indices::with_capacity(1);
    // This will not panic because there are no entries.
    map.retain_in_order(|_key, _value| true);
}

