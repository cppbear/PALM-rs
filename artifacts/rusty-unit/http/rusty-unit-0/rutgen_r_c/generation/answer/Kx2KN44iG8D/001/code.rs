// Answer 0

#[test]
fn test_from_iter_empty() {
    let iter: Vec<(HdrName, HeaderValue)> = vec![];
    let map: HeaderMap<HeaderValue> = HeaderMap::from_iter(iter);
    assert_eq!(map.entries.len(), 0);
}

#[test]
fn test_from_iter_single_entry() {
    let name = HdrName::try_from("Test-Header").unwrap();
    let value = HeaderValue::from_static("TestValue");
    let iter = vec![(name.clone(), value.clone())];
    let map: HeaderMap<HeaderValue> = HeaderMap::from_iter(iter);
    assert_eq!(map.entries.len(), 1);
    assert_eq!(map.entries[0].key, name);
    assert_eq!(map.entries[0].value, value);
}

#[test]
fn test_from_iter_multiple_entries() {
    let entries = vec![
        (HdrName::try_from("Header1").unwrap(), HeaderValue::from_static("Value1")),
        (HdrName::try_from("Header2").unwrap(), HeaderValue::from_static("Value2")),
        (HdrName::try_from("Header3").unwrap(), HeaderValue::from_static("Value3")),
    ];
    let map: HeaderMap<HeaderValue> = HeaderMap::from_iter(entries.clone());
    assert_eq!(map.entries.len(), entries.len());
    for (key, value) in entries {
        assert!(map.entries.iter().any(|entry| entry.key == key && entry.value == value));
    }
}

#[should_panic(expected = "InvalidHeaderName")]
#[test]
fn test_from_iter_invalid_header_name() {
    let name = HdrName::try_from("Invalid-Header-Name-@").unwrap_err(); // Simulating failure
    let iter = vec![(name, HeaderValue::from_static("TestValue"))]; // This will panic
    let _ = HeaderMap::from_iter(iter);
}

