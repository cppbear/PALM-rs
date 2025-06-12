// Answer 0

#[test]
fn test_value_iter_mut_empty() {
    let mut header_map: HeaderMap = HeaderMap::with_capacity(0);
    let idx = 0; // This will panic due to empty entries
    let _iter = header_map.value_iter_mut(idx);
}

#[test]
fn test_value_iter_mut_single_entry() {
    let mut header_map: HeaderMap = HeaderMap::with_capacity(1);
    header_map.insert("Key1", HeaderValue::from("Value1"));
    let idx = 0;
    let _iter = header_map.value_iter_mut(idx);
}

#[test]
fn test_value_iter_mut_multiple_entries() {
    let mut header_map: HeaderMap = HeaderMap::with_capacity(5);
    header_map.insert("Key1", HeaderValue::from("Value1"));
    header_map.insert("Key2", HeaderValue::from("Value2"));
    header_map.insert("Key3", HeaderValue::from("Value3"));
    let idx = 1; // Middle entry
    let _iter = header_map.value_iter_mut(idx);
}

#[test]
#[should_panic]
fn test_value_iter_mut_out_of_bounds() {
    let mut header_map: HeaderMap = HeaderMap::with_capacity(3);
    header_map.insert("Key1", HeaderValue::from("Value1"));
    header_map.insert("Key2", HeaderValue::from("Value2"));
    let idx = 2; // No third entry exists
    let _iter = header_map.value_iter_mut(idx);
}

#[test]
fn test_value_iter_mut_links() {
    let mut header_map: HeaderMap = HeaderMap::with_capacity(2);
    header_map.insert("Key1", HeaderValue::from("Value1"));
    header_map.insert("Key2", HeaderValue::from("Value2"));
    let idx = 0; // First entry with links
    let _iter = header_map.value_iter_mut(idx);
}

