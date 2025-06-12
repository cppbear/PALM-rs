// Answer 0

#[test]
fn test_iter_mut_with_multiple_values() {
    use http::HeaderMap;
    use http::header::{CONTENT_LENGTH, HOST};

    let mut map = HeaderMap::default();

    map.insert(HOST, "hello".to_string());
    map.append(HOST, "goodbye".to_string());
    map.insert(CONTENT_LENGTH, "123".to_string());

    let mut values: Vec<String> = Vec::new();

    for (key, value) in map.iter_mut() {
        value.push_str("-boop");
        values.push(value.clone()); // Collect the modified values
    }

    assert_eq!(values.len(), 3);
    assert!(values.contains(&"hello-boop".to_string()));
    assert!(values.contains(&"goodbye-boop".to_string()));
    assert!(values.contains(&"123-boop".to_string()));
}

#[test]
fn test_iter_mut_empty_header_map() {
    use http::HeaderMap;

    let mut map = HeaderMap::default();

    let mut values: Vec<String> = Vec::new();

    for (_, value) in map.iter_mut() {
        values.push(value.clone()); // This block should not execute
    }

    assert!(values.is_empty());
}

