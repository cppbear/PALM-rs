// Answer 0

#[test]
fn test_shift_remove_index_valid() {
    let mut map = IndexMapCore::with_capacity(5);
    map.entries.push(Bucket::new(0, "value0"));
    map.entries.push(Bucket::new(1, "value1"));
    map.entries.push(Bucket::new(2, "value2"));
    
    let result = map.shift_remove_index(1);
}

#[test]
#[should_panic]
fn test_shift_remove_index_out_of_bounds() {
    let mut map = IndexMapCore::with_capacity(3);
    map.entries.push(Bucket::new(0, "value0"));
    map.entries.push(Bucket::new(1, "value1"));
    
    let result = map.shift_remove_index(3);
}

#[test]
fn test_shift_remove_index_clear() {
    let mut map = IndexMapCore::new();
    map.entries.push(Bucket::new(0, "value0"));
    
    map.clear();
    let result = map.shift_remove_index(0);
}

#[test]
fn test_shift_remove_index_empty() {
    let mut map = IndexMapCore::new();
    
    let result = map.shift_remove_index(0);
}

#[test]
fn test_shift_remove_index_max_capacity() {
    let mut map = IndexMapCore::with_capacity(IndexMapCore::MAX_ENTRIES_CAPACITY);
    for i in 0..IndexMapCore::MAX_ENTRIES_CAPACITY {
        map.entries.push(Bucket::new(i, format!("value{}", i)));
    }
    
    let result = map.shift_remove_index(IndexMapCore::MAX_ENTRIES_CAPACITY - 1);
}

