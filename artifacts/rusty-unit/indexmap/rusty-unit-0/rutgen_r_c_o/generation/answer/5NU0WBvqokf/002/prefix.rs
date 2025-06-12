// Answer 0

#[test]
fn test_push_entry_normal_case() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let hash_value = HashValue(1);
    let key = 10;
    let value = 20;
    index_map.push_entry(hash_value, key, value);
}

#[test]
fn test_push_entry_with_capacity() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    let hash_value = HashValue(2);
    let key = 15;
    let value = 25;
    for i in 0..5 {
        index_map.push_entry(HashValue(i), i, i);
    }
    index_map.push_entry(hash_value, key, value);
}

#[test]
fn test_push_entry_at_max_capacity() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(1);
    let hash_value = HashValue(3);
    let key = 5;
    let value = 15;
    index_map.push_entry(hash_value, key, value);
}

#[test]
fn test_push_entry_in_empty_structure() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let hash_value = HashValue(4);
    let key = 2;
    let value = 8;
    index_map.push_entry(hash_value, key, value);
}

#[test]
fn test_push_entry_large_values() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    let hash_value = HashValue(5);
    let key = usize::MAX;
    let value = usize::MAX;
    index_map.push_entry(hash_value, key, value);
}

