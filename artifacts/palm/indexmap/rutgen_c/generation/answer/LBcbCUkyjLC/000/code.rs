// Answer 0

#[test]
fn test_append_unchecked_empty_maps() {
    let mut map1: IndexMapCore<usize, usize> = IndexMapCore::new();
    let mut map2: IndexMapCore<usize, usize> = IndexMapCore::new();
    map1.append_unchecked(&mut map2);
    assert_eq!(map1.len(), 0);
    assert_eq!(map2.len(), 0);
}

#[test]
fn test_append_unchecked_non_empty_map() {
    let mut map1: IndexMapCore<usize, usize> = IndexMapCore::new();
    let mut map2: IndexMapCore<usize, usize> = IndexMapCore::new();

    let hash_value1 = HashValue::new(1);
    let hash_value2 = HashValue::new(2);
    map2.entries.push(Bucket { hash: hash_value1, key: 1, value: 10 });
    map2.entries.push(Bucket { hash: hash_value2, key: 2, value: 20 });

    map1.append_unchecked(&mut map2);

    assert_eq!(map1.len(), 2);
    assert_eq!(map1.entries.len(), 2);
    assert_eq!(map2.len(), 0);
}

#[test]
fn test_append_unchecked_with_existing_entries() {
    let mut map1: IndexMapCore<usize, usize> = IndexMapCore::new();
    let mut map2: IndexMapCore<usize, usize> = IndexMapCore::new();

    let hash_value1 = HashValue::new(1);
    let hash_value2 = HashValue::new(2);
    map1.entries.push(Bucket { hash: hash_value1, key: 1, value: 10 });
    map2.entries.push(Bucket { hash: hash_value2, key: 2, value: 20 });

    map1.append_unchecked(&mut map2);

    assert_eq!(map1.len(), 2);
    assert_eq!(map1.entries.len(), 2);
    assert_eq!(map2.len(), 0);
}

#[test]
fn test_append_unchecked_overwrite_capacity() {
    let mut map1: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(1);
    let mut map2: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(1);

    let hash_value1 = HashValue::new(1);
    map1.entries.push(Bucket { hash: hash_value1, key: 1, value: 10 });

    let hash_value2 = HashValue::new(2);
    let hash_value3 = HashValue::new(3);
    map2.entries.push(Bucket { hash: hash_value2, key: 2, value: 20 });
    map2.entries.push(Bucket { hash: hash_value3, key: 3, value: 30 });

    map1.append_unchecked(&mut map2);

    assert_eq!(map1.len(), 3);
    assert_eq!(map1.entries.len(), 3);
    assert_eq!(map2.len(), 0);
}

