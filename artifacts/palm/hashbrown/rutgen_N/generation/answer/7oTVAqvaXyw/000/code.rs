// Answer 0

#[test]
fn test_fmt_with_empty_map() {
    use std::fmt;

    struct TestMap {
        inner: std::collections::HashMap<i32, String>,
    }

    impl fmt::Debug for TestMap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list()
                .entries(self.inner.iter().map(|(k, _)| k))
                .finish()
        }
    }

    let map = TestMap {
        inner: std::collections::HashMap::new(),
    };

    let result = format!("{:?}", map);
    assert_eq!(result, "[]");
}

#[test]
fn test_fmt_with_single_entry() {
    use std::fmt;

    struct TestMap {
        inner: std::collections::HashMap<i32, String>,
    }

    impl fmt::Debug for TestMap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list()
                .entries(self.inner.iter().map(|(k, _)| k))
                .finish()
        }
    }

    let mut map = TestMap {
        inner: std::collections::HashMap::new(),
    };
    map.inner.insert(1, "value".to_string());

    let result = format!("{:?}", map);
    assert_eq!(result, "[1]");
}

#[test]
fn test_fmt_with_multiple_entries() {
    use std::fmt;

    struct TestMap {
        inner: std::collections::HashMap<i32, String>,
    }

    impl fmt::Debug for TestMap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list()
                .entries(self.inner.iter().map(|(k, _)| k))
                .finish()
        }
    }

    let mut map = TestMap {
        inner: std::collections::HashMap::new(),
    };
    map.inner.insert(1, "value1".to_string());
    map.inner.insert(2, "value2".to_string());
    map.inner.insert(3, "value3".to_string());

    let result = format!("{:?}", map);
    assert_eq!(result, "[1, 2, 3]");
}

