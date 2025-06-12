// Answer 0

#[test]
fn test_index_panics_when_key_not_present() {
    struct TestHeaderName;

    impl HdrName for TestHeaderName {
        fn as_str(&self) -> &str {
            "Test-Header"
        }
    }

    let empty_map: HeaderMap<()> = HeaderMap {
        mask: 0,
        indices: Box::new([]),
        entries: vec![],
        extra_values: vec![],
        danger: Danger::Green,
    };
    
    let header_name = TestHeaderName;

    let result = std::panic::catch_unwind(|| {
        let _ = empty_map[header_name];
    });

    assert!(result.is_err(), "Expected panic when accessing index of a non-existent key");
}

