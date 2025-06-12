// Answer 0

#[test]
fn test_reverse_empty_entries() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.reverse();
}

#[test]
fn test_reverse_single_entry() {
    let mut map = IndexMapCore::with_capacity(1);
    map.entries.push(Bucket { hash: 0, key: 1, value: 100 });
    map.indices.push(0);
    map.reverse();
}

#[test]
fn test_reverse_multiple_entries() {
    let mut map = IndexMapCore::with_capacity(3);
    map.entries.push(Bucket { hash: 0, key: 1, value: 100 });
    map.entries.push(Bucket { hash: 1, key: 2, value: 200 });
    map.entries.push(Bucket { hash: 2, key: 3, value: 300 });
    map.indices.push(0);
    map.indices.push(1);
    map.indices.push(2);
    map.reverse();
}

#[test]
fn test_reverse_large_number_of_entries() {
    let mut map = IndexMapCore::with_capacity(1000);
    for i in 0..1000 {
        map.entries.push(Bucket { hash: i as HashValue, key: i, value: i * 10 });
        map.indices.push(i);
    }
    map.reverse();
}

#[test]
fn test_reverse_indices_mismatch_with_entries() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.entries.push(Bucket { hash: 0, key: 1, value: 100 });
    // Intentionally leaving indices empty
    map.reverse();
}

#[test]
fn test_reverse_with_indices_uninitialized() {
    let mut map = IndexMapCore::with_capacity(10);
    for i in 0..5 {
        map.entries.push(Bucket { hash: i as HashValue, key: i, value: i * 10 });
        map.indices.push(i);
    }
    map.entries.clear();
    // Ensure reverse does not panic when indices reference now-empty entries
    map.reverse();
}

