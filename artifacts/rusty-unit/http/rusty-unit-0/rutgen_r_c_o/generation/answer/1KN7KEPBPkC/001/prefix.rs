// Answer 0

#[test]
fn test_fmt_non_empty_map() {
    let mut map = HeaderMap::with_capacity(10);
    map.insert(HeaderName::from("key1"), HeaderValue::from("value1"));
    map.insert(HeaderName::from("key2"), HeaderValue::from("value2"));
    let _ = format!("{:?}", map);
}

#[test]
fn test_fmt_empty_map() {
    let map: HeaderMap = HeaderMap::with_capacity(0);
    let _ = format!("{:?}", map);
}

#[test]
fn test_fmt_large_map() {
    let mut map = HeaderMap::with_capacity(100);
    for i in 0..100 {
        let key = HeaderName::from(format!("key{}", i));
        let value = HeaderValue::from(format!("value{}", i));
        map.insert(key, value);
    }
    let _ = format!("{:?}", map);
}

#[test]
fn test_fmt_single_entry_map() {
    let mut map = HeaderMap::with_capacity(1);
    map.insert(HeaderName::from("single_key"), HeaderValue::from("single_value"));
    let _ = format!("{:?}", map);
}

#[test]
#[should_panic]
fn test_fmt_panic_on_empty_header_name() {
    let mut map = HeaderMap::with_capacity(1);
    map.insert(HeaderName::from(""), HeaderValue::from("value"));
    let _ = format!("{:?}", map);
}

