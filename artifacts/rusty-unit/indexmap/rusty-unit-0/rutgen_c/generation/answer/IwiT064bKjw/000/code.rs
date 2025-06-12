// Answer 0

#[test]
fn test_pop_with_entries() {
    #[derive(Debug)]
    struct TestKey(usize);
    #[derive(Debug)]
    struct TestValue(String);

    let mut index_map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    
    index_map.push_entry(HashValue(1), TestKey(1), TestValue("Value1".into()));
    index_map.push_entry(HashValue(2), TestKey(2), TestValue("Value2".into()));
    
    let popped_entry = index_map.pop();
    
    assert_eq!(popped_entry, Some((TestKey(2), TestValue("Value2".into()))));
    assert_eq!(index_map.len(), 1);
}

#[test]
fn test_pop_empty() {
    #[derive(Debug)]
    struct TestKey(usize);
    #[derive(Debug)]
    struct TestValue(String);
    
    let mut index_map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    
    let popped_entry = index_map.pop();
    
    assert_eq!(popped_entry, None);
}

