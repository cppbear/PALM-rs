// Answer 0

#[test]
fn test_borrow_mut_initial_state() {
    let mut index_map_core: IndexMapCore<usize, String> = IndexMapCore::new();
    let ref_mut = index_map_core.borrow_mut();
    assert_eq!(ref_mut.indices.len(), 0);
    assert_eq!(ref_mut.entries.len(), 0);
}

#[test]
fn test_borrow_mut_with_capacity() {
    let mut index_map_core: IndexMapCore<usize, String> = IndexMapCore::with_capacity(10);
    let ref_mut = index_map_core.borrow_mut();
    assert_eq!(ref_mut.indices.len(), 0);
    assert_eq!(ref_mut.entries.len(), 0);
}

#[test]
fn test_borrow_mut_after_insertion() {
    let mut index_map_core: IndexMapCore<usize, String> = IndexMapCore::new();
    let mut ref_mut = index_map_core.borrow_mut();
    
    ref_mut.entries.push(Bucket { hash: HashValue::default(), key: 1, value: "one".to_string() });
    ref_mut.entries.push(Bucket { hash: HashValue::default(), key: 2, value: "two".to_string() });
    
    assert_eq!(ref_mut.indices.len(), 0);
    assert_eq!(ref_mut.entries.len(), 2);
}

#[test]
fn test_borrow_mut_after_clear() {
    let mut index_map_core: IndexMapCore<usize, String> = IndexMapCore::new();
    {
        let mut ref_mut = index_map_core.borrow_mut();
        ref_mut.entries.push(Bucket { hash: HashValue::default(), key: 1, value: "one".to_string() });
        ref_mut.entries.push(Bucket { hash: HashValue::default(), key: 2, value: "two".to_string() });
    }
    
    index_map_core.clear();
    
    let ref_mut = index_map_core.borrow_mut();
    assert_eq!(ref_mut.entries.len(), 0);
}

#[test]
fn test_borrow_mut_multiple_times() {
    let mut index_map_core: IndexMapCore<usize, String> = IndexMapCore::new();
    {
        let ref_mut1 = index_map_core.borrow_mut();
        ref_mut1.entries.push(Bucket { hash: HashValue::default(), key: 1, value: "one".to_string() });
    }
    
    {
        let ref_mut2 = index_map_core.borrow_mut();
        ref_mut2.entries.push(Bucket { hash: HashValue::default(), key: 2, value: "two".to_string() });
    }

    let ref_mut = index_map_core.borrow_mut();
    assert_eq!(ref_mut.entries.len(), 2);
}

