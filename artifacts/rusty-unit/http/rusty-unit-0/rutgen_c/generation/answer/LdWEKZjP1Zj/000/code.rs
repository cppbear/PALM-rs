// Answer 0

#[test]
fn test_values_mut() {
    use http::HeaderMap;
    use http::header::{CONTENT_LENGTH, HOST};

    let mut map: HeaderMap<String> = HeaderMap::with_capacity(10);

    map.insert(HOST, "hello".to_string());
    map.append(HOST, "goodbye".to_string());
    map.insert(CONTENT_LENGTH, "123".to_string());

    for value in map.values_mut() {
        value.push_str("-boop");
    }

    assert_eq!(
        map.get(HOST).unwrap(),
        &"hello-boop".to_string()
    );
    assert_eq!(
        map.get(HOST).unwrap(),
        &"goodbye-boop".to_string()
    );
    assert_eq!(
        map.get(CONTENT_LENGTH).unwrap(),
        &"123-boop".to_string()
    );
}

#[test]
fn test_values_mut_empty() {
    use http::HeaderMap;

    let mut map: HeaderMap<String> = HeaderMap::with_capacity(10);

    for value in map.values_mut() {
        value.push_str("-boop");
    }

    assert!(map.is_empty());
}

