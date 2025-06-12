// Answer 0

#[test]
fn test_get_no_values_associated_with_key() {
    let mut map = http::HeaderMap::new();
    assert!(map.get("non_existent_key").is_none());
}

#[test]
fn test_get_single_value_associated_with_key() {
    let mut map = http::HeaderMap::new();
    let host_key = http::header::HOST;
    map.insert(host_key, "example.com".parse().unwrap());
    assert_eq!(map.get(host_key).unwrap(), &"example.com");
}

#[test]
fn test_get_case_insensitive_key() {
    let mut map = http::HeaderMap::new();
    let host_key = http::header::HOST;
    map.insert(host_key, "example.com".parse().unwrap());
    assert_eq!(map.get("HOST").unwrap(), &"example.com");
    assert_eq!(map.get("host").unwrap(), &"example.com");
}

#[test]
fn test_get_first_value_when_multiple_appended() {
    let mut map = http::HeaderMap::new();
    let host_key = http::header::HOST;
    map.insert(host_key, "first_value".parse().unwrap());
    map.append(host_key, "second_value".parse().unwrap());
    
    assert_eq!(map.get(host_key).unwrap(), &"first_value");
}

#[test]
fn test_get_with_different_key_type() {
    let mut map = http::HeaderMap::new();
    let host_key = http::header::HOST;
    map.insert(host_key, "example.com".parse().unwrap());
    
    // Using different types that implement AsHeaderName
    let string_key = "HOST";
    assert_eq!(map.get(string_key).unwrap(), &"example.com");
}

