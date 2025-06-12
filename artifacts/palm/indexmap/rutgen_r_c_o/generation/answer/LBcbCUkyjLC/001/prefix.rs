// Answer 0

#[test]
fn test_append_unchecked_empty_maps() {
    let mut map1 = IndexMapCore::new();
    let mut map2 = IndexMapCore::new();
    map1.append_unchecked(&mut map2);
}

#[test]
fn test_append_unchecked_non_empty_map_to_empty_map() {
    let mut map1 = IndexMapCore::new();
    let mut map2 = IndexMapCore::with_capacity(10);
    map2.entries.push(Bucket { hash: HashValue::from(1), key: 1, value: "a" });
    map2.entries.push(Bucket { hash: HashValue::from(2), key: 2, value: "b" });
    map1.append_unchecked(&mut map2);
}

#[test]
fn test_append_unchecked_empty_map_to_non_empty_map() {
    let mut map1 = IndexMapCore::with_capacity(10);
    map1.entries.push(Bucket { hash: HashValue::from(1), key: 1, value: "a" });
    let mut map2 = IndexMapCore::new();
    map1.append_unchecked(&mut map2);
}

#[test]
fn test_append_unchecked_non_empty_maps() {
    let mut map1 = IndexMapCore::with_capacity(5);
    map1.entries.push(Bucket { hash: HashValue::from(1), key: 1, value: "a" });
    
    let mut map2 = IndexMapCore::with_capacity(5);
    map2.entries.push(Bucket { hash: HashValue::from(2), key: 2, value: "b" });
    map2.entries.push(Bucket { hash: HashValue::from(3), key: 3, value: "c" });
    
    map1.append_unchecked(&mut map2);
}

#[test]
fn test_append_unchecked_large_capacity() {
    let mut map1 = IndexMapCore::with_capacity(IndexMapCore::<usize, &str>::MAX_ENTRIES_CAPACITY);
    let mut map2 = IndexMapCore::with_capacity(IndexMapCore::<usize, &str>::MAX_ENTRIES_CAPACITY);
    for i in 0..(IndexMapCore::<usize, &str>::MAX_ENTRIES_CAPACITY / 2) {
        map1.entries.push(Bucket { hash: HashValue::from(i), key: i, value: "value" });
        map2.entries.push(Bucket { hash: HashValue::from(i + IndexMapCore::<usize, &str>::MAX_ENTRIES_CAPACITY / 2), key: i + IndexMapCore::<usize, &str>::MAX_ENTRIES_CAPACITY / 2, value: "value" });
    }
    map1.append_unchecked(&mut map2);
}

#[test]
#[should_panic]
fn test_append_unchecked_exceeding_capacity() {
    let mut map1 = IndexMapCore::with_capacity(IndexMapCore::<usize, &str>::MAX_ENTRIES_CAPACITY);
    let mut map2 = IndexMapCore::with_capacity(IndexMapCore::<usize, &str>::MAX_ENTRIES_CAPACITY);
    
    for i in 0..(IndexMapCore::<usize, &str>::MAX_ENTRIES_CAPACITY) {
        map1.entries.push(Bucket { hash: HashValue::from(i), key: i, value: "value" });
    }
    for i in 0..(IndexMapCore::<usize, &str>::MAX_ENTRIES_CAPACITY) {
        map2.entries.push(Bucket { hash: HashValue::from(i), key: i, value: "value" });
    }
    map1.append_unchecked(&mut map2);
}

