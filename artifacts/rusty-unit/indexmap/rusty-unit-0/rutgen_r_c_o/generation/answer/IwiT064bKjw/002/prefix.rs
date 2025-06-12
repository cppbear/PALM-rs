// Answer 0

#[test]
fn test_pop_non_empty() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: 10 });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 2, value: 20 });
    
    let result = index_map.pop();
}

#[test]
fn test_pop_single_element() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.entries.push(Bucket { hash: HashValue(3), key: 3, value: 30 });

    let result = index_map.pop();
}

#[test]
fn test_pop_multiple_elements() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    for i in 0..5 {
        index_map.entries.push(Bucket { hash: HashValue(i as usize), key: i, value: i * 10 });
    }

    let result = index_map.pop();
}

#[test]
fn test_pop_empty() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    
    let result = index_map.pop();
}

#[test]
fn test_pop_after_multiple_calls() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.entries.push(Bucket { hash: HashValue(4), key: 4, value: 40 });
    index_map.entries.push(Bucket { hash: HashValue(5), key: 5, value: 50 });

    let _ = index_map.pop();
    let result = index_map.pop();
}

