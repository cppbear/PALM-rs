// Answer 0

#[test]
fn test_push_entry_with_full_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(1);
    let hash = HashValue(0);
    let key = 1;
    let value = 2;
    map.push_entry(hash, key, value);
    let hash2 = HashValue(1);
    let key2 = 2;
    let value2 = 3;
    map.push_entry(hash2, key2, value2);
}

#[test]
fn test_push_entry_with_large_key_value() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(2);
    let hash = HashValue((isize::MAX as usize) / mem::size_of::<Bucket<usize, usize>>);
    let key = 1;
    let value = 2;
    map.push_entry(hash, key, value);
}

#[test]
fn test_push_entry_with_multiple_entries() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(3);
    let hash1 = HashValue(10);
    let key1 = 1;
    let value1 = 100;
    map.push_entry(hash1, key1, value1);
    
    let hash2 = HashValue(20);
    let key2 = 2;
    let value2 = 200;
    map.push_entry(hash2, key2, value2);
    
    let hash3 = HashValue(30);
    let key3 = 3;
    let value3 = 300;
    map.push_entry(hash3, key3, value3);
}

