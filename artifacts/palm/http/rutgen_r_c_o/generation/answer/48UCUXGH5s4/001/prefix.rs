// Answer 0

#[test]
fn test_insert_with_valid_value() {
    let mut map = HeaderMap::new();
    if let Entry::Vacant(v) = map.entry(HeaderName { inner: Repr::Custom("x-hello".into()) }) {
        v.insert(HeaderValue::from("world"));
    }
}

#[test]
fn test_insert_with_empty_value() {
    let mut map = HeaderMap::new();
    if let Entry::Vacant(v) = map.entry(HeaderName { inner: Repr::Custom("x-empty".into()) }) {
        v.insert(HeaderValue::from(""));
    }
}

#[test]
fn test_insert_multiple_values() {
    let mut map = HeaderMap::new();
    if let Entry::Vacant(v) = map.entry(HeaderName { inner: Repr::Custom("x-hello-1".into()) }) {
        v.insert(HeaderValue::from("value1"));
    }
    if let Entry::Vacant(v) = map.entry(HeaderName { inner: Repr::Custom("x-hello-2".into()) }) {
        v.insert(HeaderValue::from("value2"));
    }
}

#[test]
fn test_insert_at_max_capacity() {
    let mut map = HeaderMap::new();
    for i in 0..32767 {
        let key = format!("key-{}", i);
        if let Entry::Vacant(v) = map.entry(HeaderName { inner: Repr::Custom(key.into()) }) {
            v.insert(HeaderValue::from(format!("value-{}", i)));
        }
    }
}

#[should_panic(expected = "size overflows MAX_SIZE")]
#[test]
fn test_insert_exceeding_max_size() {
    let mut map = HeaderMap::new();
    for i in 0..32768 {
        let key = format!("key-{}", i);
        if let Entry::Vacant(v) = map.entry(HeaderName { inner: Repr::Custom(key.into()) }) {
            v.insert(HeaderValue::from(format!("value-{}", i)));
        }
    }
}

#[test]
fn test_insert_with_different_keys() {
    let mut map = HeaderMap::new();
    if let Entry::Vacant(v) = map.entry(HeaderName { inner: Repr::Custom("x-unique-1".into()) }) {
        v.insert(HeaderValue::from("unique_value_1"));
    }
    if let Entry::Vacant(v) = map.entry(HeaderName { inner: Repr::Custom("x-unique-2".into()) }) {
        v.insert(HeaderValue::from("unique_value_2"));
    }
}

