// Answer 0

#[test]
fn test_values_mut_empty() {
    use http::HeaderMap;
    use http::header::CONTENT_LENGTH;

    let mut map = HeaderMap::default();
    let mut iter = map.values_mut();
    assert_eq!(iter.count(), 0);
}

#[test]
fn test_values_mut_single_entry() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::default();
    map.insert(HOST, "example.com".to_string());

    let mut iter = map.values_mut();
    if let Some(value) = iter.next() {
        value.push_str("-updated");
        assert_eq!(value, "example.com-updated");
    } else {
        panic!("Expected an entry but found none.");
    }
}

#[test]
fn test_values_mut_multiple_entries() {
    use http::HeaderMap;
    use http::header::{HOST, CONTENT_LENGTH};

    let mut map = HeaderMap::default();
    map.insert(HOST, "first.com".to_string());
    map.append(HOST, "second.com".to_string());
    map.insert(CONTENT_LENGTH, "123".to_string());

    let mut iter = map.values_mut();
    for value in iter {
        value.push_str("-updated");
    }

    let mut updated_values: Vec<String> = map.values().map(|v| v.to_string()).collect();
    assert_eq!(updated_values, vec!["first.com-updated", "second.com-updated", "123-updated"]);
}

#[test]
fn test_values_mut_no_panic_on_mutation() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::default();
    map.insert(HOST, "test.com".to_string());

    let mut iter = map.values_mut();
    if let Some(value) = iter.next() {
        value.push_str("-modified");
        assert_eq!(value, "test.com-modified");
    }
}

