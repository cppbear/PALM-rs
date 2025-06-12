// Answer 0

#[test]
fn test_split_off_empty() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    let result = index_map.split_off(0);
}

#[test]
fn test_split_off_non_empty_capacity() {
    let mut index_map = IndexMapCore::<usize, usize>::with_capacity(5);
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 100 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 200 });
    let result = index_map.split_off(2);
}

#[test]
fn test_split_off_full_capacity() {
    let mut index_map = IndexMapCore::<usize, usize>::with_capacity(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY);
    for i in 0..(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY) {
        index_map.entries.push(Bucket { hash: HashValue::default(), key: i, value: i as usize * 10 });
    }
    let result = index_map.split_off(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 1. Expected index <= len")]
fn test_split_off_panic_over_capacity() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    let _result = index_map.split_off(1);
}

#[test]
fn test_split_off_middle() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 100 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 200 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 3, value: 300 });
    let result = index_map.split_off(1);
}

