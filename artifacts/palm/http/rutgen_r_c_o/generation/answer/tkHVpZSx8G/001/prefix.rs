// Answer 0

#[test]
fn test_or_insert_with_small_value() {
    let mut map: HeaderMap<u32> = HeaderMap::default();
    let header_name = HeaderName::try_from("small-header").unwrap();
    let result = map.entry(header_name).or_insert(1);
}

#[test]
fn test_or_insert_with_large_value() {
    let mut map: HeaderMap<u32> = HeaderMap::default();
    let header_name = HeaderName::try_from("large-header").unwrap();
    let result = map.entry(header_name).or_insert(65535);
}

#[test]
fn test_or_insert_when_empty() {
    let mut map: HeaderMap<u32> = HeaderMap::default();
    let header_name = HeaderName::try_from("empty-header").unwrap();
    let result = map.entry(header_name).or_insert(0);
}

#[test]
fn test_or_insert_multiple_entries() {
    let mut map: HeaderMap<u32> = HeaderMap::default();
    let headers = vec![
        HeaderName::try_from("header1").unwrap(),
        HeaderName::try_from("header2").unwrap(),
        HeaderName::try_from("header1").unwrap(),
    ];

    for header in headers {
        let counter = map.entry(header).or_insert(1);
        *counter += 1;
    }
}

#[test]
#[should_panic(expected = "size overflows MAX_SIZE")]
fn test_or_insert_exceed_max_capacity() {
    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(32768);
    let header_name = HeaderName::try_from("overflow-header").unwrap();
    for _ in 0..32768 {
        let _ = map.entry(header_name.clone()).or_insert(1);
    }
}

