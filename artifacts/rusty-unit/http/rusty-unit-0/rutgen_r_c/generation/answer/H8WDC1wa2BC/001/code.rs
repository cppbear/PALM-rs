// Answer 0

#[test]
fn test_value_iter_mut() {
    #[derive(Default)]
    struct TestHeaderValue {
        value: String,
    }
    
    let mut header_map: HeaderMap<TestHeaderValue> = HeaderMap::with_capacity(10);
    
    // Insert some test data
    header_map.insert("Header1", TestHeaderValue { value: "Value1".to_string() });
    header_map.insert("Header2", TestHeaderValue { value: "Value2".to_string() });
    
    // Ensure the entries vector has at least 2 elements to test index 1
    assert!(header_map.entries.len() >= 2);
    
    // Test valid index
    let iter = header_map.value_iter_mut(1);
    assert_eq!(iter.index, 1);
    assert!(matches!(iter.front, Some(Cursor::Head)));
    
    // Test edge case - first index
    let iter_zero = header_map.value_iter_mut(0);
    assert_eq!(iter_zero.index, 0);
    assert!(matches!(iter_zero.front, Some(Cursor::Head)));
    
    // Test last index
    let last_index = header_map.entries.len() - 1;
    let iter_last = header_map.value_iter_mut(last_index);
    assert_eq!(iter_last.index, last_index);
    assert!(matches!(iter_last.front, Some(Cursor::Head)));
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_value_iter_mut_panic() {
    #[derive(Default)]
    struct TestHeaderValue {
        value: String,
    }
    
    let mut header_map: HeaderMap<TestHeaderValue> = HeaderMap::with_capacity(1);
    header_map.insert("Header", TestHeaderValue { value: "Value".to_string() });

    // Try to access an out-of-bounds index
    let _ = header_map.value_iter_mut(1); // This should panic
}

