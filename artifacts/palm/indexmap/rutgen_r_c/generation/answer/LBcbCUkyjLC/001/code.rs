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
fn test_append_unchecked_non_empty_map_to_empty_map() {
    let mut map1: IndexMapCore<usize, usize> = IndexMapCore::new();
    let mut map2: IndexMapCore<usize, usize> = IndexMapCore::new();
    
    map2.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    map2.indices.insert_unique(0, 0, |_| unreachable!());

    map1.append_unchecked(&mut map2);
    assert_eq!(map1.len(), 1);
    assert_eq!(map2.len(), 0);
}

#[test]
fn test_append_unchecked_empty_map_to_non_empty_map() {
    let mut map1: IndexMapCore<usize, usize> = IndexMapCore::new();
    let mut map2: IndexMapCore<usize, usize> = IndexMapCore::new();

    map1.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    map1.indices.insert_unique(0, 0, |_| unreachable!());

    map1.append_unchecked(&mut map2);
    assert_eq!(map1.len(), 1);
    assert_eq!(map2.len(), 0);
}

#[test]
fn test_append_unchecked_non_empty_maps() {
    let mut map1: IndexMapCore<usize, usize> = IndexMapCore::new();
    let mut map2: IndexMapCore<usize, usize> = IndexMapCore::new();

    map1.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    map1.indices.insert_unique(0, 0, |_| unreachable!());

    map2.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });
    map2.indices.insert_unique(0, 0, |_| unreachable!());

    map1.append_unchecked(&mut map2);
    assert_eq!(map1.len(), 2);
    assert_eq!(map2.len(), 0);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_append_unchecked_panic_on_invalid_index() {
    let mut map1: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(1);
    let mut map2: IndexMapCore<usize, usize> = IndexMapCore::new();

    map1.append_unchecked(&mut map2);
    // Triggering panic condition, expect that map1's state should not allow for more
    map1.swap_remove_index(1);
}

