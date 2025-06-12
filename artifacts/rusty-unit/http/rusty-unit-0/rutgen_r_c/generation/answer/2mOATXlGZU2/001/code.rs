// Answer 0

struct TestHeaderName;

impl HdrName for TestHeaderName {
    fn as_str(&self) -> &str {
        "Test-Header"
    }
}

#[test]
fn test_index_success() {
    let header_value = HeaderValue::from("Test Value");
    let mut header_map = HeaderMap::new();
    header_map.insert(TestHeaderName {}, header_value.clone());
    
    let value = header_map.index(TestHeaderName {});
    assert_eq!(value, &header_value);
}

#[test]
#[should_panic(expected = "no entry found for key \"Test-Header\"")]
fn test_index_panic() {
    let mut header_map = HeaderMap::new();
    header_map.index(TestHeaderName {});
}

