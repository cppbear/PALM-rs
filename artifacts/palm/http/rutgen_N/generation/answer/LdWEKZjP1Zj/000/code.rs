// Answer 0

#[test]
fn test_values_mut_single_entry() {
    use http::HeaderMap;
    use http::header::{CONTENT_LENGTH, HOST};

    let mut map = HeaderMap::default();
    map.insert(HOST, "hello".to_string());

    for value in map.values_mut() {
        value.push_str("-boop");
    }

    assert_eq!(map.get(HOST).unwrap(), "hello-boop");
}

#[test]
fn test_values_mut_multiple_entries() {
    use http::HeaderMap;
    use http::header::{CONTENT_LENGTH, HOST};

    let mut map = HeaderMap::default();
    map.insert(HOST, "hello".to_string());
    map.append(HOST, "goodbye".to_string());
    map.insert(CONTENT_LENGTH, "123".to_string());

    for value in map.values_mut() {
        value.push_str("-boop");
    }

    assert_eq!(map.get_all(HOST)[0], "hello-boop");
    assert_eq!(map.get_all(HOST)[1], "goodbye-boop");
    assert_eq!(map.get(CONTENT_LENGTH).unwrap(), "123-boop");
}

#[test]
fn test_values_mut_empty_map() {
    use http::HeaderMap;

    let mut map = HeaderMap::default();

    for value in map.values_mut() {
        value.push_str("-boop");
    }

    assert!(map.is_empty());
}

