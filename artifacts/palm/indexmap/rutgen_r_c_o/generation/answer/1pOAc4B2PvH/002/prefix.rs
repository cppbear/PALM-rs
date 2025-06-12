// Answer 0

#[test]
fn test_retain_in_order_with_empty_entries() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.indices = Indices::with_capacity(0);
    index_map.entries = Vec::new();
    index_map.retain_in_order(|_, _| true);
}

#[test]
fn test_retain_in_order_with_single_entry() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.indices = Indices::with_capacity(1);
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    index_map.retain_in_order(|_, _| true);
}

#[test]
fn test_retain_in_order_with_multiple_entries() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.indices = Indices::with_capacity(3);
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 3, value: 30 });
    index_map.retain_in_order(|_, _| true);
}

#[test]
fn test_retain_in_order_removing_entries() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.indices = Indices::with_capacity(3);
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 3, value: 30 });
    index_map.retain_in_order(|key, value| *key != 2);
}

#[test]
fn test_retain_in_order_with_equal_entries() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.indices = Indices::with_capacity(2);
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 10, value: 100 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 10, value: 200 });
    index_map.retain_in_order(|_, _| true);
}

#[test]
fn test_retain_in_order_non_removal() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.indices = Indices::with_capacity(2);
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });
    index_map.retain_in_order(|_, _| true);
}

