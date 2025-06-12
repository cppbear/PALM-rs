// Answer 0

#[test]
fn test_binary_search_by_with_existing_key() {
    let mut entries = Vec::new();
    for i in 0..1000 {
        entries.push(Bucket { hash: 0, key: i, value: i });
    }
    let slice = Box::new(Slice { entries: entries.try_into().unwrap() });

    let result = slice.binary_search_by(|key, _| key.cmp(&500));
}

#[test]
fn test_binary_search_by_with_non_existing_key_lower_bound() {
    let mut entries = Vec::new();
    for i in 0..1000 {
        entries.push(Bucket { hash: 0, key: i, value: i });
    }
    let slice = Box::new(Slice { entries: entries.try_into().unwrap() });

    let result = slice.binary_search_by(|key, _| key.cmp(&-1));
}

#[test]
fn test_binary_search_by_with_non_existing_key_upper_bound() {
    let mut entries = Vec::new();
    for i in 0..1000 {
        entries.push(Bucket { hash: 0, key: i, value: i });
    }
    let slice = Box::new(Slice { entries: entries.try_into().unwrap() });

    let result = slice.binary_search_by(|key, _| key.cmp(&1001));
}

#[test]
fn test_binary_search_by_with_random_key() {
    let mut entries = Vec::new();
    for i in 0..1000 {
        entries.push(Bucket { hash: 0, key: i, value: i });
    }
    let slice = Box::new(Slice { entries: entries.try_into().unwrap() });

    let result = slice.binary_search_by(|key, _| key.cmp(&750));
}

#[test]
fn test_binary_search_by_with_custom_comparator() {
    let mut entries = Vec::new();
    for i in 0..1000 {
        entries.push(Bucket { hash: 0, key: i, value: i * 10 });
    }
    let slice = Box::new(Slice { entries: entries.try_into().unwrap() });

    let result = slice.binary_search_by(|key, value| value.cmp(&(key * 10)));
}

