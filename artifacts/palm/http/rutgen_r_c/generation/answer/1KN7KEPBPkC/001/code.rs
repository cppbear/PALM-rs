// Answer 0

#[test]
fn test_header_map_fmt_empty() {
    let header_map: HeaderMap = HeaderMap::with_capacity(0);
    let formatted_output = format!("{:?}", header_map);
    assert_eq!(formatted_output, "{}");
}

#[test]
fn test_header_map_fmt_single_entry() {
    let mut header_map: HeaderMap = HeaderMap::with_capacity(1);
    header_map.insert("key1", "value1").unwrap(); // Assuming insert returns Result
    let formatted_output = format!("{:?}", header_map);
    assert!(formatted_output.contains("key1"));
    assert!(formatted_output.contains("value1"));
}

#[test]
fn test_header_map_fmt_multiple_entries() {
    let mut header_map: HeaderMap = HeaderMap::with_capacity(2);
    header_map.insert("key1", "value1").unwrap();
    header_map.insert("key2", "value2").unwrap();
    let formatted_output = format!("{:?}", header_map);
    assert!(formatted_output.contains("key1"));
    assert!(formatted_output.contains("value1"));
    assert!(formatted_output.contains("key2"));
    assert!(formatted_output.contains("value2"));
}

#[test]
fn test_header_map_fmt_large_capacity() {
    let mut header_map: HeaderMap = HeaderMap::with_capacity(10);
    for i in 0..10 {
        header_map.insert(format!("key{}", i), format!("value{}", i)).unwrap();
    }
    let formatted_output = format!("{:?}", header_map);
    for i in 0..10 {
        assert!(formatted_output.contains(&format!("key{}", i)));
        assert!(formatted_output.contains(&format!("value{}", i)));
    }
}

