// Answer 0

#[test]
fn test_iter_mut_empty_header_map() {
    let mut map = http::HeaderMap::default();
    let mut iter = map.iter_mut();
    
    assert_eq!(iter.entry, 0);
    assert!(iter.cursor.is_none());
}

#[test]
fn test_iter_mut_single_entry() {
    let mut map = http::HeaderMap::default();
    map.insert(http::header::HOST, "example.com".to_string());
    let mut iter = map.iter_mut();
    
    // Ensure we have one entry
    assert_eq!(iter.entry, 0);
    assert!(iter.cursor.is_some());
    
    for (key, value) in iter {
        assert_eq!(key, http::header::HOST);
        value.push_str("-test");
        assert_eq!(value, "example.com-test");
    }
}

#[test]
fn test_iter_mut_multiple_entries() {
    let mut map = http::HeaderMap::default();
    map.insert(http::header::HOST, "example.com".to_string());
    map.append(http::header::HOST, "test.com".to_string());
    map.insert(http::header::CONTENT_LENGTH, "123".to_string());

    let mut iter = map.iter_mut();

    // Ensure we have multiple entries
    assert_eq!(iter.entry, 0);
    assert!(iter.cursor.is_some());
    
    let mut count = 0;
    
    for (key, value) in iter {
        count += 1;
        if count == 1 {
            assert_eq!(key, http::header::HOST);
            value.push_str("-first");
            assert_eq!(value, "example.com-first");
        } else if count == 2 {
            assert_eq!(key, http::header::HOST);
            value.push_str("-second");
            assert_eq!(value, "test.com-second");
        } else if count == 3 {
            assert_eq!(key, http::header::CONTENT_LENGTH);
            value.push_str("-test");
            assert_eq!(value, "123-test");
        }
    }
    assert_eq!(count, 3);
}

#[test]
fn test_iter_mut_no_panic_conditions() {
    let mut map = http::HeaderMap::default();
    map.insert(http::header::USER_AGENT, "Mozilla/5.0".to_string());
    
    let mut iter = map.iter_mut();
    
    for (key, value) in iter {
        // Check for non-panicking behavior
        assert_eq!(key, http::header::USER_AGENT);
        value.push_str(" (compatible)");
        assert_eq!(value, "Mozilla/5.0 (compatible)");
    }
}

