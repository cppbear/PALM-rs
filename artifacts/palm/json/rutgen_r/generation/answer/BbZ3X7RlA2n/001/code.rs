// Answer 0

#[test]
fn test_fmt_empty_map() {
    use std::fmt;

    struct TestMap {
        map: std::collections::HashMap<String, String>,
    }

    impl fmt::Display for TestMap {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            self.map.fmt(formatter)
        }
    }

    let empty_map = TestMap {
        map: std::collections::HashMap::new(),
    };

    let result = format!("{}", empty_map);
    assert_eq!(result, "{}");
}

#[test]
fn test_fmt_single_entry_map() {
    use std::fmt;

    struct TestMap {
        map: std::collections::HashMap<String, String>,
    }

    impl fmt::Display for TestMap {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            self.map.fmt(formatter)
        }
    }

    let mut single_entry_map = std::collections::HashMap::new();
    single_entry_map.insert("key".to_string(), "value".to_string());

    let map = TestMap { map: single_entry_map };

    let result = format!("{}", map);
    assert_eq!(result, "{\"key\":\"value\"}");
}

#[test]
fn test_fmt_multiple_entries_map() {
    use std::fmt;

    struct TestMap {
        map: std::collections::HashMap<String, String>,
    }

    impl fmt::Display for TestMap {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            self.map.fmt(formatter)
        }
    }

    let mut multiple_entries_map = std::collections::HashMap::new();
    multiple_entries_map.insert("key1".to_string(), "value1".to_string());
    multiple_entries_map.insert("key2".to_string(), "value2".to_string());

    let map = TestMap { map: multiple_entries_map };

    let result = format!("{}", map);
    assert!(result.contains("\"key1\":\"value1\""));
    assert!(result.contains("\"key2\":\"value2\""));
}

