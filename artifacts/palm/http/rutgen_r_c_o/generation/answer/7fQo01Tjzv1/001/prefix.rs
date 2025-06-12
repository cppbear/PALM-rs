// Answer 0

#[test]
fn test_try_append_valid_case() {
    let mut map = HeaderMap::with_capacity(1024);
    let key = "example.com".parse::<HeaderName>().unwrap();
    let value = "value1".parse::<HeaderValue>().unwrap();
    map.try_append(key, value).unwrap();
}

#[test]
fn test_try_append_multiple_values_for_same_key() {
    let mut map = HeaderMap::with_capacity(1024);
    let key = "example.com".parse::<HeaderName>().unwrap();
    let value1 = "value1".parse::<HeaderValue>().unwrap();
    let value2 = "value2".parse::<HeaderValue>().unwrap();
    
    map.try_append(key, value1).unwrap();
    map.try_append(key, value2).unwrap();
}

#[test]
#[should_panic]
fn test_try_append_exceeds_max_capacity() {
    let mut map = HeaderMap::with_capacity(32768);
    let key = "example.com".parse::<HeaderName>().unwrap();
    let value = "overflow_value".parse::<HeaderValue>().unwrap();

    for _ in 0..40000 {
        map.try_append(key.clone(), value.clone()).unwrap();
    }
}

#[test]
fn test_try_append_no_key_present() {
    let mut map = HeaderMap::with_capacity(1024);
    let key = "new-key.com".parse::<HeaderName>().unwrap();
    let value = "new_value".parse::<HeaderValue>().unwrap();

    let result = map.try_append(key, value).unwrap();
    assert!(!result);
}

#[test]
fn test_try_append_empty_map() {
    let mut map = HeaderMap::with_capacity(512);
    let key = "empty.com".parse::<HeaderName>().unwrap();
    let value = "first_value".parse::<HeaderValue>().unwrap();

    map.try_append(key, value).unwrap();
    assert!(!map.is_empty());
}

