// Answer 0

#[test]
fn test_len_initially_empty() {
    let map = HeaderMap::new();
    assert_eq!(0, map.len());
}

#[test]
fn test_len_with_single_insert() {
    let mut map = HeaderMap::new();
    map.insert("Header1".parse().unwrap(), "Value1".parse().unwrap());
    assert_eq!(1, map.len());
}

#[test]
fn test_len_with_multiple_inserts() {
    let mut map = HeaderMap::new();
    map.insert("Header1".parse().unwrap(), "Value1".parse().unwrap());
    map.insert("Header2".parse().unwrap(), "Value2".parse().unwrap());
    assert_eq!(2, map.len());
}

#[test]
fn test_len_with_key_appending() {
    let mut map = HeaderMap::new();
    map.insert("Header1".parse().unwrap(), "Value1".parse().unwrap());
    map.append("Header1".parse().unwrap(), "Value2".parse().unwrap());
    assert_eq!(2, map.len());
}

#[test]
fn test_len_with_multiple_appends() {
    let mut map = HeaderMap::new();
    map.append("Header1".parse().unwrap(), "Value1".parse().unwrap());
    map.append("Header1".parse().unwrap(), "Value2".parse().unwrap());
    map.append("Header2".parse().unwrap(), "Value3".parse().unwrap());
    assert_eq!(4, map.len());
}

#[test]
fn test_len_with_different_headers() {
    let mut map = HeaderMap::new();
    map.insert("Header1".parse().unwrap(), "Value1".parse().unwrap());
    map.append("Header2".parse().unwrap(), "Value2".parse().unwrap());
    map.append("Header2".parse().unwrap(), "Value3".parse().unwrap());
    assert_eq!(4, map.len());
}

