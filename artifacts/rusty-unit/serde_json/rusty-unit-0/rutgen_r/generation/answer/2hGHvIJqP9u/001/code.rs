// Answer 0

#[test]
fn test_is_empty_on_empty_map() {
    #[derive(Default)]
    struct TestMap {
        map: std::collections::HashMap<String, String>,
    }

    let test_map = TestMap::default();
    assert!(test_map.is_empty());
}

#[test]
fn test_is_empty_on_non_empty_map() {
    #[derive(Default)]
    struct TestMap {
        map: std::collections::HashMap<String, String>,
    }

    let mut test_map = TestMap::default();
    test_map.map.insert("key1".to_string(), "value1".to_string());
    assert!(!test_map.is_empty());
}

#[test]
fn test_is_empty_on_map_with_removal() {
    #[derive(Default)]
    struct TestMap {
        map: std::collections::HashMap<String, String>,
    }

    let mut test_map = TestMap::default();
    test_map.map.insert("key1".to_string(), "value1".to_string());
    test_map.map.remove("key1");
    assert!(test_map.is_empty());
}

